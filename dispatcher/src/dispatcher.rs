use ci::DispatcherResponse;
use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    Status,
    Dispatch { commit_id: String },
    Register,
    Results,
}

pub struct Dispatcher {
    runners: Vec<String>,
    dead: bool,
    dispatched_commits: Vec<String>,
    pending_commits: Vec<String>,
}

impl Dispatcher {
    pub fn new() -> Self {
        Dispatcher {
            runners: vec!["".into()],
            dead: false,
            dispatched_commits: vec!["".into()],
            pending_commits: vec!["".into()],
        }
    }

    /// Listens on a port for requests from a test runner or an observer.
    /// Takes commit IDs from an observer and dispatches a runner against that commit.
    /// Registers tests runners, and adds failed runs back into a pending queue.
    ///
    /// Periodically pings the runners to determine their state
    pub fn serve(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind("127.0.0.1:8888")?;

        let heartbeat = thread::spawn(|| loop {
            check();
            thread::sleep(std::time::Duration::from_secs(5))
        });

        let redistributor = thread::spawn(|| loop {
            redistribute();
            thread::sleep(std::time::Duration::from_secs(5))
        });

        for stream in listener.incoming() {
            // Handle this error, can some connections just be errors?
            let stream = stream?;

            println!("New client: {}", stream.peer_addr()?);

            let _ = thread::spawn(move || handle(stream));
        }

        heartbeat
            .join()
            .expect("Failed to join 'heartbeat' thread to main thread");
        redistributor
            .join()
            .expect("Failed to join 'redistributor' thread to main thread");

        Ok(())
    }
}

// Impl Drop to handle hearbeat and redistributor joins

/// Handle incoming requests on their own thread
/// Dispatch tests runners and handle their requests and tests results
fn handle(mut stream: TcpStream) {
    // const MESSAGE_SIZE: usize = 1024;

    let mut buf = String::new();

    stream
        .read_to_string(&mut buf)
        .unwrap_or_else(|_| panic!("Problem receiving message: {:?}", buf));

    match serde_json::from_str::<Request>(&buf) {
        Ok(req) => {
            println!("Request: {:?}", req);
            match req {
                Request::Status => {
                    let res = serde_json::to_vec(&DispatcherResponse::Ok)
                        .expect("Problem serializing response to JSON");

                    stream.write_all(&res).expect("Problem writing to stream");
                    stream.flush().expect("Problem flushing stream");
                    stream
                        .shutdown(std::net::Shutdown::Write)
                        .expect("Problem shutting down write stream");
                    println!("Ok");
                }
                Request::Dispatch { commit_id } => {
                    println!("Dispatching job to runner...");

                    // TODO: Actually dispatch a test runner

                    // Tell the observer we did the thing it wanted
                    let res = serde_json::to_vec(&DispatcherResponse::ReceivedDispatch(commit_id))
                        .expect("Problem serializing response to JSON");

                    stream.write_all(&res).expect("Problem writing to stream");
                    stream.flush().expect("Problem flushing stream");
                    stream
                        .shutdown(std::net::Shutdown::Write)
                        .expect("Problem shutting down write stream")
                }
                Request::Register => {}
                Request::Results => {}
            }
        }
        Err(_) => panic!("Problem deserializing message: {:?}", buf),
    }
}

// Periodically ping each registered test runner, if they are unresponsive then remove it from the pool and add
// its commit id back to the pending list of commits
fn check() {
    println!("check")
}

fn redistribute() {
    println!("redistribute")
}
