mod backends;
mod message;

pub use message::BackendStatus as BackendStatus;
pub use message::FailureReason as FailureReason;
pub use message::Message as Message;
pub use message::UpdateType as UpdateType;

/// A backend, which handles messages and sends its status back.
pub trait Backend {
    fn send_message(message: Message) -> Result<(), FailureReason>;
    fn get_status() -> BackendStatus;
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn temp() {
        assert_eq!(4, 4);
    }
}
