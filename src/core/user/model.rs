use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(deserialize_with = "thing_to_user_id")]
    pub id: UserId,
    pub username: Username,
    pub password: PasswordHash,
}

impl User {
    pub fn new(id: UserId, username: Username, password: PasswordHash) -> Self {
        Self {
            id,
            username,
            password,
        }
    }
}

#[derive(Debug, Clone)]
pub struct UserId(String);

impl UserId {
    pub fn new(inner: String) -> Self {
        Self(inner)
    }

    pub fn to_string(self) -> String {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PasswordHash(String);

impl PasswordHash {
    pub fn from_hash(hash: String) -> Self {
        Self(hash)
    }

    pub fn raw(raw: String) -> Result<Self, argon2::password_hash::Error> {
        PasswordHash::hash(&raw)
    }

    fn hash(raw_password: &str) -> Result<Self, argon2::password_hash::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(raw_password.as_bytes(), &salt)?
            .to_string();

        Ok(PasswordHash(password_hash))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Username(String);

impl Username {
    pub fn new(username: String) -> Self {
        Self(username)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Password(String);

impl Password {
    pub fn new(inner: String) -> Self {
        Self(inner)
    }
}

impl Into<String> for Password {
    fn into(self) -> String {
        self.0
    }
}

impl Serialize for UserId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'da> Deserialize<'da> for UserId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'da>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(UserId::new(s))
    }
}

impl Into<String> for UserId {
    fn into(self) -> String {
        self.0
    }
}

impl Serialize for PasswordHash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'da> Deserialize<'da> for PasswordHash {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'da>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(PasswordHash::from_hash(s))
    }
}

impl Serialize for Username {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'da> Deserialize<'da> for Username {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'da>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Username::new(s))
    }
}

impl Serialize for Password {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'da> Deserialize<'da> for Password {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'da>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Password::new(s))
    }
}

fn thing_to_user_id<'de, D>(deserializer: D) -> Result<UserId, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let thing = Thing::deserialize(deserializer)?;
    Ok(UserId(thing.id.to_string()))
}

impl From<Thing> for UserId {
    fn from(thing: Thing) -> Self {
        UserId(format!("{}:{}", thing.tb, thing.id))
    }
}
