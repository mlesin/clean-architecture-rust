use async_trait::async_trait;
use std::error::Error;

use crate::{connection::DbConnection, mappers::DogFactDbMapper, models::DogFact};
use business::{gateways::dog_facts::DogFactsGateway, mappers::gateway::GatewayMapper};
use entities::dog_fact_entity::DogFactEntity;

pub struct DogFactsgatewayDB {
    pub db_connection: DbConnection,
}

#[async_trait(?Send)]
impl DogFactsGateway for DogFactsgatewayDB {
    async fn get_dog_fact_by_id(&self, dog_fact_id: i32) -> Result<DogFactEntity, Box<dyn Error>> {
        let conn = self.db_connection.get_pool().await?;

        let result = sqlx::query_as!(
            DogFact,
            "SELECT * FROM dog_facts WHERE id = $1",
            dog_fact_id
        )
        .fetch_one(&conn)
        .await;

        match result {
            Ok(model) => Ok(DogFactDbMapper::to_entity(model)),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn get_all_dog_facts(&self) -> Result<Vec<DogFactEntity>, Box<dyn Error>> {
        let conn = self.db_connection.get_pool().await?;

        let results = sqlx::query_as!(DogFact, "SELECT * FROM dog_facts")
            .fetch_all(&conn)
            .await;

        match results {
            Ok(models) => Ok(models
                .into_iter()
                .map(DogFactDbMapper::to_entity)
                .collect::<Vec<DogFactEntity>>()),
            Err(e) => Err(Box::new(e)),
        }
    }
}
