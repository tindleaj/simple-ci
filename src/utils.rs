pub enum DispatcherRequest {
    Status,
    Dispatch(String),
}

#[derive(PartialEq, Eq)]
pub enum DispatcherResponse {
    Ok,
    Err,
}

pub fn communicate(host: &str, port: i32, request: DispatcherRequest) -> DispatcherResponse {
    unimplemented!();
}
