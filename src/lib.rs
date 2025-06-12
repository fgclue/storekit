mod backends;

/// Status of the backend
/// 
/// - `Waiting`: Backend just started and still hasn't done anything
/// - `Finished`: Done doing a task
/// - `Failed(String)`: Failed doing a task
/// - `Dead`: Backend has crashed
/// - `Working`: Doing something non-critical such as searching packages or syncing.
/// - `WorkingImportant`: Doing something critical such as installing, removing or cleaning.
/// - `Updating(UpdateType)`: Updating the system, distribution or a package.
pub enum BackendStatus {
    Waiting,
    Finished,
    Failed(String),
    Dead,
    Working,
    WorkingImportant,
    Updating(UpdateType),
}

/// Types of updates
/// 
/// `DistUpdate` is not supported on all backends, such as pacman.
/// 
/// If not supported, the backend will use System instead.
/// 
/// Ideally you should avoid `DistUpdate`.
pub enum UpdateType {
    DistUpdate, // not supported on all backends, some will ignore this
    System,
    Package(String)
}

/// Reason for failure
pub enum FailureReason {
    NonexistentCommand,
    BackendDied,
    NonexistentPackage,
    FailedConnection,
    PartialUpdateNotAllowed,
    NullBackend,
    Other(String),
}

/// Message sent to a backend
pub enum Message {
    Install(String),
    Remove(String),
    Query(String),
    Upgrade(UpdateType),
    Sync(),
    Clean(), // not supported on all backends, some will ignore this
}

/// A backend, which handles messages and sends its status back.
pub trait Backend {
    fn send_message(message: Message) -> Result<(), FailureReason>;
    fn get_status() -> BackendStatus;
}

pub fn send_message(_message: Message) {
    
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn temp() {
        assert_eq!(4, 4);
    }
}
