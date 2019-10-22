/// Represents an entity.
#[derive(Copy, Clone, PartialEq, Hash, Eq, Debug, Ord, PartialOrd, Default)]
pub struct Entity(pub u32);

impl From<u32> for Entity {
    fn from(u: u32) -> Self {
        Entity(u)
    }
}

/// This trait is used to define a custom store for entities.
/// A entity container is used for entity iteration inside of the
/// system's run methods.
pub trait EntityStore {
    /// Registers the give 'entity'.
    fn register_entity(&mut self, entity: impl Into<Entity>);

    /// Removes the given 'entity'.
    fn remove_entity(&mut self, entity: impl Into<Entity>);
}

/// VecEntityStore is the default vector based implementation of an entity store.
#[derive(Default)]
pub struct VecEntityStore {
    pub inner: Vec<Entity>,
}

impl EntityStore for VecEntityStore {
    fn register_entity(&mut self, entity: impl Into<Entity>) {
        self.inner.push(entity.into());
    }

    fn remove_entity(&mut self, entity: impl Into<Entity>) {
        let entity = entity.into();
        self.inner
            .iter()
            .position(|&n| n == entity)
            .map(|e| self.inner.remove(e));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_from() {
        let entity = Entity::from(2);
        assert_eq!(entity.0, 2);

        let entity = Entity::from(5);
        assert_eq!(entity.0, 5);
    }

     #[test]
    fn test_register_entity() {
        let mut store = VecEntityStore::default();
        let entity_one = Entity::from(1);
        store.register_entity(entity_one);
        let entity_two = Entity::from(2);
        store.register_entity(entity_two);
        let entity_three = Entity::from(3);

        assert!(store.inner.contains(&entity_one));
        assert!(store.inner.contains(&entity_two));
        assert!(!store.inner.contains(&entity_three));
    }
}
