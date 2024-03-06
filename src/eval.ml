let _ = Toploop.initialize_toplevel_env ()

let eval code =
  let as_buf = Lexing.from_string code in
  let parsed = !Toploop.parse_toplevel_phrase as_buf in
  ignore (Toploop.execute_phrase true Format.std_formatter parsed)

let _ = Topdirs.dir_directory "." (* To activate Topdirs*)

let _ = Callback.register "eval" eval
