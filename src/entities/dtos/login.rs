use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Claims {
    pub nbf: usize,
    pub iat: usize,
    pub exp: usize,
    pub tipo: TokenType,
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum TokenType {
    Refresh,
    Normal,
}
impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::Refresh => f.write_str("refresh"),
            TokenType::Normal => f.write_str("normal"),
        }
    }
}
impl Into<String> for TokenType {
    fn into(self) -> String {
        match self {
            TokenType::Refresh => String::from("refresh"),
            TokenType::Normal => String::from("normal"),
        }
    }
}

impl From<String> for TokenType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "refresh" => TokenType::Refresh,
            "normal" => TokenType::Normal,
            _ => TokenType::Normal,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct LoginForm {
    pub nombre: String,
    pub apellido: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LoginResult {
    pub id: String,
    pub token: String,
    pub refresh: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RefreshResult {
    pub id: String,
    pub token: String,
}
