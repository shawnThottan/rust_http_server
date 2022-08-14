use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Copy)]
pub enum Status {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl Status {
    pub fn get_reason(&self) -> &'static str {
        match self {
            Self::Ok => "OK",
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found",
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", *self as u16)
    }
}