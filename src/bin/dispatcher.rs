use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let dispatcher = cimple::dispatcher::Dispatcher::new();

    Ok(dispatcher.serve()?)
}