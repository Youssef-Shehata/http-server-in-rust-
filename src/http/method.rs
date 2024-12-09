use std::{fmt::Display, str::FromStr};

#[derive(Eq,Hash,Debug, PartialEq, Clone, Copy)]
pub enum HttpMethod {
    Get,
    Post,
    Delete,
}

impl Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
           HttpMethod::Get => write!(f, "GET"),
           HttpMethod::Post => write!(f, "POST"),
           HttpMethod::Delete => write!(f, "DELETE"),
        }
    }
}
impl FromStr for HttpMethod {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "GET" => Ok(HttpMethod::Get),
            "POST" => Ok(HttpMethod::Post),
            "DELETE" => Ok(HttpMethod::Delete),
            _ => Err(crate::Error::InvalidMethod(s.to_string())),
        }
    }
}
