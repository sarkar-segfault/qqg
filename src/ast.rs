//! defines abstract syntax tree components and parsing functions

/// contains information about the metaline (title + by)
#[derive(Default)]
pub struct Meta {
    pub title: String,
    pub by: String,
}

/// represents an answer directive
#[derive(Default)]
pub struct Answer {
    pub has: Vec<String>,
    pub is: Vec<String>,
    pub show: bool,
}

/// represents different styling features
pub enum StyleKind {
    Fg,
    Bg,
    Br,

    Magenta,
    Yellow,
    Green,
    White,
    Black,
    Blue,
    Cyan,
    Red,

    Underline,
    Strike,
    Italic,
    Invert,
    Hidden,
    Blink,
    Bold,
    Dim,
}

/// represents a style block
#[derive(Default)]
pub struct Style {
    pub show: bool,
    pub styles: Vec<StyleKind>,
}

/// represents a question
#[derive(Default)]
pub struct Question {
    pub text: String,
    pub answer: Answer,
    pub style: Style,
}

/// contains information about the program
#[derive(Default)]
pub struct Program {
    pub meta: Meta,
    pub questions: Vec<Question>,
}
