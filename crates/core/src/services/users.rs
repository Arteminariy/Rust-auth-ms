use uuid::Uuid;

use crate::error::ServiceError;
use crate::models::users::{UpdateUserDto, UserDto};
use crate::pagination::{List, ResponsePagination};
use crate::repositories::users::UserRepository;
use crate::Result;

#[derive(Clone)]
pub struct UserService {
    pub repo: UserRepository,
}

impl UserService {
    pub fn get_one(&self, user_id: Uuid) -> Result<UserDto> {
        self.repo.get_one(user_id).map(Into::into).map_err(Into::into)
    }

    pub fn get_list(&self, pagination: ResponsePagination) -> Result<List<UserDto>> {
        let list = self.repo.get_list(pagination)?;
        let items = list.items.into_iter().map(UserDto::from).collect();
        Ok(List {
            pagination: list.pagination,
            items,
        })
    }

    pub fn update(&self, user_id: Uuid, dto: UpdateUserDto) -> Result<UserDto> {
        self.repo.update(user_id, dto).map(Into::into).map_err(Into::into)
    }

    pub fn delete(&self, user_id: Uuid) -> Result<()> {
        self.repo.delete(user_id).map_err(ServiceError::from)
    }
}
