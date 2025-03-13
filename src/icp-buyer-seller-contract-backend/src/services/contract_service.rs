use alloy::primitives::Address;
use candid::Principal;
use crate::repositories::{ApiError, Contract, ContractRepository, ContractRepositoryImpl, Signer, Uuid};

use super::{WalletService, WalletServiceImpl};

pub trait ContractService {
    fn create_contract(&self, contract_json: String, buyer: Principal, seller: Principal) -> Uuid;
    fn sign_contract(&self, contract_id: String, caller: Principal) -> Result<(), ApiError>;
    fn get_contract(&self, contract_id: String) -> Option<Contract>;
    fn is_signed(&self, contract_id: String) -> Result<bool, ApiError>;
    async fn issue_payment(&self, contract_id: String, caller: Principal, address: Address, amount: u64) -> Result<(), ApiError>;
}

pub struct ContractServiceImpl<T: ContractRepository, U: WalletService> {
    contract_repository: T,
    wallet_service: U,
}

impl<T: ContractRepository, U: WalletService> ContractServiceImpl<T, U> {
    pub fn new(contract_repository: T, wallet_service: U) -> Self {
        Self { contract_repository, wallet_service}
    }

    pub fn with_wallet(mut self, wallet: U) -> Self {
        self.wallet_service = wallet;
        self
    }
}

impl Default for ContractServiceImpl<ContractRepositoryImpl, WalletServiceImpl> {
    fn default() -> Self {
        Self::new(ContractRepositoryImpl::default(), WalletServiceImpl::default())
    }
}

impl<T: ContractRepository, U: WalletService> ContractService for ContractServiceImpl<T, U> {
    /// Create a new unsigned contract in storage
    fn create_contract(&self, contract_json: String, buyer: Principal, seller: Principal) -> Uuid {
        self.contract_repository.create_contract(contract_json, buyer, seller)
    }

    /// Sign a contract
    fn sign_contract(&self, contract_id: String, caller: Principal) -> Result<(), ApiError> {
        let contract_id = Uuid::try_from(contract_id.as_str())?;
        if let Some(contract) = self.contract_repository.get_contract(contract_id) {
            if caller == contract.signatories.buyer.0 {
                self.contract_repository.update_contract_signature(contract_id, Signer::Buyer);
                Ok(())
            } else if caller == contract.signatories.seller.0 {
                self.contract_repository.update_contract_signature(contract_id, Signer::Seller);
                Ok(())
            } else {
                return Err(ApiError::permission_denied("Caller not authorized to sign this contract"));
            }
        } else {
            return Err(ApiError::not_found("Contract not found"));
        }
    }

    /// Query a contract by its ID
    fn get_contract(&self, contract_id: String) -> Option<Contract> {
        if let Ok(contract_id) = Uuid::try_from(contract_id.as_str()) {
            self.contract_repository.get_contract(contract_id)
        } else {
            None
        }
    }

    /// query signature status of a contract
    fn is_signed(&self, contract_id: String) -> Result<bool, ApiError> {
        let contract_id = Uuid::try_from(contract_id.as_str())?;
        if let Some(contract) =  self.contract_repository.get_contract(contract_id) {
            Ok(contract.is_signed())
        } else {
            Err(ApiError::not_found("Contract not found"))
        }
    }

    async fn issue_payment(&self, contract_id: String, caller: Principal, address: Address, amount: u64) -> Result<(), ApiError>{
        let contract_id = Uuid::try_from(contract_id.as_str())?;
        if let Some(contract) = self.contract_repository.get_contract(contract_id) {
            if caller != contract.signatories.seller.0 {
                return Err(ApiError::permission_denied("Caller not authorized"));
            }

            if !contract.is_signed() {
                return Err(ApiError::internal("Contract not signed"));
            }

            if contract.issued_payment(){
                return Err(ApiError::internal("Payment already issued"));
            }

            //eagerly set payment status to true to prevent double spending
            self.contract_repository.update_payment_status(contract_id, true);
            //TODO: validate amount against contract or use value stored in contract
            match self.wallet_service.transfer_usdc(amount, address).await{
                Ok(_) => Ok(()),
                Err(e) => {
                    //rollback payment status if transfer failed
                    self.contract_repository.update_payment_status(contract_id, false);
                    Err(ApiError::internal(format!("Transfer failed: {}", e).as_str()))
                }
            }
        } else {
            Err(ApiError::not_found("Contract not found"))
        }
    }
}