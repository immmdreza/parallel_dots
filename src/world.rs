use std::{
    any::{Any, TypeId},
    collections::{HashMap, HashSet},
    marker::PhantomData,
};

pub struct Entity {
    id: usize,
    component_id: Option<usize>,
    inner: Box<dyn Any>,
}

impl Entity {
    fn new<T>(id: usize, inner: T) -> Self
    where
        T: Any + 'static,
    {
        Self {
            id,
            component_id: None,
            inner: Box::new(inner),
        }
    }

    fn new_composite<T>(id: usize, component_id: usize, inner: T) -> Self
    where
        T: Any + 'static,
    {
        Self {
            id,
            component_id: Some(component_id),
            inner: Box::new(inner),
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

pub struct TypedEntity<'a, T: Any, TEntity> {
    _phantom: PhantomData<&'a T>,
    entity: TEntity,
}

impl<'a, T: Any, TEntity> TypedEntity<'a, T, TEntity> {
    fn new(entity: TEntity) -> Self {
        Self {
            _phantom: PhantomData,
            entity,
        }
    }
}

impl<'a, T: Any> TypedEntity<'a, T, &'a Entity> {
    pub fn as_ref(&self) -> Option<&'a T> {
        self.entity.inner.downcast_ref()
    }
}

impl<'a, T: Any> TypedEntity<'a, T, &'a mut Entity> {
    pub fn as_mut(&'a mut self) -> Option<&'a mut T> {
        self.entity.inner.downcast_mut()
    }
}

impl<'a, T: Any> TypedEntity<'a, T, Entity> {
    pub fn inner(self) -> Option<T> {
        self.entity.inner.downcast().ok().map(|e| *e)
    }
}

pub struct Component {
    id: usize,
    entities: HashSet<usize>,
}

impl Component {
    fn new(id: usize, entities: impl IntoIterator<Item = usize>) -> Self {
        Self {
            id,
            entities: entities.into_iter().collect(),
        }
    }
}

pub struct World {
    component_id_counter: usize,
    entity_id_counter: usize,
    components: HashMap<usize, Component>,
    entities: HashMap<TypeId, HashMap<usize, Entity>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entity_id_counter: 1,
            component_id_counter: 1,
            components: HashMap::new(),
            entities: HashMap::new(),
        }
    }

    pub fn insert<T>(&mut self, val: T) -> usize
    where
        T: Any + 'static,
    {
        let id = self.generate_new_entity_id();

        let category = self.entities.entry(TypeId::of::<T>());
        category.or_default().insert(id, Entity::new(id, val));
        id
    }

    pub fn insert_composite<T1, T2>(&mut self, val1: T1, val2: T2) -> usize
    where
        T1: Any + 'static,
        T2: Any + 'static,
    {
        let id1 = self.generate_new_entity_id();
        let id2 = self.generate_new_entity_id();

        let comp_id = self.generate_new_component_id();

        self.entities
            .entry(TypeId::of::<T1>())
            .or_default()
            .insert(id1, Entity::new_composite(id1, comp_id, val1));
        self.entities
            .entry(TypeId::of::<T2>())
            .or_default()
            .insert(id2, Entity::new_composite(id2, comp_id, val2));

        self.components
            .insert(comp_id, Component::new(comp_id, [id1, id2]));

        comp_id
    }

    pub fn insert_to_component<T>(&mut self, component_id: usize, val: T) -> Option<usize>
    where
        T: Any + 'static,
    {
        if !self.components.contains_key(&component_id) {
            return None;
        }

        let id = self.generate_new_entity_id();

        self.entities
            .entry(TypeId::of::<T>())
            .or_default()
            .insert(id, Entity::new_composite(id, component_id, val));

        self.components.get_mut(&component_id)?.entities.insert(id);

        Some(id)
    }

    pub fn get_entity<'a, T>(&'a self, id: &usize) -> Option<TypedEntity<'a, T, &'a Entity>>
    where
        T: Any + 'static,
    {
        self.entities
            .get(&TypeId::of::<T>())
            .map(|e| e.get(id).map(|entity| TypedEntity::new(entity)))
            .flatten()
    }

    pub fn get<T>(&self, id: &usize) -> Option<&T>
    where
        T: Any + 'static,
    {
        self.get_entity::<T>(id).map(|e| e.as_ref()).flatten()
    }

    pub fn get_entity_mut<'a, T>(
        &'a mut self,
        id: &usize,
    ) -> Option<TypedEntity<'a, T, &'a mut Entity>>
    where
        T: Any + 'static,
    {
        self.entities
            .get_mut(&TypeId::of::<T>())
            .map(|e| e.get_mut(id).map(|entity| TypedEntity::new(entity)))
            .flatten()
    }

    pub fn get_mut<T>(&mut self, id: &usize) -> Option<&mut T>
    where
        T: Any + 'static,
    {
        self.entities
            .get_mut(&TypeId::of::<T>())
            .map(|e| e.get_mut(id).map(|entity| entity.inner.downcast_mut()))
            .flatten()?
    }

    pub fn remove_entity<T>(&'_ mut self, id: &usize) -> Option<TypedEntity<'_, T, Entity>>
    where
        T: Any + 'static,
    {
        let entity = self
            .entities
            .get_mut(&TypeId::of::<T>())?
            .remove(id)
            .map(|e| TypedEntity::new(e));

        if let Some(entity) = &entity {
            if let Some(comp_id) = entity.entity.component_id {
                let comp = self.components.get_mut(&comp_id).unwrap();
                comp.entities.remove(&entity.entity.id);
                if comp.entities.is_empty() {
                    self.components.remove(&comp_id);
                }
            }
        }

        entity
    }

    pub fn remove<T>(&mut self, id: &usize) -> Option<T>
    where
        T: Any + 'static,
    {
        self.remove_entity::<T>(id).map(|e| e.inner()).flatten()
    }

    fn generate_new_entity_id(&mut self) -> usize {
        let current = self.entity_id_counter;
        self.entity_id_counter += 1;
        current
    }

    fn generate_new_component_id(&mut self) -> usize {
        let current = self.component_id_counter;
        self.component_id_counter += 1;
        current
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let mut world = World::new();
        let id = world.insert("Hello World".to_string());

        let t = world.get_mut::<String>(&id).unwrap();
        t.push_str(" My dear!");

        assert_eq!(
            Some("Hello World My dear!".to_string()),
            world.get::<String>(&id).cloned()
        );

        assert_eq!(
            Some("Hello World My dear!".to_string()),
            world.remove::<String>(&id)
        );

        assert_eq!(2, world.insert("Goodbye World.".to_string()));
        assert_eq!(None, world.get::<String>(&id).cloned());
    }
}
