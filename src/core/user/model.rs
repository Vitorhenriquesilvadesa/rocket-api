use argon2::{
    Argon2, PasswordVerifier,
    password_hash::{PasswordHash as PH, PasswordHasher, SaltString, rand_core::OsRng},
};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::auth::roles::Role;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(deserialize_with = "deserialize_thing_id")]
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: PasswordHash,
    pub roles: Vec<Role>,
}

impl User {
    pub fn new(
        id: String,
        username: String,
        email: String,
        password: PasswordHash,
        roles: Vec<Role>,
    ) -> Self {
        Self {
            id,
            username,
            email,
            password,
            roles,
        }
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

    pub fn verify(&self, raw_password: &str) -> bool {
        match PH::new(&self.0) {
            Ok(parsed_hash) => Argon2::default()
                .verify_password(raw_password.as_bytes(), &parsed_hash)
                .is_ok(),
            Err(_) => false,
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
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

fn deserialize_thing_id<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let thing = Thing::deserialize(deserializer)?;
    if let surrealdb::sql::Id::String(s) = thing.id {
        Ok(s)
    } else {
        Err(serde::de::Error::custom("Expected string ID"))
    }
}
