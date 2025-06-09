use crate::UpdateType;

pub struct Handler {}

impl crate::Backend for Handler {
    fn send_message(message: crate::Message) -> Result<(), crate::FailureReason> {
        return match message {
            crate::Message::Install(_) => Ok(()),
            crate::Message::Remove(_) => Ok(()),
            crate::Message::Query(_) => Ok(()),
            crate::Message::Upgrade(updateType) => match updateType {
                UpdateType::DistUpdate => Ok(()),
                UpdateType::System => Ok(()),
                UpdateType::Package(_) => Ok(()),
            },
            crate::Message::Sync() => Err(crate::FailureReason::PartialUpdateNotAllowed),
            crate::Message::Clean() => Ok(()),
        }
    }
    fn get_status() -> crate::BackendStatus {
        return crate::BackendStatus::Waiting;
    }
}