use serde::Serialize;

fn skip(attr: &bool) -> bool { return !attr; }

#[derive(Serialize,Debug)]
pub struct Chat {
    pub text: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub color: String,
    #[serde(skip_serializing_if = "skip")]
    pub bold: bool,
    #[serde(skip_serializing_if = "skip")]
    pub italic: bool,
    #[serde(skip_serializing_if = "skip")]
    pub underlined: bool,
    #[serde(skip_serializing_if = "skip")]
    pub strikethrough: bool,
    #[serde(skip_serializing_if = "skip")]
    pub obfuscated: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub extra: Vec<Self>,
}

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

    pub fn finish(&self) -> Chat {
        let extra: Vec<Chat> = self.extra.iter().map(|ex| ex.finish()).collect();
        let message = Chat {
            text: self.text.clone(),
            color: self.color.clone(),
            bold: self.is_style(0x01),
            italic: self.is_style(0x02),
            underlined: self.is_style(0x04),
            strikethrough: self.is_style(0x08),
            obfuscated: self.is_style(0x10),
            extra,
        };

        message
    }
}

