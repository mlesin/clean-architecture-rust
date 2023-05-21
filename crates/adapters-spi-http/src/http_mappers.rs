use crate::http_models::CatFactApiModel;
use application::mappers::spi_mapper::SpiMapper;
use domain::cat_fact_entity::CatFactEntity;

pub struct CatFactHttpMapper {}

impl SpiMapper<CatFactEntity, CatFactApiModel> for CatFactHttpMapper {
    fn to_spi(entity: CatFactEntity) -> CatFactApiModel {
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
