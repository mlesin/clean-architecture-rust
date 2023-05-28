use crate::{
    dog_facts::{
        dog_facts_mappers::DogFactPresenterMapper, dog_facts_presenters::DogFactPresenter,
    },
    shared::{app_state::AppState, error_presenter::ErrorReponse},
};
use actix_web::{get, web, HttpResponse};
use app_core::{
    mappers::presenter::ApiMapper,
    usecases::{
        get_all_dog_facts::GetAllDogFactsUseCase, get_one_dog_fact_by_id::GetOneDogFactByIdUseCase,
    },
};
use app_domain::{entities::DogFactEntity, error::AppError};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_dog_facts)
        .service(get_one_dog_fact_by_id);
}

#[get("/")]
async fn get_all_dog_facts(data: web::Data<AppState>) -> Result<HttpResponse, ErrorReponse> {
    let get_all_dog_facts_usecase = GetAllDogFactsUseCase::new(&*data.db_service);
    let dog_facts: Result<Vec<DogFactEntity>, AppError> = get_all_dog_facts_usecase.execute().await;

    dog_facts.map_err(ErrorReponse::map_io_error).map(|facts| {
        HttpResponse::Ok().json(
            facts
                .into_iter()
                .map(DogFactPresenterMapper::to_api)
                .collect::<Vec<DogFactPresenter>>(),
        )
    })
}

#[get("/{fact_id}")]
async fn get_one_dog_fact_by_id(
    data: web::Data<AppState>,
    path: web::Path<(i32,)>,
) -> Result<HttpResponse, ErrorReponse> {
    let fact_id = path.into_inner().0;
    let get_one_dog_fact_by_id_usecase = GetOneDogFactByIdUseCase::new(&fact_id, &*data.db_service);
    let dog_fact = get_one_dog_fact_by_id_usecase.execute().await;

    dog_fact
        .map_err(ErrorReponse::map_io_error)
        .map(|fact| HttpResponse::Ok().json(DogFactPresenterMapper::to_api(fact)))
}
