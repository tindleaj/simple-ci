use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use std::net::{Shutdown, TcpStream};

#[derive(Serialize, Deserialize, Debug)]
enum ActionType {
    Status,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    Status,
    Dispatch { commit_id: String },
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum DispatcherResponse {
    Ok,
    ReceivedDispatch(String),
    Err,
}

pub fn communicate(
    host: &str,
    port: i32,
    request: Request,
) -> Result<DispatcherResponse, std::io::Error> {
    let mut stream = TcpStream::connect(&format!("{}:{}", host, port.to_string()))?;

    let mut buf = String::new();

    stream.write_all(serde_json::to_string(&request)?.as_bytes())?;
    stream.flush()?;
    stream.shutdown(Shutdown::Write)?;

    stream.read_to_string(&mut buf)?;

    println!("{}", buf);

    let dispatcher_response = serde_json::from_str(&buf)?;

    Ok(dispatcher_response)

    // the stream is closed after this
}
