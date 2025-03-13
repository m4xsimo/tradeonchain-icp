use std::borrow::Cow;
use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_stable_structures::{
    storable::{Blob, Bound},
    Storable,
};

/// A struct representing the signatories of a contract.
/// bool flag represents if they have signed the contract
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct ContractSignatories {
    pub buyer: (Principal, bool),
    pub seller: (Principal, bool),
}

/// A struct representing a contract.
/// It contains the signatories and the contract json.
/// The contract json is a json string representation of the contract computed offchain
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Contract {
    pub signatories: ContractSignatories,
    pub contract_json: String,
    pub created_at: u64,
    //TODO: add for validation?
    //pub amount : u64,
    pub issued_payment : bool,
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
    pub fn new(contract_json: String, buyer: Principal, seller: Principal) -> Self {
        Self {
            signatories: ContractSignatories {
                buyer: (buyer, false),
                seller: (seller, false),
            },
            contract_json,
            created_at: ic_cdk::api::time(),
            issued_payment: false,
        }
    }

    pub fn is_signed(&self) -> bool {
        self.signatories.buyer.1 && self.signatories.seller.1
    }

    pub fn issued_payment(&self) -> bool {
        self.issued_payment
    }
}
