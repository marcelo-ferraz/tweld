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
}

#[derive(Debug)]
pub enum TokenPart {
    Literal(String),    
    Plain(String),    
    Modified(String, Vec<Modifier>),
}

pub enum StringParserState {
    Idle,
    InsideBrackets,
    InsideGroup,
    Modifiers
}

