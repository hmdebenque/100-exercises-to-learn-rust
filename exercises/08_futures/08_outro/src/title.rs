use std::convert::TryFrom;

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct TicketTitle(String);

impl From<TicketTitle> for String {
    fn from(value: TicketTitle) -> Self {
        value.0
    }
}

impl From<&TicketTitle> for String {
    fn from(value: &TicketTitle) -> Self {
        value.0.clone()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TicketTitleError {
    #[error("The title cannot be empty")]
    Empty,
    #[error("The title cannot be longer than 50 bytes")]
    TooLong,
}

impl TryFrom<String> for TicketTitle {
    type Error = TicketTitleError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        validate(&value)?;
        Ok(Self(value))
    }
}

impl TryFrom<&str> for TicketTitle {
    type Error = TicketTitleError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        validate(value)?;
        Ok(Self(value.to_string()))
    }
}

fn validate(title: &str) -> Result<(), TicketTitleError> {
    if title.is_empty() {
        Err(TicketTitleError::Empty)
    } else if title.len() > 50 {
        Err(TicketTitleError::TooLong)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::*;

    #[test]
    fn test_try_from_string() {
        let input = valid_title();
        let title = TicketTitle::try_from(input.clone()).unwrap();
        assert_eq!(title.0, input);
    }

    #[test]
    fn test_try_from_empty_string() {
        let err = TicketTitle::try_from("".to_string()).unwrap_err();
        assert_eq!(err.to_string(), "The title cannot be empty");
    }

    #[test]
    fn test_try_from_long_string() {
        let err = TicketTitle::try_from(overly_long_title()).unwrap_err();
        assert_eq!(err.to_string(), "The title cannot be longer than 50 bytes");
    }

    #[test]
    fn test_try_from_str() {
        let title = TicketTitle::try_from("A title").unwrap();
        assert_eq!(title.0, "A title");
    }

    fn valid_title() -> String {
        "A title".into()
    }

    fn overly_long_title() -> String {
        "A title that's definitely longer than what should be allowed in a development ticket".into()
    }
}
