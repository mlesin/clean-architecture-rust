use std::marker::PhantomData;

use crate::{
    cat_facts::{
        cat_facts_mappers::CatFactPresenterMapper, cat_facts_presenters::CatFactPresenter,
    },
    shared::{app_state::AppState, error_presenter::ErrorReponse},
};
use actix_web::{web, HttpResponse};
use app_core::{
    mappers::presenter::ApiMapper,
    services::{CatRepo, Persistence},
};
use app_core::{
    services::Transaction,
    usecases::{
        get_all_cat_facts::GetAllCatFactsUseCase,
        get_one_random_cat_fact::GetOneRandomCatFactUseCase,
    },
};

pub struct CatFactControllers<P, R> {
    persistance: PhantomData<P>,
    cat_repository: PhantomData<R>,
}

impl<P, R> CatFactControllers<P, R>
where
    P: Persistence + Clone,
    R: CatRepo<P>,
    <P as Persistence>::Transaction: Transaction,
{
    pub fn routes(cfg: &mut web::ServiceConfig) {
        cfg.service(web::resource("/").route(web::get().to(Self::get_all_cat_facts)))
            .service(web::resource("/random").route(web::get().to(Self::get_one_random_cat_fact)));
    }

    async fn get_all_cat_facts(data: web::Data<AppState<P>>) -> Result<HttpResponse, ErrorReponse> {
        let get_all_cat_facts_usecase =
            GetAllCatFactsUseCase::<P, R>::new(data.persistence_service.clone());
        let facts = get_all_cat_facts_usecase.execute().await?;

        Ok(HttpResponse::Ok().json(
            facts
                .into_iter()
                .map(CatFactPresenterMapper::to_api)
                .collect::<Vec<CatFactPresenter>>(),
        ))
    }

    async fn get_one_random_cat_fact(
        data: web::Data<AppState<P>>,
    ) -> Result<HttpResponse, ErrorReponse> {
        let get_one_random_cat_fact_usecase =
            GetOneRandomCatFactUseCase::<P, R>::new(data.persistence_service.clone());
        let fact = get_one_random_cat_fact_usecase.execute().await?;

        Ok(HttpResponse::Ok().json(CatFactPresenterMapper::to_api(fact)))
    }
}
