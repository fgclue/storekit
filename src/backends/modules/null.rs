pub struct Handler;

impl crate::Backend for Handler {
    fn send_message(_message: crate::Message) -> Result<(), crate::FailureReason> {
        Err(crate::FailureReason::NullBackend)
    }
    fn get_status() -> crate::BackendStatus {
        crate::BackendStatus::Dead
    }
}