use candid::Principal;
use ic_stable_structures::BTreeMap;
use super::{Memory, MEMORY_MANAGER, USERS_MEMORY_ID};
use crate::repositories::{User};


pub type UserMemory = BTreeMap<Principal, User, Memory>;

pub fn init_users() -> UserMemory {
    UserMemory::init(get_user_memory())
}

fn get_user_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(USERS_MEMORY_ID))
}