pub mod drivers;
pub mod structproto;

#[derive(thiserror::Error, Debug)]
pub enum DriverError {
    #[error("transient, retryable error")]
    TransientError(Box<dyn std::error::Error>),
}
/// A base driver object, which is responsible for shutting state to and from
/// interfaces when polled.
pub trait Driver {
    /// Poll the driver, updating any internal state
    fn poll(&mut self) -> Result<(), DriverError>;
}
