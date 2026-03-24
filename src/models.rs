#[derive(Debug)]
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
}

#[derive(Debug)]
pub enum TokenPart {
    Literal(String),    
    Plain(String),  
    Modified(Box<TokenPart>, Vec<Modifier>),
}

pub enum StringParserState {
    Idle,
    InsideBrackets,
    InsideGroup,
    Modifiers
}

#[derive(Debug)]
pub enum TokenParserState {    
    InsideBrackets,
    InsideGroup,
    Modifiers
}

