pub trait SpiMapper<Entity, SpiModel> {
    // Map an Entity to a DbModel
    fn to_spi(entity: Entity) -> SpiModel;

    // Map a DbModel to an Entity
    fn to_entity(model: SpiModel) -> Entity;
}
