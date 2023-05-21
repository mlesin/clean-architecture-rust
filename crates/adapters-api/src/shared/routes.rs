use actix_web::web;

use crate::{cat_facts::cat_facts_controllers, dog_facts::dog_facts_controllers};

pub fn routes(config: &mut web::ServiceConfig) {
    config
        .service(web::scope("/api/v1/dogs").configure(dog_facts_controllers::routes))
        .service(web::scope("/api/v1/cats").configure(cat_facts_controllers::routes));
}
