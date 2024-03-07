use caml_dyn_call::*;
use eyre::Result;

fn main() -> Result<()> {
    init(Some(std::path::Path::new("examples/stream.ml")))?;

    let stream = unsafe { dyn_call!("stream_of_string", "Hello, World!") }?;
    loop {
        let c = unsafe { dyn_call!("stream_empty", &stream) }?;
        match unsafe { c.get_str() }?.as_str() {
            "true" => break,
            "false" => {
                let c = unsafe { dyn_call!("stream_next", &stream)?.get_str() }?.parse()?;
                let c = char::from_u32(c).unwrap();
                print!("{}", c);
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}
