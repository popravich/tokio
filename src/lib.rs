extern crate libc;
extern crate net2;
extern crate futures;
extern crate tokio_core;
extern crate boxfnonce;
#[macro_use] extern crate log;
#[macro_use] extern crate cpython;
#[macro_use] extern crate lazy_static;

use cpython::*;

mod addrinfo;
mod handle;
mod utils;
mod future;
mod event_loop;
mod transport;
mod server;
pub use event_loop::{new_event_loop, spawn_event_loop};


py_module_initializer!(_ext, init_ext, PyInit__ext, |py, m| {
    m.add(py, "__doc__", "Asyncio event loop based on tokio")?;

    init_tokio_module(py, m)?;
    Ok(())
});


pub fn init_tokio_module(py: cpython::Python, m: &cpython::PyModule) -> cpython::PyResult<()> {
    m.add_class::<event_loop::TokioEventLoop>(py)?;
    m.add_class::<future::TokioFuture>(py)?;
    m.add_class::<handle::TokioHandle>(py)?;
    m.add_class::<handle::TokioTimerHandle>(py)?;
    m.add_class::<server::TokioServer>(py)?;

    m.add(py, "spawn_event_loop", py_fn!(py, spawn_event_loop(name: &PyString)))?;
    m.add(py, "new_event_loop", py_fn!(py, new_event_loop()))?;
    Ok(())
}