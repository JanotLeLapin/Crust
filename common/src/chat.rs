use serde_json::{Value,json};

pub struct ChatBuilder {
    text: String,
    color: String,
    style: u8,
    extra: Vec<Self>,
}

impl ChatBuilder {
    pub fn new(text: &str) -> Self {
        ChatBuilder {
            text: String::from(text),
            color: String::from("reset"),
            style: 0,
            extra: Vec::new(),
        }
    }

    pub fn color(mut self, color: String) -> Self {
        self.color = color;
        self
    }

    fn style(mut self, style: u8) -> Self {
        self.style |= style;
        self
    }

    fn is_style(&self, style: u8) -> bool {
        (self.style & style) == style
    }

    pub fn bold(self) -> Self {
        self.style(0x01)
    }

    pub fn italic(self) -> Self {
        self.style(0x02)
    }

    pub fn underlined(self) -> Self {
        self.style(0x04)
    }

    pub fn strikethrough(self) -> Self {
        self.style(0x08)
    }

    pub fn obfuscated(self) -> Self {
        self.style(0x10)
    }

    pub fn add_extra(mut self, chat: Self) -> Self {
        self.extra.push(chat);
        self
    }

    pub fn finish(&self) -> Value {
        let extra: Vec<Value> = self.extra.iter().map(|ex| ex.finish()).collect();
        let mut message = json!({
            "text": self.text,
            "color": self.color,
        });
        let t = Value::Bool(true);
        if self.is_style(0x01) {
            message["bold"] = t.clone();
        }
        if self.is_style(0x02) {
            message["italic"] = t.clone();
        }
        if self.is_style(0x04) {
            message["underlined"] = t.clone();
        }
        if self.is_style(0x08) {
            message["strikethrough"] = t.clone();
        }
        if self.is_style(0x10) {
            message["obfuscated"] = t.clone();
        }
        if extra.len() > 0 {
            message["extra"] = t.clone();
        }

        message
    }
}

