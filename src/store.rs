use std::{cell::RefCell, rc::Rc};

use ocaml_interop::BoxRoot;
use slotmap::{DefaultKey, SlotMap};

pub type Obj = Rc<BoxRoot<String>>;
pub type Key = DefaultKey;

thread_local! {
    static STORE: RefCell<SlotMap<DefaultKey, Obj>> = RefCell::new(SlotMap::new());
}

/// Store a value in the global store and return a key to it.
pub fn store_value(value: Obj) -> DefaultKey {
    STORE.with(|store| store.borrow_mut().insert(value))
}

/// Get a value from the global store by key.
pub fn get_value(key: DefaultKey) -> Option<Obj> {
    STORE.with(|store| Some(store.borrow().get(key)?.clone()))
}

/// Remove a value from the global store by key.
pub fn remove_value(key: DefaultKey) -> Option<Obj> {
    STORE.with(|store| store.borrow_mut().remove(key))
}
