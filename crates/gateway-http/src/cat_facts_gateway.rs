use async_trait::async_trait;
use std::error::Error;

use crate::{
    connection::HttpConnection,
    mappers::CatFactHttpMapper,
    models::{CatFactApiModel, CatFactsApiModel},
};
use business::{gateways::cat_facts::CatFactsGateway, mappers::gateway::GatewayMapper};
use entities::cat_fact_entity::CatFactEntity;

pub struct CatFactsgatewayHTTP {
    pub http_connection: HttpConnection,
    pub source: String,
}

#[async_trait(?Send)]
impl CatFactsGateway for CatFactsgatewayHTTP {
    async fn get_random_cat_fact(&self) -> Result<CatFactEntity, Box<dyn Error>> {
        let response = self
            .http_connection
            .client()
            .get(&format!("{}/fact", &self.source))
            .send()
            .await;

        match response {
            Ok(r) => {
                let json = r.json::<CatFactApiModel>().await;

                match json {
                    Ok(http_obj) => Ok(CatFactHttpMapper::to_entity(http_obj)),
                    Err(e) => Err(Box::new(e)),
                }
            }
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn get_all_cat_facts(&self) -> Result<Vec<CatFactEntity>, Box<dyn Error>> {
        let response = self
            .http_connection
            .client()
            .get(&format!("{}/facts", &self.source))
            .send()
            .await;

        match response {
            Ok(r) => {
                let json = r.json::<CatFactsApiModel>().await;

                match json {
                    Ok(j) => Ok(j
                        .data
                        .into_iter()
                        .map(CatFactHttpMapper::to_entity)
                        .collect::<Vec<CatFactEntity>>()),
                    Err(e) => Err(Box::new(e)),
                }
            }
            Err(e) => Err(Box::new(e)),
        }
    }
}
