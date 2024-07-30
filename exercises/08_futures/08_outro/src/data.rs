use crate::data::Status::{Done, InProgress, ToDo};
use crate::description::TicketDescription;
use crate::store::TicketId;
use crate::title::TicketTitle;

#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TryFrom<&str> for Status {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "todo" => Ok(ToDo),
            "inprogress" => Ok(InProgress),
            "done" => Ok(Done),
            _ => Err(())
        }
    }
}

impl Into<String> for &Status {
    fn into(self) -> String {
        format!("{self:?}")
    }
}
