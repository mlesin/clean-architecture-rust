use crate::cat_facts::cat_facts_payloads::CatFactPayload;
use crate::cat_facts::cat_facts_presenters::CatFactPresenter;
use app_core::mappers::presenter::ApiMapper;
use app_domain::entities::CatFactEntity;

pub struct CatFactPresenterMapper {}

impl ApiMapper<CatFactEntity, CatFactPresenter, CatFactPayload> for CatFactPresenterMapper {
    fn to_api(entity: CatFactEntity) -> CatFactPresenter {
        CatFactPresenter {
            fact: entity.fact_txt,
            nb_chars: entity.fact_length,
        }
    }

    fn to_entity(_payload: CatFactPayload) -> CatFactEntity {
        panic!("not implemented");
    }
}
