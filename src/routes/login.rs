use crate::authentication::{AuthError, Credentials, validate_credentials};
use actix_web::{HttpResponse, web};
use secrecy::SecretString;
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct FormData {
    username: String,
    password: SecretString,
}

#[tracing::instrument(skip(form, pool), fields(username=tracing::field::Empty, user_id=tracing::field::Empty))]
pub async fn login(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AuthError> {
    let credentials = Credentials {
        username: form.0.username,
        password: form.0.password,
    };

    tracing::Span::current().record("username", tracing::field::display(&credentials.username));

    let user_id = validate_credentials(credentials, &pool).await?;

    tracing::Span::current().record("user_id", tracing::field::display(&user_id));
    Ok(HttpResponse::Ok().finish())
}
