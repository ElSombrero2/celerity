use serde::Serialize;


#[derive(Serialize, Clone)]
pub struct CommandsOutput{
    pub output: String,
}