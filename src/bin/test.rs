use std::{
    io::{BufRead, BufReader, LineWriter, Write},
    os::unix::net::UnixStream,
};

use serde::{Deserialize, Serialize};

const SOKCET_PATH: &str = "/tmp/caml_dyn_call.sock";

type Key = slotmap::DefaultKey;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum Val {
    Token(Key),
    String(String),
}

impl From<&str> for Val {
    fn from(s: &str) -> Self {
        Val::String(s.to_string())
    }
}

impl From<Key> for Val {
    fn from(key: Key) -> Self {
        Val::Token(key)
    }
}

macro_rules! args {
    ($($x:expr),*) => {
        vec![$(Val::from($x)),*]
    };
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "op")]
#[serde(rename_all = "snake_case")]
pub enum Command {
    Call { name: String, args: Vec<Val> },
    Str { key: Key },
    Dis { key: Key },
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "st")]
#[serde(rename_all = "snake_case")]
pub enum Response {
    Ok { val: Val },
    Err { msg: String },
}

pub struct DynCall {
    writer: LineWriter<UnixStream>,
    reader: BufReader<UnixStream>,
}

impl DynCall {
    pub fn new(path: &str) -> std::io::Result<Self> {
        let stream = UnixStream::connect(path)?;
        let writer = LineWriter::new(stream.try_clone()?);
        let reader = BufReader::new(stream);
        Ok(Self { writer, reader })
    }

    fn run_command(&mut self, command: Command) -> Result<Response, Box<dyn std::error::Error>> {
        serde_json::to_writer(&mut self.writer, &command)?;
        self.writer.write_all(b"\n")?;

        let mut response = String::new();
        while response.is_empty() {
            self.reader.read_line(&mut response)?;
            response.truncate(response.trim_end().len());
        }
        dbg!(&response);
        let response: Response = serde_json::from_str(&response)?;
        Ok(response)
    }

    pub fn dyn_call(
        &mut self,
        name: impl Into<String>,
        args: Vec<Val>,
    ) -> Result<Val, Box<dyn std::error::Error>> {
        let name = name.into();
        let response: Response = self.run_command(Command::Call { name, args })?;

        match response {
            Response::Ok { val } => Ok(val),
            Response::Err { msg } => Err(msg.into()),
        }
    }

    pub fn get_str(&mut self, val: Val) -> Result<String, Box<dyn std::error::Error>> {
        match val {
            Val::String(s) => Ok(s),
            Val::Token(key) => {
                let response: Response = self.run_command(Command::Str { key })?;

                match response {
                    Response::Ok { val } => match val {
                        Val::String(s) => Ok(s),
                        _ => Err("Invalid response".into()),
                    },
                    Response::Err { msg } => Err(msg.into()),
                }
            }
        }
    }

    pub fn remove(&mut self, key: Key) -> Result<Key, Box<dyn std::error::Error>> {
        let response: Response = self.run_command(Command::Dis { key })?;

        match response {
            Response::Ok { val } => match val {
                Val::Token(key) => Ok(key),
                _ => Err("Invalid response".into()),
            },
            Response::Err { msg } => Err(msg.into()),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut dyn_call = DynCall::new(SOKCET_PATH)?;

    let a = dyn_call.dyn_call("parse_int", args!("123"))?;
    let b = dyn_call.dyn_call("parse_int", args!("456"))?;
    let c = dyn_call.dyn_call("add", args!(a.clone(), b.clone()))?;
    let d = dyn_call.dyn_call("print_int", args!(c.clone()))?;
    let d = dyn_call.get_str(d)?;

    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("c: {:?}", c);
    println!("d: {:?}", d);

    Ok(())
}
