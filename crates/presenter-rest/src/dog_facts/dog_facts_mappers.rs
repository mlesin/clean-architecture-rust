use crate::dog_facts::{
    dog_facts_payloads::DogFactPayload, dog_facts_presenters::DogFactPresenter,
};
use app_core::mappers::presenter::ApiMapper;
use app_domain::entities::DogFactEntity;

pub struct DogFactPresenterMapper {}

impl ApiMapper<DogFactEntity, DogFactPresenter, DogFactPayload> for DogFactPresenterMapper {
    fn to_api(entity: DogFactEntity) -> DogFactPresenter {
        DogFactPresenter {
            fact_id: entity.fact_id,
            txt: entity.fact,
        }
    }

    fn to_entity(_payload: DogFactPayload) -> DogFactEntity {
        panic!("not implemented");
    }
}
