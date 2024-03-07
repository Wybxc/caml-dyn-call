use std::path::Path;

use caml_dyn_call::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init(Path::new("examples/simple.ml"))?;

    let a = dyn_call!("parse_int", "123")?;
    let b = dyn_call!("parse_int", "456")?;
    let c = dyn_call!("add", a, b)?;
    let d = dyn_call!("print_int", c)?;
    let d = get_str(d)?;

    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("c: {:?}", c);
    println!("d: {:?}", d);

    Ok(())
}
