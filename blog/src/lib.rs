trait State {}

trait Draft {}

impl State for Draft {}

pub struct Post {
    content: String,
    state: Option<Box<dyn State>>,
}

impl Post {
    pub fn new() -> Post {
        Post {
            content: String::new(),
            state: Some(Box::new(Draft {}))
        }
    }
}