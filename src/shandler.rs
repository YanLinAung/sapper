use request::Request;
use response::Response;
use sapp::Result;
use std::any::Any;

// all handler function in each module should fit this Handler trait
pub trait SHandler: Send + Sync + Any {
    fn handle(&self, &mut Request) -> Result<Response>;
}


impl<F> SHandler for F
where F: Send + Sync + Any + Fn(&mut Request) -> Result<Response> {
    fn handle(&self, req: &mut Request) -> Result<Response> {
        (*self)(req)
    }
}


impl SHandler for Box<SHandler> {
    fn handle(&self, req: &mut Request) -> Result<Response> {
        (**self).handle(req)
    }
}
