use std::{cell::RefCell, collections::HashMap, rc::Rc};

use eyre::{OptionExt, Result};
use ocaml_interop::{internal::OCamlClosure, OCaml, OCamlRuntime, ToOCaml};
use slotmap::{Key, KeyData};

use crate::store::{get_value, store_value, FKey};

/// Dynamic call argument type.
pub enum Val {
    Token(FKey),
    String(String),
}

impl From<&str> for Val {
    fn from(s: &str) -> Self {
        Val::String(s.to_string())
    }
}

impl From<String> for Val {
    fn from(s: String) -> Self {
        Val::String(s)
    }
}

impl From<FKey> for Val {
    fn from(key: FKey) -> Self {
        Val::Token(key)
    }
}

fn find_function(name: &str) -> Result<OCamlClosure> {
    thread_local! {
        static MAP: RefCell<HashMap<String, OCamlClosure>> = RefCell::new(HashMap::new());
    }

    MAP.with_borrow_mut(|map| {
        if let Some(func) = map.get(name) {
            return Ok(*func);
        }

        let func = OCamlClosure::named(name).ok_or_eyre(format!("Function {} not found", name))?;
        map.insert(name.to_string(), func);
        Ok(func)
    })
}

/// Dynamic call function.
pub fn dyn_call(cr: &mut OCamlRuntime, name: &str, args: &[Val]) -> Result<FKey> {
    let func = find_function(name)?;

    let mut ocaml_args = Vec::with_capacity(args.len());
    for arg in args {
        match arg {
            Val::Token(key) => {
                let value = get_value(KeyData::from_ffi(*key))
                    .ok_or_eyre(format!("Invalid key {}", key))?;
                ocaml_args.push(value.to_boxroot(cr));
            }
            Val::String(s) => ocaml_args.push(s.to_boxroot(cr)),
        }
    }

    let result = if ocaml_args.is_empty() {
        func.call(cr, OCaml::unit().as_ref())
    } else if ocaml_args.len() == 1 {
        func.call(cr, &ocaml_args[0])
    } else if ocaml_args.len() == 2 {
        func.call2(cr, &ocaml_args[0], &ocaml_args[1])
    } else if ocaml_args.len() == 3 {
        func.call3(cr, &ocaml_args[0], &ocaml_args[1], &ocaml_args[2])
    } else {
        let mut args = ocaml_args
            .iter()
            .map(|arg| unsafe { arg.get_raw() })
            .collect::<Vec<_>>();
        func.call_n(cr, &mut args)
    };
    let result = result.root();

    Ok(store_value(Rc::new(result)).data().as_ffi())
}
