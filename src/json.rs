use ocaml_interop::OCamlRuntime;
use serde::{Deserialize, Serialize};
use slotmap::Key as SlotMapKey;

use crate::{
    call::{dyn_call, Val},
    store::{remove_value, Key},
};

#[derive(Serialize, Deserialize)]
#[serde(tag = "op")]
#[serde(rename_all = "snake_case")]
pub enum Command {
    Call { name: String, args: Vec<Val> },
    Str { key: Key },
    Dis { key: Key },
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "st")]
#[serde(rename_all = "snake_case")]
pub enum Response {
    Ok { val: Val },
    Err { msg: String },
}

pub fn dyn_call_json(cr: &mut OCamlRuntime, command: &str) -> Response {
    let mut inner = move || {
        let command: Command =
            serde_json::from_str(command).map_err(|e| format!("Invalid JSON: {}", e))?;

        match command {
            Command::Call { name, args } => {
                let key = dyn_call(cr, &name, &args)?;
                Ok(Val::Token(key))
            }
            Command::Str { key } => {
                let value = crate::store::get_value(key)
                    .ok_or_else(|| format!("Invalid key {}", key.data().as_ffi()))?;
                Ok(Val::String(value.to_rust(cr)))
            }
            Command::Dis { key } => {
                remove_value(key);
                Ok(Val::Token(key))
            }
        }
    };
    match inner() {
        Ok(val) => Response::Ok { val },
        Err(msg) => Response::Err { msg },
    }
}
