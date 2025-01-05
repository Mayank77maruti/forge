mod ask;
mod error;
mod fs;
mod outline;
mod shell;
mod think;
mod tool_call_service;
mod tool_service;
pub use tool_service::*;

trait Description {
    fn description() -> &'static str;
}

pub struct Service;
