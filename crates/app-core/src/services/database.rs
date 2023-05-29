use async_trait::async_trait;

use app_domain::entities::{CatFactEntity, DogFactEntity};
use dyno::{Tag, Tagged};
use std::{any::Any, sync::Arc};
use thiserror::Error;

#[cfg(test)]
use mockall::{predicate::*, *};

#[cfg_attr(test, automock)]
#[async_trait()]
pub trait DatabaseService {
    async fn get_repo(&self) -> Result<Box<dyn DatabaseServiceRepo + Send + Sync>, Error>;
}

#[cfg_attr(test, automock)]
#[async_trait()]
pub trait DatabaseServiceRepo {
    async fn commit(&mut self) -> Result<(), Error>;
    async fn get_dog_fact_by_id(&self, fact_id: i32) -> Result<DogFactEntity, Error>;
    async fn get_all_dog_facts(&self) -> Result<Vec<DogFactEntity>, Error>;
    async fn get_random_cat_fact(&self) -> Result<CatFactEntity, Error>;
    async fn get_all_cat_facts(&self) -> Result<Vec<CatFactEntity>, Error>;
}

/// An interface of any persistence
///
/// Persistence is anything that a Repository implementation could
/// use to store data.
#[async_trait]
pub trait Persistence: Send + Sync {
    /// Get a connection to persistence
    async fn get_connection(&self) -> Result<OwnedConnection, Error>;
}

pub type SharedPersistence = Arc<dyn Persistence>;

pub trait Connection: Any {
    fn start_transaction<'a>(&'a mut self) -> Result<OwnedTransaction<'a>, Error>;

    fn cast<'borrow>(&'borrow mut self) -> Caster<'borrow, 'static>;
}

pub type OwnedConnection = Box<dyn Connection>;

pub trait Transaction<'a> {
    fn commit(self: Box<Self>) -> Result<(), Error>;
    fn rollback(self: Box<Self>) -> Result<(), Error>;

    fn cast<'b>(&'b mut self) -> Caster<'b, 'a>
    where
        'a: 'b;
}

pub type OwnedTransaction<'a> = Box<dyn Transaction<'a> + 'a>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("wrong type")]
    WrongType,
    #[error("database error")]
    DatabaseError,
}

/// Dynamic cast helper
///
/// This struct allows an implementation of a Repository
/// to cast at runtime a type-erased [`Transaction`] or [`Connection`]
/// instance to back to a concrete type that it needs and expects.
///
/// # Safety
/// See https://users.rust-lang.org/t/help-with-using-any-to-cast-t-a-back-and-forth/69900/8
///
/// The safety is enforced by the fact that `Caster` pinky-promises to never
/// allow any reference other that `&'caster mut T` out of itself, and
/// `'a` must always outlive `'caster` or borrowck will be upset.
pub struct Caster<'borrow, 'value>(&'borrow mut (dyn Tagged<'value> + 'value));

impl<'borrow, 'value> Caster<'borrow, 'value> {
    pub fn new<I: Tag<'value>>(any: &'borrow mut I::Type) -> Self {
        Self(<dyn Tagged>::tag_mut::<I>(any))
    }

    // Returns `Result` so it's easier to handle with ? than an option
    pub fn as_mut<I: Tag<'value>>(self) -> Result<&'borrow mut I::Type, Error> {
        self.0.downcast_mut::<I>().ok_or_else(|| Error::WrongType)
    }
}

#[async_trait]
pub trait DBDogRepo {
    async fn get_all_dog_facts<'a>(
        &self,
        conn: &mut dyn Transaction<'a>,
    ) -> Result<Vec<DogFactEntity>, Error>;
    async fn get_dog_fact_by_id<'a>(
        &self,
        conn: &mut dyn Transaction<'a>,
        fact_id: i32,
    ) -> Result<DogFactEntity, Error>;
}

#[async_trait]
pub trait DBCatRepo {
    async fn get_all_cat_facts<'a>(
        &self,
        conn: &mut dyn Transaction<'a>,
    ) -> Result<Vec<CatFactEntity>, Error>;
    async fn get_random_cat_fact<'a>(
        &self,
        conn: &mut dyn Transaction<'a>,
    ) -> Result<CatFactEntity, Error>;
}
