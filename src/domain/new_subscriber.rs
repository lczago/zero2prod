use crate::domain::SubscriberEmail;
use crate::domain::subscriber_name::SubscriberName;
use crate::routes::FormData;

#[derive(serde::Deserialize, Debug)]
pub struct NewSubscriber {
    pub email: SubscriberEmail,
    pub name: SubscriberName,
}


impl TryFrom<FormData> for NewSubscriber {
    type Error = String;
    
    fn try_from(form: FormData) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(form.name)?;
        let email = SubscriberEmail::parse(form.email)?;
        Ok(Self { name, email })
    }
}