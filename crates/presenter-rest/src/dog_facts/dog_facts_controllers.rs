use std::marker::PhantomData;

use crate::{
    dog_facts::{
        dog_facts_mappers::DogFactPresenterMapper, dog_facts_presenters::DogFactPresenter,
    },
    shared::{app_state::AppState, error_presenter::ErrorReponse},
};
use actix_web::{web, HttpResponse};
use app_core::{
    mappers::presenter::ApiMapper,
    services::{DBCatRepo, DBDogRepo, Persistence, Transaction},
    usecases::{
        get_all_dog_facts::GetAllDogFactsUseCase, get_one_dog_fact_by_id::GetOneDogFactByIdUseCase,
    },
};

pub struct DogFactControllers<P, DR, CR> {
    persistance: PhantomData<P>,
    dog_repository: PhantomData<DR>,
    cat_repository: PhantomData<CR>,
}

impl<'a, P, DR, CR> DogFactControllers<P, DR, CR>
where
    P: Persistence<'a> + Clone + 'static,
    DR: DBDogRepo<'a, P> + Copy + 'static,
    CR: DBCatRepo<'a, P> + Copy + 'static,
    <P as Persistence<'a>>::Transaction: Transaction,
{
    pub fn routes(cfg: &mut web::ServiceConfig) {
        cfg.service(web::resource("/").route(web::get().to(Self::get_all)))
            .service(web::resource("/{fact_id}").route(web::get().to(Self::get_one_by_id)));
    }

    async fn get_all(data: web::Data<AppState<P, DR, CR>>) -> Result<HttpResponse, ErrorReponse> {
        let get_all_dog_facts_usecase =
            GetAllDogFactsUseCase::<P, DR>::new(data.persistence_service.clone(), data.dog_repo);
        let dog_facts = get_all_dog_facts_usecase
            .execute()
            .await
            .map_err(ErrorReponse::map_io_error)?;

        Ok(HttpResponse::Ok().json(
            dog_facts
                .into_iter()
                .map(DogFactPresenterMapper::to_api)
                .collect::<Vec<DogFactPresenter>>(),
        ))
    }

    async fn get_one_by_id(
        data: web::Data<AppState<P, DR, CR>>,
        path: web::Path<(i32,)>,
    ) -> Result<HttpResponse, ErrorReponse> {
        let fact_id = path.into_inner().0;
        let get_one_dog_fact_by_id_usecase =
            GetOneDogFactByIdUseCase::<P, DR>::new(data.persistence_service.clone(), data.dog_repo);
        let dog_fact = get_one_dog_fact_by_id_usecase
            .execute(&fact_id)
            .await
            .map_err(ErrorReponse::map_io_error)?;

        Ok(HttpResponse::Ok().json(DogFactPresenterMapper::to_api(dog_fact)))
    }
}
