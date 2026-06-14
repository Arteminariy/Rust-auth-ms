use uuid::Uuid;

use crate::error::ServiceError;
use crate::models::roles::{NewRole, Role, UpdateRole};
use crate::pagination::{List, ResponsePagination};
use crate::repositories::roles::RolesRepository;
use crate::Result;

#[derive(Clone)]
pub struct RolesService {
    pub repo: RolesRepository,
}

impl RolesService {
    pub fn create(&self, dto: NewRole) -> Result<Role> {
        self.repo.create(dto).map_err(ServiceError::from)
    }

    pub fn get_one(&self, role_id: Uuid) -> Result<Role> {
        self.repo.get_one(role_id).map_err(ServiceError::from)
    }

    pub fn get_list(&self, pagination: ResponsePagination) -> Result<List<Role>> {
        self.repo.get_list(pagination).map_err(ServiceError::from)
    }

    pub fn update(&self, role_id: Uuid, dto: UpdateRole) -> Result<Role> {
        self.repo.update(role_id, dto).map_err(ServiceError::from)
    }

    pub fn delete(&self, role_id: Uuid) -> Result<()> {
        self.repo.delete(role_id).map_err(ServiceError::from)
    }
}
