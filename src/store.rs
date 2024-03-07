use std::{
    cell::RefCell,
    fmt::{self, Display, Formatter},
    rc::Rc,
};

use ocaml_interop::BoxRoot;
use slotmap::{DefaultKey, Key as SlotMapKey, SlotMap};

pub type Obj = Rc<BoxRoot<String>>;

/// A token to an opaque OCaml value.
pub struct Token(DefaultKey);

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Token({})", self.0.data().as_ffi())
    }
}

thread_local! {
    static STORE: RefCell<SlotMap<DefaultKey, Obj>> = RefCell::new(SlotMap::new());
}

impl Token {
    /// Store a value in the global store and return a key to it.
    pub(crate) fn store_value(value: Obj) -> Token {
        STORE.with(|store| Token(store.borrow_mut().insert(value)))
    }

    /// Get a value from the global store by key.
    pub(crate) fn get_value(&self) -> Option<Obj> {
        STORE.with(|store| Some(store.borrow().get(self.0)?.clone()))
    }
}

impl Drop for Token {
    fn drop(&mut self) {
        STORE.with(|store| store.borrow_mut().remove(self.0));
    }
}
