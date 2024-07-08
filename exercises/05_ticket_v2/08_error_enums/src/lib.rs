use crate::TicketNewError::{TitleEmpty, DescriptionEmpty, DescriptionTooLong, TitleTooLong};

// TODO: Use two variants, one for a title error and one for a description error.
//   Each variant should contain a string with the explanation of what went wrong exactly.
//   You'll have to update the implementation of `Ticket::new` as well.
#[derive(Debug)]
enum TicketNewError {
    TitleEmpty,
    TitleTooLong,
    DescriptionEmpty,
    DescriptionTooLong,
}

impl TicketNewError {
    fn desc(&self) -> &str {
        match self {
            TitleEmpty => { "Title cannot be empty" }
            TitleTooLong => { "Title cannot be longer than 50 bytes" }
            DescriptionEmpty => { "Description cannot be empty" }
            DescriptionTooLong => { "Description cannot be longer than 500 bytes" }
        }
    }
}

// TODO: `easy_ticket` should panic when the title is invalid, using the error message
//   stored inside the relevant variant of the `TicketNewError` enum.
//   When the description is invalid, instead, it should use a default description:
//   "Description not provided".
fn easy_ticket(title: String, description: String, status: Status) -> Ticket {
    Ticket::new(title.clone(), description, status.clone())
        .unwrap_or_else(|e| {
            match e {
                TitleEmpty => {
                    panic!("{}", e.desc())
                }
                TitleTooLong => {
                    panic!("{}", e.desc())
                }
                DescriptionEmpty => {
                    Ticket::new(title, "Description not provided".to_string(), status).unwrap()
                }
                DescriptionTooLong => {
                    Ticket::new(title, "Description not provided".to_string(), status).unwrap()
                }
            }
        })
}

#[derive(Debug, PartialEq)]
struct Ticket {
    title: String,
    description: String,
    status: Status,
}

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress { assigned_to: String },
    Done,
}

impl Ticket {
    pub fn new(
        title: String,
        description: String,
        status: Status,
    ) -> Result<Ticket, TicketNewError> {
        if title.is_empty() {
            return Err(TitleEmpty);
        }
        if title.len() > 50 {
            return Err(TitleTooLong);
        }
        if description.is_empty() {
            return Err(DescriptionEmpty);
        }
        if description.len() > 500 {
            return Err(DescriptionTooLong);
        }

        Ok(Ticket {
            title,
            description,
            status,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        easy_ticket("".into(), valid_description(), Status::ToDo);
    }

    #[test]
    fn template_description_is_used_if_empty() {
        let ticket = easy_ticket(valid_title(), "".into(), Status::ToDo);
        assert_eq!(ticket.description, "Description not provided");
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 bytes")]
    fn title_cannot_be_longer_than_fifty_chars() {
        easy_ticket(overly_long_title(), valid_description(), Status::ToDo);
    }

    #[test]
    fn template_description_is_used_if_too_long() {
        let ticket = easy_ticket(valid_title(), overly_long_description(), Status::ToDo);
        assert_eq!(ticket.description, "Description not provided");
    }
}
