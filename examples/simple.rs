use caml_dyn_call::*;
use eyre::Result;

fn main() -> Result<()> {
    init(std::path::Path::new("examples/simple.ml"))?;

    let a = dyn_call!("parse_int", "123")?;
    let b = dyn_call!("parse_int", "456")?;
    let c = dyn_call!("add", &a, &b)?;
    let d = dyn_call!("print_int", &c)?.get_str()?;

    println!("a: {}", a);
    println!("b: {}", b);
    println!("c: {}", c);
    println!("d: {}", d);

    Ok(())
}
