use std::marker::PhantomData;

use actix_web::web;
use app_core::services::{CatRepo, DogRepo, Persistence, Transaction};

use crate::{
    cat_facts::cat_facts_controllers::CatFactControllers,
    dog_facts::dog_facts_controllers::DogFactControllers,
};

pub struct RestControllers<P, D, C> {
    persistance: PhantomData<P>,
    dog_repository: PhantomData<D>,
    cat_repository: PhantomData<C>,
}

impl<P, D, C> RestControllers<P, D, C>
where
    P: Persistence + Clone,
    <P as Persistence>::Transaction: Transaction,
    D: DogRepo<P>,
    C: CatRepo<P>,
{
    pub fn routes(config: &mut web::ServiceConfig) {
        config
            .service(web::scope("/api/v1/dogs").configure(DogFactControllers::<P, D>::routes))
            .service(web::scope("/api/v1/cats").configure(CatFactControllers::<P, C>::routes));
    }
}
