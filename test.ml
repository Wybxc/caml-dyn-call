let add (x: int) (y: int) = x + y in
Callback.register "add" add;;

let sub (x: int) (y: int) = x - y in
Callback.register "sub" sub;;

let parse_int (s: string) = int_of_string s in
Callback.register "parse_int" parse_int;;

let print_int (i: int) = string_of_int i in
Callback.register "print_int" print_int;;


#load "libdyn.cma";;
#load "unix.cma";;

open Stdlib;;

print_endline "Server starting";;

let sockaddr = "/tmp/caml_dyn_call.sock";;
Unix.unlink sockaddr;;
let sock = Unix.socket Unix.PF_UNIX Unix.SOCK_STREAM 0;;
Unix.bind sock (Unix.ADDR_UNIX sockaddr);;
Unix.listen sock 1;;

print_endline "Listening";;

let (client, _) = Unix.accept sock;;
let in_channel = Unix.in_channel_of_descr client;;
let out_channel = Unix.out_channel_of_descr client;;

print_endline "Connected";;

try
  while true do
    let cmd = input_line in_channel |> String.trim in
    if String.equal cmd "" then raise End_of_file;
    let res = Dyn.dyn_call cmd in
    output_string out_channel res;
    output_char out_channel '\n';
    flush out_channel
  done
with End_of_file -> 
  print_endline "Connection closed";;

Unix.close sock;;
