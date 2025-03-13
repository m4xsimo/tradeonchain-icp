use std::cell::RefCell;

use candid::Principal;
use crate::repositories::{Contract, Uuid};
use super::{init_contracts, ContractMemory};


pub trait ContractRepository {
    fn create_contract(&self, contract_json: String, buyer: Principal, seller: Principal) -> Uuid;
    fn get_contract(&self, contract_id: Uuid) -> Option<Contract>;
    fn update_contract_signature(&self, contract_id: Uuid, signer: Signer);
    fn update_payment_status(&self, contract_id: Uuid, status: bool);
}

#[derive(Debug)]
pub enum Signer {
    Buyer,
    Seller,
}

pub struct ContractRepositoryImpl;

impl ContractRepository for ContractRepositoryImpl{
    /// Create a new unsigned contract in storage
    fn create_contract(&self, contract_json: String, buyer: Principal, seller: Principal) -> Uuid {
        let contract_id = Uuid::new();

        let contract = Contract::new(contract_json, buyer, seller);
        STATE.with(|contracts| {
            contracts.borrow_mut().insert(
                contract_id.clone(),
                contract,
            );
        });

        contract_id
    }

    /// Query a contract by its ID
    fn get_contract(&self, contract_id: Uuid) -> Option<Contract> {
        STATE.with(|contracts| contracts.borrow().get(&contract_id).clone())
    }

    fn update_contract_signature(&self, contract_id: Uuid, signer: Signer) {
        STATE.with(|contracts| {
            let mut contracts = contracts.borrow_mut();
            if let Some(mut contract) = contracts.get(&contract_id) {
                match signer {
                    Signer::Buyer => contract.signatories.buyer.1 = true,
                    Signer::Seller => contract.signatories.seller.1 = true,
                }
                contracts.insert(contract_id, contract);
            }
        });
    }

    fn update_payment_status(&self, contract_id: Uuid, status: bool) {
        STATE.with(|contracts| {
            let mut contracts = contracts.borrow_mut();
            if let Some(mut contract) = contracts.get(&contract_id) {
                contract.issued_payment = status;
                contracts.insert(contract_id, contract);
            }
        });
    }
}

impl ContractRepositoryImpl {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ContractRepositoryImpl {
    fn default() -> Self {
        Self::new()
    }
}

thread_local! {
    static STATE: RefCell<ContractMemory> = RefCell::new(init_contracts());
}