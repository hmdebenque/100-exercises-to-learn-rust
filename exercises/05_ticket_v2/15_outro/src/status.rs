// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `Status` enum.
//  The parsing should be case-insensitive.

use crate::Status::{Done, InProgress, ToDo};
use crate::status::StatusCreationError::Invalid;

#[derive(Debug, PartialEq, Clone)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

#[derive(thiserror::Error, Debug)]
pub enum StatusCreationError {
    #[error("Status is invalid.")]
    Invalid
}

impl Status {
    fn new(value: &str) -> Result<Status, StatusCreationError> {
        match value.to_lowercase().as_str() {
            "todo" => { Ok(ToDo) }
            "inprogress" => { Ok(InProgress) }
            "done" => { Ok(Done) }
            _ => Err(Invalid)
        }
    }
}

impl TryFrom<&str> for Status {
    type Error = StatusCreationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Status::new(value)
    }
}


impl TryFrom<String> for Status {
    type Error = StatusCreationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Status::new(value.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("ToDO").unwrap();
        assert_eq!(status, ToDo);

        let status = Status::try_from("inproGress").unwrap();
        assert_eq!(status, InProgress);

        let status = Status::try_from("Done").unwrap();
        assert_eq!(status, Done);
    }

    #[test]
    fn test_try_from_invalid() {
        let status = Status::try_from("Invalid");
        assert!(status.is_err());
    }
}
