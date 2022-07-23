use serde::Serialize;

fn skip(attr: &bool) -> bool { return !attr; }

#[derive(Serialize,Debug)]
/// Minecraft [chat](https://wiki.vg/Chat) object
pub struct Chat {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
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

/// Utility struct for making Minecraft [chat](https://wiki.vg/Chat) objects
///
/// # Example
///
/// The following snippet will create a Chat struct containing "Hello, World!"
/// "Hello, " is blue
/// "World!" is blue, bold and italic
///
/// ```
/// ChatBuilder::new("Hello, ")
///     .color("blue")
///     .append(
///         ChatBuilder::new("World!")
///             .bold()
///             .italic()
///     )
///     .finish()
/// ```
pub struct ChatBuilder {
    text: String,
    color: Option<String>,
    style: u8,
    extra: Vec<Self>,
}

impl ChatBuilder {
    /// New chat builder, only containing given `text`
    pub fn new(text: &str) -> Self {
        ChatBuilder {
            text: String::from(text),
            color: None,
            style: 0,
            extra: Vec::new(),
        }
    }

    /// Update component color
    ///
    /// `color` must be one of [Minecraft color codes](https://wiki.vg/Chat#Colors)
    ///
    /// # Example
    ///
    /// ```
    /// ChatBuilder::new("I'm blue").color("blue");
    /// ```
    pub fn color(mut self, color: &str) -> Self {
        self.color = Some(String::from(color));
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

    /// Adds an extra component to self
    /// The given component will inherit properties from self
    ///
    /// # Examples
    ///
    /// ```
    /// ChatBuilder::new("parent")
    ///     .bold()
    ///     .append(
    ///         ChatBuilder::new("child").italic()
    ///     )
    /// ```
    ///
    /// Result:
    ///
    /// - parent: bold
    /// - child: bold, italic
    ///
    /// ```
    /// ChatBuilder::new("")
    ///     .append(
    ///         ChatBuilder::new("first child").bold()
    ///     )
    ///     .append(
    ///         ChatBuilder::new("second child").italic()
    ///     )
    /// ```
    ///
    /// Result:
    ///
    /// - first child: bold
    /// - second chuld: italic
    pub fn append(mut self, chat: Self) -> Self {
        self.extra.push(chat);
        self
    }

    /// Adds a single space as an extra to self
    pub fn space(mut self) -> Self {
        self.extra.push(Self::new(" "));
        self
    }

    /// Constructs a new Chat object, ready to be serialized using serde then sent to a client
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

