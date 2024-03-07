//! Dynamically call OCaml functions by name.
//!
//! # Overview
//!
//! The `Callback.register` feature in OCaml enables the registration of OCaml
//! functions to be discovered by name in native code. This crate offers a
//! mechanism to invoke these functions by name from Rust. To manage arbitrary
//! OCaml values, the crate treats all OCaml values as opaque, with the
//! exception of strings. It is recommended to create a lightweight wrapper
//! around the OCaml functions you intend to call, ensuring that data
//! transferred between Rust and OCaml can be represented in string format.
//!
//! This crate is specifically designed to function with the OCaml toplevel.
//! Consequently, the function registration should be executed in an OCaml
//! toplevel script (which is notably marked by the `;;` delimiter), rather than
//! a compiled OCaml library. This script is stored in a file and supplied to
//! the [`init`] function.
//!
//! Note that nearly all functions in this crate are signatured as `unsafe`
//! due to the inability to guarantee type safety when dynamically calling OCaml
//! functions. However, this doesn't imply that you are responsible for manual
//! memory management. The crate automatically handles memory management.
//!
//! # Example
//!
//! simple.ml:
//!
//! ```ocaml
//! let add (x: int) (y: int) = x + y in
//! Callback.register "add" add;;
//!
//! let sub (x: int) (y: int) = x - y in
//! Callback.register "sub" sub;;
//!
//! let parse_int (s: string) = int_of_string s in
//! Callback.register "parse_int" parse_int;;
//!
//! let print_int (i: int) = string_of_int i in
//! Callback.register "print_int" print_int;;
//! ```
//!
//! main.rs:
//!
//! ```rust,no_run
//! use caml_dyn_call::*;
//!
//! fn main() -> eyre::Result<()> {
//!     init(Some(std::path::Path::new("./simple.ml")))?;
//!
//!     let (a, b, c, d) = unsafe {
//!         let a = dyn_call!("parse_int", "123")?;
//!         let b = dyn_call!("parse_int", "456")?;
//!         let c = dyn_call!("add", &a, &b)?;
//!         let d = dyn_call!("print_int", &c)?.get_str()?;
//!         (a, b, c, d)
//!     };
//!
//!     println!("a: {}", a);
//!     println!("b: {}", b);
//!     println!("c: {}", c);
//!     println!("d: {}", d);
//!
//!     Ok(())
//! }
//! ```
#![deny(missing_docs)]
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

/// Call an OCaml function by name.
///
/// # Safety
/// All arguments must be of the correct type for the OCaml function.
pub unsafe fn caml_dyn_call<'a>(name: &str, args: impl AsRef<[Val<'a>]>) -> Result<Token> {
    OCAML_RUNTIME.with_borrow_mut(|cr| {
        let key = call::dyn_call(cr, name, args.as_ref())?;
        Ok(key)
    })
}

impl Token {
    /// Get the value of the token as a string.
    ///
    /// # Safety
    /// The value behind the token must be an OCaml string.
    pub unsafe fn get_str(&self) -> Result<String> {
        OCAML_RUNTIME.with_borrow_mut(|cr| {
            let value = self
                .get_value()
                .ok_or_eyre(format!("Invalid key {}", self))?;
            Ok(value.to_rust(cr))
        })
    }
}

/// Initialize the OCaml runtime and run an initialization script.
pub fn init(init: Option<&std::path::Path>) -> Result<()> {
    OCAML_RUNTIME.with_borrow_mut(|cr| {
        if let Some(init) = init {
            let command = format!(
                "#use \"{}\";;",
                init.canonicalize().wrap_err("No such file")?.display()
            );
            let command = command.to_boxroot(cr);
            eval(cr, &command);
        }
        Ok(())
    })
}

ocaml! {
    fn eval(command: String);
}

/// Call an OCaml function by name.
///
/// # Safety
/// All arguments must be of the correct type for the OCaml function.
#[macro_export]
macro_rules! dyn_call {
    ($name: literal) => {
        $crate::caml_dyn_call($name, [])
    };
    ($name: literal, $($x:expr),*) => {
        $crate::caml_dyn_call($name, [$($x.into()),*])
    };
}
