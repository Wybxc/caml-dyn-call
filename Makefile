libdyn.cma: dyn.cmo dllcaml_dyn_call.so
	ocamlc -a -o libdyn.cma dyn.cmo -dllib -lcaml_dyn_call

dllcaml_dyn_call.so: src/**.rs
	cargo build --release
	cp target/release/libcaml_dyn_call.so dllcaml_dyn_call.so

dyn.cmo: dyn.ml
	ocamlc -c dyn.ml

.PHONY: clean
clean:
	rm -f ./*.a ./*.cma ./*.cmo ./*.so ./*.cmi
	cargo clean