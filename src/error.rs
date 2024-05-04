use thiserror::Error as ThisError;

use crate::event::Event;

/// Custom error type.
#[derive(Debug, ThisError)]
pub enum Error {
    /// Error that may occur during I/O operations.
    #[error("IO error: `{0}`")]
    IoError(#[from] std::io::Error),
    /// Error that may occur while receiving messages from the channel.
    #[error("channel receive error: `{0}`")]
    ChannelReceiveError(#[from] std::sync::mpsc::RecvError),
    /// Error that may occur while sending messages to the channel.
    #[error("channel send error: `{0}`")]
    ChannelSendError(#[from] std::sync::mpsc::SendError<Event>),
    /// Error that is related to CVE cache.
    #[error("CVE cache error: `{0:?}`")]
    CacheError(nvd_cve::cache::CacheError),
    /// Error that may occur while parsing colors.
    #[error("Failed to parse color")]
    ParseColorError,
    /// Error that can occur while parsing the arguments.
    #[error("Invalid range given. Please use the format <start:end>")]
    RangeArgsError,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Error as IoError, ErrorKind};

    #[test]
    fn test_error() {
        let message = "your computer is on fire!";
        let error = Error::from(IoError::new(ErrorKind::Other, message));
        assert_eq!(format!("IO error: `{message}`"), error.to_string());
        assert_eq!(
            format!("\"IO error: `{message}`\""),
            format!("{:?}", error.to_string())
        );
    }
}
