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

pub enum TokenPart {
    Plain(String),    
    // if it does have a split function I could change this to Vec<String>
    Modified(String, Vec<Modifier>),
}