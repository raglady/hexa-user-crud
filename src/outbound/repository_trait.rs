use std::{
    fmt::{Debug, Display},
    future::Future,
};

pub trait RepositoryTrait: Clone + Sync + Send + 'static {
    type Id: Clone + Sync + Send + 'static;
    type Entity: Clone + Sync + Send + 'static;
    type Error: Debug + Display;
    type FindOptions: FindOptionTrait;
    type FindResult: FindResultTrait;
    fn save(
        &self,
        entity: &Self::Entity,
    ) -> impl Future<Output = Result<Self::Entity, Self::Error>> + Send;

    fn update(
        &self,
        entity_id: &Self::Id,
        entity: &Self::Entity,
    ) -> impl Future<Output = Result<Self::Entity, Self::Error>> + Send;

    fn delete(&self, entity_id: &Self::Id) -> impl Future<Output = Result<(), Self::Error>> + Send;

    fn find_by_id(
        &self,
        entity_id: &Self::Id,
    ) -> impl Future<Output = Result<Self::Entity, Self::Error>> + Send;

    fn find_all(
        &self,
        options: &Self::FindOptions,
    ) -> impl Future<Output = Result<Self::FindResult, Self::Error>> + Send;
}

pub trait FindOptionTrait: Clone + Sync + Send + 'static {
    type QueryFilter: Clone + Sync + Send + 'static;
    fn get_query(&self) -> Self::QueryFilter;
    fn get_order_by(&self) -> String {
        String::from("id")
    }
    fn get_limit(&self) -> u16 {
        25
    }
    fn get_offset(&self) -> u64 {
        1
    }
}

pub trait FindResultTrait: Clone + Sync + Send + 'static {
    type Entity: Clone + Sync + Send + 'static;
    fn get_page_count(&self) -> u64;
    fn get_result(&self) -> impl Iterator<Item = Self::Entity>;
}
