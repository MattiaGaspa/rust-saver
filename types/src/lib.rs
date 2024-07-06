use serde;

#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, Debug)]
pub struct Password {
    pub name: String,
    pub password: String,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, Debug)]
pub struct PasswordList {
    pub list: Vec<Password>,
}