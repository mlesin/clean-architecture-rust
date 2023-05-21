use crate::models::DogFact;
use application::mappers::spi_mapper::SpiMapper;
use domain::dog_fact_entity::DogFactEntity;

pub struct DogFactDbMapper {}

impl SpiMapper<DogFactEntity, DogFact> for DogFactDbMapper {
    fn to_spi(entity: DogFactEntity) -> DogFact {
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
