mod dispatcher;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let dispatcher = dispatcher::Dispatcher::new();

    Ok(dispatcher.serve()?)
}
