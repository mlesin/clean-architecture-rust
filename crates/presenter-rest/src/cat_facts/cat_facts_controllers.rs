use crate::{
    cat_facts::{
        cat_facts_mappers::CatFactPresenterMapper, cat_facts_presenters::CatFactPresenter,
    },
    shared::{app_state::AppState, error_presenter::ErrorReponse},
};
use actix_web::{get, web, HttpResponse};
use app_core::mappers::presenter::ApiMapper;
use app_core::usecases::{
    get_all_cat_facts::GetAllCatFactsUseCase, get_one_random_cat_fact::GetOneRandomCatFactUseCase,
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_cat_facts)
        .service(get_one_random_cat_fact);
}

#[get("/")]
async fn get_all_cat_facts(data: web::Data<AppState>) -> Result<HttpResponse, ErrorReponse> {
    let get_all_cat_facts_usecase = GetAllCatFactsUseCase::new(&*data.db_service);
    let cat_facts = get_all_cat_facts_usecase.execute().await;

    cat_facts.map_err(ErrorReponse::map_io_error).map(|facts| {
        HttpResponse::Ok().json(
            facts
                .into_iter()
                .map(CatFactPresenterMapper::to_api)
                .collect::<Vec<CatFactPresenter>>(),
        )
    })
}

#[get("/random")]
async fn get_one_random_cat_fact(data: web::Data<AppState>) -> Result<HttpResponse, ErrorReponse> {
    let get_one_random_cat_fact_usecase = GetOneRandomCatFactUseCase::new(&*data.db_service);
    let cat_fact = get_one_random_cat_fact_usecase.execute().await;

    cat_fact
        .map_err(ErrorReponse::map_io_error)
        .map(|fact| HttpResponse::Ok().json(CatFactPresenterMapper::to_api(fact)))
}
