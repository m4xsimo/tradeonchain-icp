use ic_stable_structures::StableBTreeMap;
use super::{Memory, CONTRACTS_MEMORY_ID, MEMORY_MANAGER};
use crate::repositories::{Contract, Uuid};


pub type ContractMemory = StableBTreeMap<Uuid, Contract, Memory>;

pub fn init_contracts() -> ContractMemory {
    StableBTreeMap::init(get_contracts_memory())
}

fn get_contracts_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(CONTRACTS_MEMORY_ID))
}