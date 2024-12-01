use std::{collections::HashMap, future::Future, sync::Arc};

use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{
    business::user::{
        dtos::{UserFindRequest, UserFindResponse},
        model::user::UserError,
        User, UserRepositoryTrait,
    },
    outbound::repository_trait::{FindOptionTrait, RepositoryTrait},
};

#[derive(Debug, Clone)]
pub struct InMemoryUserRepository {
    data: Arc<RwLock<HashMap<Uuid, User>>>,
}

impl Default for InMemoryUserRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        InMemoryUserRepository {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl RepositoryTrait for InMemoryUserRepository {
    type Id = Uuid;
    type Entity = User;
    type Error = UserError;
    type FindOptions = UserFindRequest;
    type FindResult = UserFindResponse;

    fn save(
        &self,
        entity: &Self::Entity,
    ) -> impl Future<Output = Result<Self::Entity, Self::Error>> + Send {
        Box::pin(async move {
            let user_id = Uuid::new_v4();
            let user = User::new(
                &user_id,
                entity.get_firstname(),
                entity.get_lastname(),
                entity.get_email(),
            );
            let mut data = self.data.write().await;
            data.insert(user_id, user.clone());
            Ok(user)
        })
    }

    fn update(
        &self,
        entity_id: &Self::Id,
        entity: &Self::Entity,
    ) -> impl Future<Output = Result<Self::Entity, Self::Error>> + Send {
        Box::pin(async move {
            let mut data = self.data.write().await;
            if entity_id.ne(entity.get_id()) {
                Err(UserError::MismatchUserId {
                    id1: *entity_id,
                    id2: *entity.get_id(),
                })
            } else if data.contains_key(entity_id) {
                if data
                    .iter()
                    .filter(|(_k, v)| {
                        v.get_id().ne(entity.get_id()) && v.get_email().eq(entity.get_email())
                    })
                    .count()
                    > 0
                {
                    Err(UserError::EmailAlreadyUsedByOther {
                        email: entity.get_email().clone(),
                    })
                } else {
                    data.insert(*entity_id, entity.clone());
                    Ok(entity.clone())
                }
            } else {
                Err(UserError::UserNotExists { id: *entity_id })
            }
        })
    }

    fn delete(&self, entity_id: &Self::Id) -> impl Future<Output = Result<(), Self::Error>> + Send {
        Box::pin(async move {
            let mut data = self.data.write().await;
            if data.contains_key(entity_id) {
                data.remove(entity_id);
                Ok(())
            } else {
                Err(UserError::UserNotExists { id: *entity_id })
            }
        })
    }

    fn find_all(
        &self,
        options: &Self::FindOptions,
    ) -> impl Future<Output = Result<Self::FindResult, Self::Error>> + Send {
        Box::pin(async {
            let query = options.get_query();
            let limit = options.get_limit();
            let order_by = options.get_order_by();
            let offset = options.get_offset();
            let data = self.data.read().await;

            let mut filtered: Vec<User> = data
                .iter()
                .filter(|(k, v)| {
                    let mut found = true;
                    if query.id.is_some() {
                        found &= query.id.unwrap().eq(k);
                    }
                    if query.email.is_some() {
                        found &= query.email.as_ref().unwrap().eq(&v.get_email().to_string());
                    }
                    if query.firstname.is_some() {
                        found &= query
                            .firstname
                            .as_ref()
                            .unwrap()
                            .eq(&v.get_firstname().to_string());
                    }
                    if query.lastname.is_some() {
                        found &= query
                            .lastname
                            .as_ref()
                            .unwrap()
                            .eq(&v.get_lastname().to_string());
                    }
                    found
                })
                .map(|u| u.1.clone())
                .collect();
            filtered.sort_by(|a, b| match order_by.to_lowercase().as_str() {
                "email" => a.get_email().cmp(b.get_email()),
                "firstname" => a.get_firstname().cmp(b.get_firstname()),
                "lastname" => a.get_lastname().cmp(b.get_lastname()),
                _ => a.get_id().cmp(b.get_id()),
            });
            let mut limited = filtered.chunks(limit as usize);
            let num_page = limited.len();
            let selected = limited
                .nth(offset as usize)
                .map_or(Vec::new(), |chunk| chunk.to_vec());
            Ok(UserFindResponse::new(selected, num_page as u64))
        })
    }

    fn find_by_id(
        &self,
        entity_id: &Self::Id,
    ) -> impl Future<Output = Result<Self::Entity, Self::Error>> + Send {
        Box::pin(async {
            match self.data.read().await.get(entity_id) {
                Some(u) => Ok(u.clone()),
                None => Err(UserError::UserNotExists { id: *entity_id }),
            }
        })
    }
}

impl UserRepositoryTrait for InMemoryUserRepository {}
