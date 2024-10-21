use candid::{Decode, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::{Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::borrow::Cow;
use std::cell::RefCell;
use candid::{Principal, CandidType, Deserialize};
//use std::{cell::RefCell, collections::BTreeMap};

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    // The memory manager is used for simulating multiple memories. Given a `MemoryId` it can
    // return a memory that can be used by stable structures.
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    // Initialize a `StableBTreeMap` with `MemoryId(0)`.
    static CONTRACTS: RefCell<StableBTreeMap<u64, Contract, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );


    // Initialize a `StableBTreeMap` with `MemoryId(0)`.
    static NEXT_CONTRACT_ID: RefCell<Cell<u64, Memory>> = RefCell::new(
        Cell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))), 1
        ).unwrap()
    );
}


/// A struct representing the signatories of a contract.
/// bool flag represents if they have signed the contract
#[derive(Clone, Debug, CandidType, Deserialize)]
struct ContractSignatories {
    buyer: (Principal, bool),
    seller: (Principal, bool),
}

/// A struct representing a contract.
/// It contains the signatories and the contract json.
/// The contract json is a json string representation of the contract computed offchain
#[derive(Clone, Debug, CandidType, Deserialize)]
struct Contract {
    signatories: ContractSignatories,
    contract_json: String,
    created_at: u64
}

impl Storable for Contract {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl Contract {
    fn new(contract_json: String, buyer: Principal, seller: Principal) -> Self {
        Self {
            signatories: ContractSignatories {
                buyer: (buyer, false),
                seller: (seller, false),
            },
            contract_json,
            created_at: ic_cdk::api::time()
        }
    }

    fn is_signed(&self) -> bool {
        self.signatories.buyer.1 && self.signatories.seller.1
    }
}

/// Create a new unsigned contract in storage
#[ic_cdk::update]
fn create_contract(contract_json: String, buyer: Principal, seller: Principal) -> u64 {
    //let caller = ic_cdk::caller();
    let contract_id = next_contract_id();

    let contract = Contract::new(contract_json, buyer, seller);
    CONTRACTS.with(|contracts| {
        contracts.borrow_mut().insert(
            contract_id.clone(),
            contract,
        );
    });

    contract_id
}

/// Sign a contract
#[ic_cdk::update]
async fn sign_contract(contract_id: u64) {
    let caller = ic_cdk::caller();

    CONTRACTS.with(|contracts| {
        let mut c = contracts.borrow_mut();
        if let Some(mut contract) = c.get(&contract_id) {
            if caller == contract.signatories.buyer.0 {
                contract.signatories.buyer.1 = true;
            } else if caller == contract.signatories.seller.0 {
                contract.signatories.seller.1 = true;
            } else {
                ic_cdk::trap("Error: buyer or seller not valid.")
            }
            c.insert(contract_id, contract);
        }
    });

}

/// Query a contract by its ID
#[ic_cdk::query]
fn get_contract(contract_id: u64) -> Option<Contract> {
    CONTRACTS.with(|contracts| contracts.borrow().get(&contract_id).clone())
}

/// query signature status of a contract
#[ic_cdk::query]
fn is_signed(contract_id: u64) -> bool {
    if let Some(contract) =  get_contract(contract_id) {
        contract.is_signed()
    } else {
        ic_cdk::trap("Error: contract not found")
    }
}
/// Get the next available Id for a creating a new contract and increment the ID counter
/// contract IDs are generated sequentially
/// Starting with 1 - cannot be zero
fn next_contract_id() -> u64 {
    NEXT_CONTRACT_ID.with(|counter| {
        let mut c = counter.borrow_mut();
        let new_count =c.get()+1;
        c.set(new_count).unwrap();
        new_count
    })
}

//ic_cdk::export_candid!();