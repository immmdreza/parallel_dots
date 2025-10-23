use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

pub struct Entity {
    id: usize,
    inner: Box<dyn Any>,
}

impl Entity {
    fn new<T>(id: usize, inner: T) -> Self
    where
        T: Any + 'static,
    {
        Self {
            id,
            inner: Box::new(inner),
        }
    }
}

pub struct World {
    id_counter: usize,
    entities: HashMap<TypeId, HashMap<usize, Entity>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            id_counter: 1,
            entities: HashMap::new(),
        }
    }

    pub fn insert<T>(&mut self, val: T) -> usize
    where
        T: Any + 'static,
    {
        let id = self.generate_new_id();

        let category = self.entities.entry(TypeId::of::<T>());
        category.or_default().insert(id, Entity::new(id, val));
        id
    }

    pub fn get_entity<T>(&self, id: &usize) -> Option<&Entity>
    where
        T: Any + 'static,
    {
        self.entities
            .get(&TypeId::of::<T>())
            .map(|e| e.get(id))
            .flatten()
    }

    pub fn get<T>(&self, id: &usize) -> Option<&T>
    where
        T: Any + 'static,
    {
        self.get_entity::<T>(id)
            .map(|e| e.inner.downcast_ref())
            .flatten()
    }

    pub fn get_entity_mut<T>(&mut self, id: &usize) -> Option<&mut Entity>
    where
        T: Any + 'static,
    {
        self.entities
            .get_mut(&TypeId::of::<T>())
            .map(|e| e.get_mut(id))
            .flatten()
    }

    pub fn get_mut<T>(&mut self, id: &usize) -> Option<&mut T>
    where
        T: Any + 'static,
    {
        self.get_entity_mut::<T>(id)
            .map(|e| e.inner.downcast_mut())
            .flatten()
    }

    pub fn remove_entity<T>(&mut self, id: &usize) -> Option<Entity>
    where
        T: Any + 'static,
    {
        self.entities.get_mut(&TypeId::of::<T>())?.remove(id)
    }

    pub fn remove<T>(&mut self, id: &usize) -> Option<T>
    where
        T: Any + 'static,
    {
        self.remove_entity::<T>(id)
            .map(|e| e.inner.downcast().ok())
            .flatten()
            .map(|e| *e)
    }

    fn generate_new_id(&mut self) -> usize {
        let current = self.id_counter;
        self.id_counter += 1;
        current
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let mut world = World::new();
        world.insert("Hello World".to_string());
    }
}
