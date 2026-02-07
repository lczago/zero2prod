mod health_check;
mod subscriptions;
mod subscriptions_confirm;
mod newsletter;

pub use health_check::health_check;
pub use subscriptions::{subscribe, FormData};
pub use subscriptions_confirm::confirm;
pub use newsletter::publish_newsletter;

fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}