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

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}