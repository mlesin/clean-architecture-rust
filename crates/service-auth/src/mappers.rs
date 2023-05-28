use crate::models::CatFactApiModel;
use app_core::mappers::service::ServiceMapper;
use app_domain::entities::CatFactEntity;

pub struct CatFactHttpMapper {}

impl ServiceMapper<CatFactEntity, CatFactApiModel> for CatFactHttpMapper {
    fn to_service(entity: CatFactEntity) -> CatFactApiModel {
        CatFactApiModel {
            fact: entity.fact_txt,
            length: entity.fact_id,
        }
    }

    fn to_entity(http_obj: CatFactApiModel) -> CatFactEntity {
        CatFactEntity {
            fact_txt: http_obj.fact,
            fact_id: http_obj.length,
        }
    }
}
