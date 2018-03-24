pub trait Input {
    fn on_up(&mut self) -> bool {
        true
    }
    fn on_down(&mut self) -> bool {
        true
    }
    fn on_select(&mut self) -> bool {
        true
    }
}
