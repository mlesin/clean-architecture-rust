pub trait GatewayMapper<Entity, GatewayModel> {
    // Map an Entity to a DbModel
    fn to_gateway(entity: Entity) -> GatewayModel;

    // Map a DbModel to an Entity
    fn to_entity(model: GatewayModel) -> Entity;
}
