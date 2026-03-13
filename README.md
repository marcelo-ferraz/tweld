
# Tweld
(you can read it as **tiny-weld**, **token-weld**, or just **tweld**, I am just happy to be here)    
   
Tweld is a procedural macro toolkit and naming DSL for Rust. It allows you to dynamically generate, modify, and compose identifiers directly within your Rust code using a clean, safe, and intuitive `@[]` syntax (hopefully).    

```rust
weld!("## @[(the idea | titlecase)]");
```
The name comes from idea of fusing tokens, to help when writing macros, or macros for your macros (which was my initial case).

## The `@[]` "interpolator"
Anything inside the `@[]` "interpolator" will be fused together. 

>`"@[one - two]"`  will render `"one-two"`

It can be used with tokens to create identifiers, or inside a string literal.
```rust
weld!(
	fn @[weld_(_these toKens ById|snek|substr{1,})]() -> String {
	  "@This will render a function name (@[weld_(_these toKens ById|snek|substr{1,})]) with all these @[(tokens fused | title | lower )] together!]".to_string()
	}
);
```

Inside the the brackets, you can organize it in groups, and apply specific modifiers to that group.


```rust
use weld::render_names;

weld! {
    // 1. Basic interpolation
    pub struct @[(Users | singular | pascal)] {
        pub id: i64,
    }

    // 2. Inline string replacement and casing
    impl @[(Users | singular | PascalCase)] {
        
        // Generates: pub fn get_user_profile_by_id(id: i64)
        pub fn @[get_ (UserProfiles | replace{'s', ''}  snakecase) _by_id](id: i64) {
            println!("Fetching @[(Users | singular | lower)] {}...", id);
        }
    }
}
```
## Modifiers
While inside a group, you can apply a chain of modifiers, where each one will perform an operation on the previous result.

### Simple modifiers:
These are self explanatory, being `singular` and `plural` just the removal of the letter `'s'` from the last word.
- `singular`,
- `plural`,
- `lower` , `lowercase`
- `upper`,  `uppercase`
### Casing style modifiers:
Casing style modifiers use the crate [heck](https://crates.io/crates/heck). 
- PascalCase: `pascal` , `pascalcase`, `uppercamelcase`
- camelCase:`lowercamelcase`,  `camelcase`,  `camel`
- snake_case`snakecase`,  `snake`,  `snekcase`,  `snek`
- SHOUTY_SNAKE_CASE: `shoutysnakecase`,  `shoutysnake`,  `shoutysnekcase`,  `shoutysnek`
- Title Case: `titlecase`,  `title`
- kebab-case: `kebabcase`,  `kebab`
- SHOUTY-KEBAB-CASE:`shoutykebabcase`,  `shoutykebab`  
- Train-Case: `traincase`,  `train`
### Advanced modifiers:

#### `replace`
 
#### `substr`,  `substring`

 