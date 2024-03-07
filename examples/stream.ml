let stream_of_string s = Stdlib.Stream.of_string s in
Callback.register "stream_of_string" stream_of_string;;

let stream_next s = Stdlib.Stream.next s |> Char.code |> string_of_int in
Callback.register "stream_next" stream_next;;

let stream_empty s = try Stdlib.Stream.empty s; "true" with Stdlib.Stream.Failure -> "false" in
Callback.register "stream_empty" stream_empty;;
