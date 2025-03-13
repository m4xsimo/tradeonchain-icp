use alloy::primitives::Address;
use ic_cdk::init;
use repositories::{ApiError, Contract, Role, User, UserRepositoryImpl};
use candid::{Principal, CandidType, Deserialize};
use services::{AccessControlServiceImpl, AccessControlService, ContractService, ContractServiceImpl, UserService, UserServiceImpl, WalletService, WalletServiceImpl};
use repositories::{Uuid};

mod repositories;
mod services;
mod system_api;

#[init]
fn init() {
    let calling_principal = ic_cdk::caller();

    //add principal of canister creator as admin
    UserServiceImpl::default().create_user(calling_principal, User{role: Role::Admin}).unwrap();
}

#[ic_cdk::update]
fn add_permission(principal: Principal, role: Role) -> Result<(), ApiError> {
    let caller = ic_cdk::caller();
    AccessControlServiceImpl::default().assert_principal_is_admin(&caller)?;

    UserServiceImpl::default().create_user(principal, User{role})
}

#[ic_cdk::update]
fn remove_permission(principal: Principal) -> Result<(), ApiError> {
    let caller = ic_cdk::caller();
    AccessControlServiceImpl::default().assert_principal_is_admin(&caller)?;

    UserServiceImpl::default().remove_user(&principal)
}

#[ic_cdk::update]
fn update_permission(principal: Principal, role: Role) -> Result<(), ApiError> {
    let caller = ic_cdk::caller();
    AccessControlServiceImpl::default().assert_principal_is_admin(&caller)?;

    UserServiceImpl::default().update_user(principal, User{role})
}

#[ic_cdk::query]
fn get_users() -> Vec<(Principal, User)> {
    UserServiceImpl::default().list_users()
}

/// Create a new unsigned contract in storage
#[ic_cdk::update]
fn create_contract(contract_json: String, buyer: Principal, seller: Principal) -> Uuid {
    ContractServiceImpl::default().create_contract(contract_json, buyer, seller)
}

// Sign a contract
#[ic_cdk::update]
async fn sign_contract(contract_id: String) -> Result<(), ApiError> {
    let caller = ic_cdk::caller();

    ContractServiceImpl::default()
    .with_wallet(WalletServiceImpl::new(true))
    .sign_contract(contract_id, caller)
}

/// Query a contract by its ID
#[ic_cdk::query]
fn get_contract(contract_id: String) -> Option<Contract> {
    ContractServiceImpl::default().get_contract(contract_id)
}

/// query signature status of a contract
#[ic_cdk::query]
fn is_signed(contract_id: String) -> Result<bool, ApiError> {
    ContractServiceImpl::default().is_signed(contract_id)
}

#[ic_cdk::update]
async fn get_balance(address: String) -> Result<String, ApiError> {
    WalletServiceImpl::new(true).get_balance(address).await
}

/// Get the Ethereum address of the backend canister.
#[ic_cdk::update]
async fn get_address() -> Result<String, ApiError> {
    WalletServiceImpl::new(true).get_address().await
}

/// Request the balance of an ETH account.
#[ic_cdk::update]
async fn get_balance_usdc(address: Option<String>) -> Result<String, ApiError> {
    WalletServiceImpl::new(true).get_balance_usdc(address).await
}

#[ic_cdk::update]
async fn issue_payment(contract_id: String, seller_principal: Principal, address: String, amount: u64) -> Result<(), ApiError> {
    let caller = ic_cdk::caller();
    AccessControlServiceImpl::default().assert_principal_is_frontend(&caller)?;

    let address = address.parse::<Address>().map_err(|e| ApiError::internal(e.to_string().as_str()))?;

    ContractServiceImpl::default()
    .with_wallet(WalletServiceImpl::new(true))
    .issue_payment(contract_id, seller_principal, address, amount).await
}

// #[ic_cdk::update]
// async fn _sign_contract(contract_id: Uuid) -> Result<(), ApiError> {
//     let caller = ic_cdk::caller();
//     AccessControlServiceImpl::default().assert_principal_is_admin(&caller)?;

//     let contract_service = ContractServiceImpl::default().with_wallet(WalletServiceImpl::new(true));
//     let contract = contract_service.get_contract(contract_id)
//     .ok_or_else(|| ApiError::not_found("Contract not found"))?;

//     contract_service.sign_contract(contract_id, contract.signatories.buyer.0)?;
//     contract_service.sign_contract(contract_id, contract.signatories.seller.0)?;
//     Ok(())
// }

// #[ic_cdk::update]
// async fn transfer_usdc() -> Result<String, String> {
//     //TODO add access control
//     services::wallet_service::transfer_usdc().await
// }

#[ic_cdk::query]
async fn get_principal() -> Principal {
    ic_cdk::caller()
}

ic_cdk::export_candid!();