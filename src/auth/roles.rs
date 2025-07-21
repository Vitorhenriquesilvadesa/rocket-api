use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Role {
    Admin,
    User,
    Guest,
}

pub struct Admin;
pub struct User;
pub struct Guest;
