use crate::repositories::{ApiError, UserRepository, UserRepositoryImpl};
use candid::Principal;

pub trait AccessControlService {
    fn assert_principal_not_anonymous(&self, calling_principal: &Principal)
        -> Result<(), ApiError>;

    fn assert_principal_is_admin(&self, calling_principal: &Principal) -> Result<(), ApiError>;

    fn assert_principal_is_frontend(&self, calling_principal: &Principal) -> Result<(), ApiError>;
}

pub struct AccessControlServiceImpl<T: UserRepository> {
    user_repository: T,
}

impl Default for AccessControlServiceImpl<UserRepositoryImpl> {
    fn default() -> Self {
        Self::new(UserRepositoryImpl::default())
    }
}

impl<T: UserRepository> AccessControlService for AccessControlServiceImpl<T> {
    fn assert_principal_not_anonymous(
        &self,
        calling_principal: &Principal,
    ) -> Result<(), ApiError> {
        if calling_principal == &Principal::anonymous() {
            return Err(ApiError::unauthenticated());
        }

        Ok(())
    }

    fn assert_principal_is_admin(&self, calling_principal: &Principal) -> Result<(), ApiError> {
        let user = self
            .user_repository
            .get_user_by_principal(calling_principal)
            .ok_or_else(|| {
                ApiError::not_found(&format!(
                    "Principal {} must be admin to call this endpoint",
                    calling_principal.to_text()
                ))
            })?;

        if !user.is_admin() {
            return Err(ApiError::permission_denied(&format!(
                "Principal {} must be an admin to call this endpoint",
                calling_principal.to_text()
            )));
        }

        Ok(())
    }

    fn assert_principal_is_frontend(&self, calling_principal: &Principal) -> Result<(), ApiError> {
        let user = self
            .user_repository
            .get_user_by_principal(calling_principal)
            .ok_or_else(|| {
                ApiError::not_found(&format!(
                    "{} not authorized. Only frontend server principals can call this endpoint.",
                    calling_principal.to_text()
                ))
            })?;

        if !user.is_frontend_server() {
            return Err(ApiError::permission_denied(&format!(
                "{} not authorized. Only frontend server principals can call this endpoint.",
                calling_principal.to_text()
            )));
        }

        Ok(())
    }
}

impl<T: UserRepository> AccessControlServiceImpl<T> {
    fn new(user_repository: T) -> Self {
        Self {
            user_repository,
        }
    }
}
