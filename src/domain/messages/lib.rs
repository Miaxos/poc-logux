pub trait LoguxEvent {
    fn encode(&self) -> String;
}
