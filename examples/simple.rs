use std::path::Path;

use caml_dyn_call::*;

macro_rules! args {
    ($($x:expr),*) => {
        vec![$(Val::from($x)),*]
    };
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init(Path::new("examples/simple.ml"))?;

    let a = dyn_call("parse_int", args!("123"))?;
    let b = dyn_call("parse_int", args!("456"))?;
    let c = dyn_call("add", args!(a, b))?;
    let d = dyn_call("print_int", args!(c))?;
    let d = get_str(d)?;

    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("c: {:?}", c);
    println!("d: {:?}", d);

    Ok(())
}
