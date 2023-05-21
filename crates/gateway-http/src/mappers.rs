use crate::models::CatFactApiModel;
use business::mappers::gateway::GatewayMapper;
use entities::cat_fact_entity::CatFactEntity;

pub struct CatFactHttpMapper {}

impl GatewayMapper<CatFactEntity, CatFactApiModel> for CatFactHttpMapper {
    fn to_gateway(entity: CatFactEntity) -> CatFactApiModel {
        CatFactApiModel {
            fact: entity.fact_txt,
            length: entity.fact_length,
        }
    }

    fn to_entity(http_obj: CatFactApiModel) -> CatFactEntity {
        CatFactEntity {
            fact_txt: http_obj.fact,
            fact_length: http_obj.length,
        }
    }
}
