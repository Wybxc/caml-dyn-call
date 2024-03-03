use std::rc::Rc;

use ocaml_interop::{OCaml, OCamlRuntime, ToOCaml};
use serde::{Deserialize, Serialize};
use slotmap::Key as SlotMapKey;

use crate::store::{get_value, store_value, Key};

/// Dynamic call argument type.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Val {
    Token(Key),
    String(String),
}

/// Dynamic call function.
pub fn dyn_call(cr: &mut OCamlRuntime, name: &str, args: &[Val]) -> Result<Key, String> {
    let func = ocaml_interop::internal::OCamlClosure::named(name)
        .ok_or_else(|| format!("Function {} not found", name))?;

    let mut ocaml_args = Vec::with_capacity(args.len());
    for arg in args {
        match arg {
            Val::Token(key) => {
                let value = get_value(*key)
                    .ok_or_else(|| format!("Invalid key {}", key.data().as_ffi()))?;
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
    Ok(store_value(Rc::new(result)))
}
