use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Status {
    Todo,
    InProgress,
    Done,
    Aborted,
}

impl From<i16> for Status {
    fn from(i: i16) -> Self {
        match i {
            0 => Self::Todo,
            1 => Self::InProgress,
            2 => Self::Done,
            -1 => Self::Aborted,
            _ => panic!("Invalid status value"),
        }
    }
}

impl Into<i16> for Status {
    fn into(self) -> i16 {
        match self {
            Self::Todo => 0,
            Self::InProgress => 1,
            Self::Done => 2,
            Self::Aborted => -1,
        }
    }
}
