use caml_dyn_call::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init(std::path::Path::new("examples/stream.ml"))?;

    let stream = dyn_call!("stream_of_string", "Hello, World!")?;
    loop {
        let c = dyn_call!("stream_empty", stream)?;
        match get_str_dispose(c)?.as_str() {
            "true" => break,
            "false" => {
                let c = dyn_call!("stream_next", stream)?;
                let c: u32 = get_str_dispose(c)?.parse()?;
                let c = char::from_u32(c).unwrap();
                print!("{}", c);
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}
