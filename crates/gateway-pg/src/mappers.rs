use crate::models::DogFact;
use app_core::mappers::gateway::GatewayMapper;
use app_domain::entities::DogFactEntity;

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
