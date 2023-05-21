use crate::models::DogFact;
use business::mappers::gateway::GatewayMapper;
use entities::dog_fact_entity::DogFactEntity;

pub struct DogFactDbMapper {}

impl GatewayMapper<DogFactEntity, DogFact> for DogFactDbMapper {
    fn to_gateway(entity: DogFactEntity) -> DogFact {
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
