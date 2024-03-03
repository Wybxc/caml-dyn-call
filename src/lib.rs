use ocaml_interop::{ocaml_export, FromOCaml, OCaml, OCamlRef, ToOCaml};

mod call;
mod json;
mod store;

ocaml_export! {
    fn caml_dyn_call(cr, command: OCamlRef<String>) -> OCaml<String> {
        let command = String::from_ocaml(command.to_ocaml(cr));
        let response = json::dyn_call_json(cr, &command);
        let response = serde_json::to_string(&response).expect("Failed to serialize response");
        response.to_ocaml(cr)
    }
}
