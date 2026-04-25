#[derive(Debug, Clone)]
pub enum Modifier {
    Singular,
    Plural,
    Lowercase,
    Uppercase,
    PascalCase,
    LowerCamelCase,
    SnakeCase,
    KebabCase,
    ShoutySnakeCase,
    TitleCase,
    ShoutyKebabCase,
    TrainCase,
    Replace(String, String),
    Substr(Option<usize>, Option<usize>),
    Reverse,
    Repeat(usize),
    Split(String),
    SplitAt(usize),
    Join(String),
    PadStart(usize, String),
    PadEnd(usize, String),
    Slice(Option<i32>, Option<i32>),
    Splice(Output, Option<i32>, Option<i32>, Option<String>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Output {
    Value,
    Removed,
}

#[derive(Debug, Clone)]
pub enum WeldToken {
    Plain(String),
    ConcatGroup(Vec<WeldToken>),
    ListGroup(Vec<WeldToken>),
    Modify(Box<WeldToken>, Vec<Modifier>),
}

#[derive(Debug)]
pub enum TokenParserState {
    Root,
    InsideGroup(bool),
    Modifiers,
}

#[derive(Debug, Clone)]
pub enum RenderType {
    StringLiteral,
    Identifier,
}
