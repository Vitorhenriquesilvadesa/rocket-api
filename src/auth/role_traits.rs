use super::roles::{Admin, Guest, Role, User};

pub trait RequiredRole {
    const ROLE: Role;
}

impl RequiredRole for Admin {
    const ROLE: Role = Role::Admin;
}

impl RequiredRole for User {
    const ROLE: Role = Role::User;
}

impl RequiredRole for Guest {
    const ROLE: Role = Role::Guest;
}
