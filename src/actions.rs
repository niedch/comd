#[derive(Debug)]
pub enum StreamType {
    StreamResult,
    StreamEnd,
}

#[derive(Debug)]
pub struct StreamResult {
    pub action_type: StreamType,
    pub result: String,
}

#[derive(Debug)]
pub struct StartRequest {
    pub prompt: String,
}
