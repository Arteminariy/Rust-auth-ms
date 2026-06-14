use uuid::Uuid;

use crate::auth::{decode_token, encode_tokens, hash_password, verify_password, now_unix_secs};
use crate::dto::change_password::ChangePasswordDto;
use crate::dto::login::LoginData;
use crate::dto::refresh::RefreshData;
use crate::dto::token::TokenResponse;
use crate::error::ServiceError;
use crate::models::users::{CreateUserDto, CreateUserEntity};
use crate::repositories::users::UserRepository;
use crate::Result;

#[derive(Clone)]
pub struct AuthService {
    pub user_repository: UserRepository,
    pub jwt_secret: String,
    pub access_ttl_secs: u64,
    pub refresh_ttl_secs: u64,
}

impl AuthService {
    pub fn register(&self, new_user: CreateUserDto) -> Result<TokenResponse> {
        let pass_hash = hash_password(&new_user.password)?;
        let entity = CreateUserEntity {
            password_hash: pass_hash,
            name: new_user.name,
            role_id: new_user.role_id,
        };
        let user = self.user_repository.create(entity)?;
        self.tokens_for(&user)
    }

    pub fn login(&self, login: LoginData) -> Result<TokenResponse> {
        let user = self
            .user_repository
            .get_by_name(&login.name)
            .map_err(|_| ServiceError::NotFound("User not found".into()))?;
        verify_password(&user.password_hash, &login.password)
            .map_err(|_| ServiceError::Unauthorized("Invalid login data".into()))?;
        self.tokens_for(&user)
    }

    pub fn refresh(&self, data: RefreshData) -> Result<TokenResponse> {
        let claims = decode_token::<crate::auth::RefreshClaim>(&data.refresh_token, &self.jwt_secret)
            .map_err(|e| ServiceError::Unauthorized(e.to_string()))?;
        let user = self.user_repository.get_by_name(&claims.sub)?;
        self.tokens_for(&user)
    }

    pub fn change_password(&self, user_id: Uuid, data: ChangePasswordDto) -> Result<()> {
        let user = self.user_repository.get_one(user_id)?;
        verify_password(&user.password_hash, &data.old_password)
            .map_err(|_| ServiceError::Unauthorized("Invalid old password".into()))?;
        let new_hash = hash_password(&data.new_password)?;
        self.user_repository.change_password(user_id, new_hash)?;
        Ok(())
    }

    fn tokens_for(&self, user: &crate::models::users::UserModel) -> Result<TokenResponse> {
        encode_tokens(
            user,
            &self.jwt_secret,
            self.access_ttl_secs,
            self.refresh_ttl_secs,
            now_unix_secs(),
        )
        .map_err(|e| ServiceError::Internal(e.to_string()))
    }
}
