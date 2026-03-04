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

pub enum TokenPart {
    Plain(String),
    Modified(String, Vec<Modifier>),
}