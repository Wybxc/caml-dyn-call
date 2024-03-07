use std::cell::RefCell;

use ocaml_interop::{ocaml, OCamlRuntime, ToOCaml};
use slotmap::KeyData;

mod call;
mod store;

pub use call::Val;

thread_local! {
    static OCAML_RUNTIME: RefCell<OCamlRuntime> = RefCell::new(OCamlRuntime::init());
}

pub fn caml_dyn_call(name: &str, args: Vec<Val>) -> Result<store::FKey, String> {
    OCAML_RUNTIME.with_borrow_mut(|cr| {
        let key = call::dyn_call(cr, name, &args)?;
        Ok(key)
    })
}

pub fn get_str(key: store::FKey) -> Result<String, String> {
    OCAML_RUNTIME.with_borrow_mut(|cr| {
        let value = crate::store::get_value(KeyData::from_ffi(key))
            .ok_or_else(|| format!("Invalid key {}", key))?;
        Ok(value.to_rust(cr))
    })
}

pub fn get_str_dispose(key: store::FKey) -> Result<String, String> {
    let result = get_str(key);
    dispose(key);
    result
}

pub fn dispose(key: store::FKey) {
    store::remove_value(KeyData::from_ffi(key));
}

pub fn init(prompt: &std::path::Path) -> Result<(), String> {
    OCAML_RUNTIME.with_borrow_mut(|cr| {
        let command = format!(
            "#use \"{}\";;",
            prompt
                .canonicalize()
                .map_err(|e| format!("Invalid path: {}", e))?
                .display()
        );
        let command = command.to_boxroot(cr);
        eval(cr, &command);
        Ok(())
    })
}

ocaml! {
    fn eval(command: String);
}

#[macro_export]
macro_rules! dyn_call {
    ($name: literal, $($x:expr),*) => {
        $crate::caml_dyn_call($name, vec![$($x.into()),*])
    };
}
