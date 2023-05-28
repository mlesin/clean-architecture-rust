use crate::models::{CatFact, DogFact};
use app_core::mappers::service::ServiceMapper;
use app_domain::entities::{CatFactEntity, DogFactEntity};

pub struct DogFactDbMapper {}

impl ServiceMapper<DogFactEntity, DogFact> for DogFactDbMapper {
    fn to_service(entity: DogFactEntity) -> DogFact {
        DogFact {
            id: entity.fact_id,
            fact: entity.fact,
        }
    }

    fn to_entity(model: DogFact) -> DogFactEntity {
        DogFactEntity {
            fact_id: model.id,
            fact: model.fact,
        }
    }
}

pub struct CatFactDbMapper {}

impl ServiceMapper<CatFactEntity, CatFact> for CatFactDbMapper {
    fn to_service(entity: CatFactEntity) -> CatFact {
        CatFact {
            id: entity.fact_id,
            fact: entity.fact_txt,
        }
    }

    fn to_entity(model: CatFact) -> CatFactEntity {
        CatFactEntity {
            fact_id: model.id,
            fact_txt: model.fact,
        }
    }
}
