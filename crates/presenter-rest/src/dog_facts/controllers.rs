use std::marker::PhantomData;

use super::{mappers::DogFactPresenterMapper, presenters::DogFactPresenter};
use crate::shared::{app_state::RestAppState, error::ErrorReponse};
use actix_web::{web, HttpResponse};
use app_core::{
    mappers::presenter::ApiMapper,
    services::{DogRepo, Persistence, Transaction},
    usecases::{
        get_all_dog_facts::GetAllDogFactsUseCase, get_one_dog_fact_by_id::GetOneDogFactByIdUseCase,
    },
};

pub struct DogFactControllers<P, R> {
    persistance: PhantomData<P>,
    dog_repository: PhantomData<R>,
}

impl<P, R> DogFactControllers<P, R>
where
    P: Persistence + Clone,
    R: DogRepo<P>,
    <P as Persistence>::Transaction: Transaction,
{
    pub fn routes(cfg: &mut web::ServiceConfig) {
        cfg.service(web::resource("/").route(web::get().to(Self::get_all)))
            .service(web::resource("/{fact_id}").route(web::get().to(Self::get_one_by_id)));
    }

    async fn get_all(data: web::Data<RestAppState<P>>) -> Result<HttpResponse, ErrorReponse> {
        let get_all_dog_facts_usecase =
            GetAllDogFactsUseCase::<P, R>::new(data.persistence_service.clone());
        let facts = get_all_dog_facts_usecase.execute().await?;

        Ok(HttpResponse::Ok().json(
            facts
                .into_iter()
                .map(DogFactPresenterMapper::to_api)
                .collect::<Vec<DogFactPresenter>>(),
        ))
    }

    async fn get_one_by_id(
        data: web::Data<RestAppState<P>>,
        path: web::Path<(i32,)>,
    ) -> Result<HttpResponse, ErrorReponse> {
        let fact_id = path.into_inner().0;
        let get_one_dog_fact_by_id_usecase =
            GetOneDogFactByIdUseCase::<P, R>::new(data.persistence_service.clone());
        let fact = get_one_dog_fact_by_id_usecase.execute(&fact_id).await?;

        Ok(HttpResponse::Ok().json(DogFactPresenterMapper::to_api(fact)))
    }
}
