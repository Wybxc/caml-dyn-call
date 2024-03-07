use std::cell::RefCell;

use eyre::{Context, OptionExt, Result};
use ocaml_interop::{ocaml, OCamlRuntime, ToOCaml};

mod call;
mod store;

pub use call::Val;
pub use store::Token;

thread_local! {
    static OCAML_RUNTIME: RefCell<OCamlRuntime> = RefCell::new(OCamlRuntime::init());
}

pub fn caml_dyn_call<'a>(name: &str, args: impl AsRef<[Val<'a>]>) -> Result<Token> {
    OCAML_RUNTIME.with_borrow_mut(|cr| {
        let key = call::dyn_call(cr, name, args.as_ref())?;
        Ok(key)
    })
}

impl Token {
    pub fn get_str(&self) -> Result<String> {
        OCAML_RUNTIME.with_borrow_mut(|cr| {
            let value = self
                .get_value()
                .ok_or_eyre(format!("Invalid key {}", self))?;
            Ok(value.to_rust(cr))
        })
    }
}

pub fn init(prompt: &std::path::Path) -> Result<()> {
    OCAML_RUNTIME.with_borrow_mut(|cr| {
        let command = format!(
            "#use \"{}\";;",
            prompt.canonicalize().wrap_err("No such file")?.display()
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
    ($name: literal) => {
        $crate::caml_dyn_call($name, [])
    };
    ($name: literal, $($x:expr),*) => {
        $crate::caml_dyn_call($name, [$($x.into()),*])
    };
}
