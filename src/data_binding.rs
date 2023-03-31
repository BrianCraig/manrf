use core::any::{Any, TypeId};
use std::collections::HashMap;

pub enum DataBindStore {
    /// One per app
    Global,
    /// Shared while it has the same ID
    Identified(i32),
    /// Unique to each component
    Unique,
}

pub struct DataBind {
    pub store: DataBindStore,
    pub type_id: core::any::TypeId,
}

pub struct GlobalStore {
    store: HashMap<TypeId, Box<dyn Any>>,
}

impl GlobalStore {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn insert<T: Any>(&mut self, element: T) {
        self.store.insert(TypeId::of::<T>(), Box::new(element));
    }

    pub fn get<T: Any + Default>(&mut self) -> &T {
        let type_id = TypeId::of::<T>();
        match self.store.contains_key(&type_id) {
            true => self
                .store
                .get(&type_id)
                .unwrap()
                .downcast_ref::<T>()
                .unwrap(),
            false => {
                self.store.insert(type_id, Box::new(T::default()));
                self.store
                    .get(&type_id)
                    .unwrap()
                    .downcast_ref::<T>()
                    .unwrap()
            }
        }
    }
}

#[test]
fn global_store() {
    let mut store = GlobalStore::new();
    store.insert(3);
    assert_eq!(store.get::<i32>(), &3);
    assert_eq!(store.get::<i64>(), &0);
    assert_eq!(store.get::<bool>(), &false);
    store.insert(true);
    assert_eq!(store.get::<bool>(), &true);
}
