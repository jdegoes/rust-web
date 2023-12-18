use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl From<i16> for Priority {
    fn from(i: i16) -> Self {
        match i {
            0 => Self::Low,
            1 => Self::Medium,
            2 => Self::High,
            _ => panic!("Invalid priority value"),
        }
    }
}

impl From<String> for Priority {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Low" => Self::Low,
            "Medium" => Self::Medium,
            "High" => Self::High,
            t => panic!("Invalid priority value: {}", t),
        }
    }
}

impl Into<i16> for Priority {
    fn into(self) -> i16 {
        match self {
            Self::Low => 0,
            Self::Medium => 1,
            Self::High => 2,
        }
    }
}
