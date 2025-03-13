use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_stable_structures::{storable::Bound, Storable};
use std::borrow::Cow;

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum Role{
    Admin,
    FrontendServer
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct User {
    pub role: Role,
    //this could be used to gate certain endpoints only to users authorized by the frontend to ensure they are registered.
}

impl User {
    pub fn new(role: Role) -> Self {
        Self { role }
    }

    pub fn is_admin(&self) -> bool {
        matches!(self.role, Role::Admin)
    }

    pub fn is_frontend_server(&self) -> bool {
        matches!(self.role, Role::FrontendServer)
    }
}

impl Storable for User {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}