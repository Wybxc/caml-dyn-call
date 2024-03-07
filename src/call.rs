use std::{cell::RefCell, collections::HashMap, rc::Rc};

use eyre::{OptionExt, Result};
use ocaml_interop::{internal::OCamlClosure, OCaml, OCamlRuntime, ToOCaml};

use crate::store::Token;

/// Dynamic call argument type.
pub enum Val<'a> {
    /// A token to a value stored in the global store.
    Token(&'a Token),
    /// A string.
    Str(&'a str),
}

impl<'a> From<&'a str> for Val<'a> {
    fn from(s: &'a str) -> Self {
        Val::Str(s)
    }
}

impl<'a> From<&'a Token> for Val<'a> {
    fn from(key: &'a Token) -> Self {
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
pub fn dyn_call(cr: &mut OCamlRuntime, name: &str, args: &[Val]) -> Result<Token> {
    let func = find_function(name)?;

    let mut ocaml_args = Vec::with_capacity(args.len());
    for arg in args {
        match arg {
            Val::Token(key) => {
                let value = key.get_value().ok_or_eyre(format!("Invalid key {}", key))?;
                ocaml_args.push(value.to_boxroot(cr));
            }
            Val::Str(s) => ocaml_args.push(s.to_boxroot(cr)),
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

    Ok(Token::store_value(Rc::new(result)))
}
