//! Pass a bearer for authentication

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::ops::Deref;

/// The bearer token
#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Hash, Debug)]
pub struct BearerToken(Box<[u8]>);

impl BearerToken {}

impl<B: AsRef<[u8]>> From<B> for BearerToken {
    fn from(value: B) -> Self {
        Self(Vec::from(value.as_ref()).into_boxed_slice())
    }
}

impl Deref for BearerToken {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl Display for BearerToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bearer {}", String::from_utf8_lossy(&self.0))
    }
}
