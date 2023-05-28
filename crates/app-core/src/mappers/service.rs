pub trait ServiceMapper<Entity, ServiceModel> {
    // Map an Entity to a DbModel
    fn to_service(entity: Entity) -> ServiceModel;

    // Map a DbModel to an Entity
    fn to_entity(model: ServiceModel) -> Entity;
}
