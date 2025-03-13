use std::cell::RefCell;

use candid::Principal;
use crate::repositories::init_users;

use super::{ApiError, User, UserMemory};


pub trait UserRepository {
    fn get_user_by_principal(&self, principal: &Principal) -> Option<User>;
    fn create_user(&mut self, principal: Principal, user: User) -> Result<(), ApiError>;
    fn remove_user(&mut self, principal: &Principal) -> Result<(), ApiError>;
    fn update_user(&mut self, principal: Principal, user: User) -> Result<(), ApiError>;
    fn list_users(&self) -> Vec<(Principal, User)>;
}

pub struct UserRepositoryImpl {}

impl Default for UserRepositoryImpl {
    fn default() -> Self {
        Self {}
    }
}

impl UserRepositoryImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl UserRepository for UserRepositoryImpl {
    fn get_user_by_principal(&self, principal: &Principal) -> Option<User> {
        STATE.with(|state| {
            let state = state.borrow();
            state.get(principal)
        })
    }

    fn create_user(&mut self, principal: Principal, user: User) -> Result<(), ApiError> {
        if self.get_user_by_principal(&principal).is_some() {
            return Err(ApiError::conflict("User already exists"));
        }

        STATE.with_borrow_mut(|state| {
            state.insert(principal, user);
        });

        Ok(())
    }

    fn remove_user(&mut self, principal: &Principal) -> Result<(), ApiError> {
        if self.get_user_by_principal(principal).is_none() {
            return Err(ApiError::not_found("User not found"));
        }

        STATE.with_borrow_mut(|state| {
            state.remove(principal);
        });
        Ok(())
    }

    fn update_user(&mut self, principal: Principal, user: User) -> Result<(), ApiError> {
        STATE.with_borrow_mut(|state| {
            state.insert(principal, user);
        });
        Ok(())
    }

    fn list_users(&self) -> Vec<(Principal, User)> {
        STATE.with_borrow(|s| s.iter().collect())
    }
}


thread_local! {
    static STATE: RefCell<UserMemory> = RefCell::new(init_users());
}