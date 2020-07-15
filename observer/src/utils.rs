pub enum DispatcherRequest {
    Status,
    Dispatch(String),
}

#[derive(PartialEq, Eq, Debug)]
pub enum DispatcherResponse {
    Ok,
    ReceivedDispatch(String),
    Err,
}

pub fn communicate(host: &str, port: i32, request: DispatcherRequest) -> DispatcherResponse {
    // TODO: make this do  something
    match request {
        DispatcherRequest::Dispatch(id) => DispatcherResponse::ReceivedDispatch(id),
        DispatcherRequest::Status => DispatcherResponse::Ok,
    }
}
