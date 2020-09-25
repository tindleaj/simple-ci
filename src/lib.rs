//! cimple - Simple Continuous Integration
//!
//! cimple includes three binaries that work together to create the CI system:
//!
//! - [dispatcher](../dispatcher/index.html): delegates testing tasks and returns results
//! - [observer](../observer/index.html): monitors a repository and notifies the dispatcher when a new commit is seen
//! - [runner](../runner/index.html): responsible for running tests against a given commit ID and returning the results
//!

use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use std::net::{Shutdown, TcpStream};

pub mod dispatcher;
pub mod observer;

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

    println!("Sending request: {:#?}", request);

    stream.write_all(serde_json::to_string(&request)?.as_bytes())?;
    stream.flush()?;
    stream.shutdown(Shutdown::Write)?;

    stream.read_to_string(&mut buf)?;

    let dispatcher_response = serde_json::from_str(&buf)?;

    println!("Recieved response: {:?}", dispatcher_response);

    Ok(dispatcher_response)

    // the stream is closed after this
}
