use crate::telemetry::spawn_blocking_with_tracing;
use actix_web::http::header::HeaderValue;
use actix_web::http::{StatusCode, header};
use actix_web::{HttpResponse, ResponseError};
use anyhow::Context;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use secrecy::{ExposeSecret, SecretString};
use sqlx::PgPool;
use uuid::Uuid;

pub struct Credentials {
    pub username: String,
    pub password: SecretString,
}

#[derive(thiserror::Error)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        crate::routes::error_chain_fmt(self, f)
    }
}

impl ResponseError for AuthError {
    fn status_code(&self) -> StatusCode {
        match self {
            AuthError::InvalidCredentials(_) => StatusCode::UNAUTHORIZED,
            AuthError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn error_response(&self) -> HttpResponse {
        match self {
            AuthError::InvalidCredentials(_) => {
                let mut response = HttpResponse::new(StatusCode::UNAUTHORIZED);
                let header_value = HeaderValue::from_str(r#"Basic realm="publish""#).unwrap();
                response
                    .headers_mut()
                    .insert(header::WWW_AUTHENTICATE, header_value);
                response
            }
            AuthError::UnexpectedError(_) => HttpResponse::InternalServerError().finish(),
        }
    }
}

#[tracing::instrument(name = "Validate credentials", skip(credentials, pool))]
pub async fn validate_credentials(
    credentials: Credentials,
    pool: &PgPool,
) -> Result<Uuid, AuthError> {
    let stored = get_stored_credentials(&credentials.username, pool)
        .await
        .map_err(AuthError::UnexpectedError)?;

    let mut user_id = None;
    if let Some((stored_user_id, expected_password_hash)) = stored {
        user_id = Some(stored_user_id);
        spawn_blocking_with_tracing(move || {
            verify_password_hash(expected_password_hash, credentials.password)
        })
        .await
        .context("Failed to spawn blocking task")
        .map_err(AuthError::UnexpectedError)??;
    }

    user_id.ok_or_else(|| AuthError::InvalidCredentials(anyhow::anyhow!("Invalid username.")))
}

#[tracing::instrument(
    name = "Verify password hash",
    skip(expected_password_hash, password_candidate)
)]
fn verify_password_hash(
    expected_password_hash: SecretString,
    password_candidate: SecretString,
) -> Result<(), AuthError> {
    let expected_password_hash = PasswordHash::new(expected_password_hash.expose_secret())
        .context("Failed to parse password hash")
        .map_err(AuthError::UnexpectedError)?;

    Argon2::default()
        .verify_password(
            password_candidate.expose_secret().as_bytes(),
            &expected_password_hash,
        )
        .context("Invalid password")
        .map_err(AuthError::InvalidCredentials)
}

#[tracing::instrument(name = "Get stored credentials", skip(username, pool))]
async fn get_stored_credentials(
    username: &str,
    pool: &PgPool,
) -> Result<Option<(Uuid, SecretString)>, anyhow::Error> {
    let row: Option<_> = sqlx::query!(
        r#"SELECT  user_id, password_hash FROM users WHERE username = $1"#,
        username
    )
    .fetch_optional(pool)
    .await
    .context("Failed to perform a query to validate auth credentials.")?
    .map(|row| (row.user_id, SecretString::from(row.password_hash)));

    Ok(row)
}
