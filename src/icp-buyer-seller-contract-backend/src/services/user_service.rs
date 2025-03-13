use candid::Principal;
use crate::repositories::{ApiError, User, UserRepository, UserRepositoryImpl};

pub trait UserService {
    fn get_user_by_principal(&self, principal: &Principal) -> Result<(), ApiError>;
    fn create_user(&mut self, principal: Principal, user: User) -> Result<(), ApiError>;
    fn remove_user(&mut self, principal: &Principal) -> Result<(), ApiError>;
    fn update_user(&mut self, principal: Principal, user: User) -> Result<(), ApiError>;
    fn list_users(&self) -> Vec<(Principal, User)>;
}

pub struct UserServiceImpl<T: UserRepository> {
    user_repository: T,
}

impl Default for UserServiceImpl<UserRepositoryImpl> {
    fn default() -> Self {
        Self::new(UserRepositoryImpl::default())
    }
}

impl<T: UserRepository> UserServiceImpl<T> {
    pub fn new(user_repository: T) -> Self {
        Self { user_repository }
    }
}

impl<T: UserRepository> UserService for UserServiceImpl<T> {
    fn get_user_by_principal(&self, principal: &Principal) -> Result<(), ApiError> {
        self.user_repository.get_user_by_principal(principal)
        .ok_or_else(|| ApiError::not_found(format!("User with principal {} not found", principal).as_str()))?;
        Ok(())
    }

    fn create_user(&mut self, principal: Principal, user: User) -> Result<(), ApiError> {
        self.user_repository.create_user(principal, user)
    }

    fn remove_user(&mut self, principal: &Principal) -> Result<(), ApiError> {
        self.user_repository.remove_user(principal)
    }

    fn update_user(&mut self, principal: Principal, user: User) -> Result<(), ApiError> {
        self.user_repository.update_user(principal, user)
    }

    fn list_users(&self) -> Vec<(Principal, User)> {
        self.user_repository.list_users()
    }
}