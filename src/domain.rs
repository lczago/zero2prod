use unicode_segmentation::UnicodeSegmentation;

#[derive(serde::Deserialize, Debug)]
pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName,
}

#[derive(serde::Deserialize, Debug)]
pub struct SubscriberName(String);

impl SubscriberName {
    pub fn inner_ref(&self) -> &str {
        &self.0
    }

    pub fn parse(s: String) -> SubscriberName {
        let is_empty_or_whitespace = s.trim().is_empty();

        let is_too_long = s.graphemes(true).count() > 256;

        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];

        let contains_forbidden_characters = forbidden_characters.iter().any(|c| s.contains(*c));

        if !is_empty_or_whitespace && !is_too_long && !contains_forbidden_characters {
            panic!("Invalid subscriber name: {}", s);
        } else {
            Self(s)
        }
    }
}

#[tracing::instrument(name = "Saving new subscriber details in the database", skip(pool))]
pub async fn insert_subscriber(
    pool: &sqlx::PgPool,
    new_subscriber: &NewSubscriber,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)",
        Uuid::new_v4(),
        new_subscriber.email,
        new_subscriber.name.inner_ref(),
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
