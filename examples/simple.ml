let add (x: int) (y: int) = x + y in
Callback.register "add" add;;

let sub (x: int) (y: int) = x - y in
Callback.register "sub" sub;;

let parse_int (s: string) = int_of_string s in
Callback.register "parse_int" parse_int;;

let print_int (i: int) = string_of_int i in
Callback.register "print_int" print_int;;
