#[derive(Debug, Clone)]
pub struct MyLogger {
    pub content: String,
}

impl MyLogger {
    pub fn new() -> Self {
        Self {
            content: String::new(),
        }
    }

    pub fn log(&mut self, msg: &str) {
        self.content = msg.into();

        /*
                self.content.push_str(msg);
                self.content.push('\n');
        */
    }
}
