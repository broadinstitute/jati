pub trait RunState {
    fn request_stop(&mut self);
    fn stop_requested(&self) -> bool;
}