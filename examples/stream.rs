use caml_dyn_call::*;
use eyre::Result;

fn main() -> Result<()> {
    init(std::path::Path::new("examples/stream.ml"))?;

    let stream = dyn_call!("stream_of_string", "Hello, World!")?;
    loop {
        let c = dyn_call!("stream_empty", &stream)?;
        match c.get_str()?.as_str() {
            "true" => break,
            "false" => {
                let c = dyn_call!("stream_next", &stream)?;
                let c = c.get_str()?.parse()?;
                let c = char::from_u32(c).unwrap();
                print!("{}", c);
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}
