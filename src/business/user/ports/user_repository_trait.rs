use crate::outbound::repository_trait::RepositoryTrait;
pub trait UserRepositoryTrait: RepositoryTrait + Sync + Send + 'static {}
