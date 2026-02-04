use crate::domain::SubscriberEmail;
use crate::domain::subscriber_name::SubscriberName;

#[derive(serde::Deserialize, Debug)]
pub struct NewSubscriber {
    pub email: SubscriberEmail,
    pub name: SubscriberName,
}
