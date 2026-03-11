use thiserror::Error;

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
    // split
}

#[derive(Debug)]
pub enum TokenPart {
    Literal(String),    
    Plain(String),    
    // if it does have a split function I could change this to Vec<String>
    Modified(String, Vec<Modifier>),
}

#[derive(Error, Debug)]
pub enum StringParserError {
    #[error("Unknown modifier: {0}!")]
    UnknownModifier(String),
    #[error("Brackets were not closed!")]
    OpenBrackets,
    #[error("Group was left open!")]
    OpenGroup,
    #[error("Modifiers section is unfinished!")]
    UnfinishModifiers,
    #[error("The value for '{name:?}' is not a number ('{value}')!")]
    NaNParam{
        name: &'static str, 
        value: String        
    },
}