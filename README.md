
# Tweld
(you can read it as **tiny-weld**, **token-weld**, or just **tweld**, I am just happy to be here)    
   
Tweld is a procedural macro toolkit and naming DSL for Rust. It allows you to dynamically generate, modify, and compose identifiers directly within your Rust code using a clean, safe, and intuitive `@[]` syntax (hopefully). Because the parsing happens safely inside the macro, any syntax errors in your modifiers will point exactly to the broken line in your IDE (also hopefully).   

```rust
weld!("## @[(the idea | title)]");
```
The name comes from idea of fusing tokens, to help when writing macros, or macros for your macros (which was my initial case).

## The `@[]` "interpolator"
Anything inside the `@[]` "interpolator" will be fused together. You can use the `@[]` syntax inside structs, functions, trait implementations, or anywhere an identifier is expected, as well as in the content of string literal.

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
    pub struct @[Users | singular | pascal] {
        pub id: i64,
    }

    // 2. Inline string replacement and casing
    impl @[Users | singular | PascalCase] {
        
        // Generates: pub fn get_user_profile_by_id(id: i64)
        pub fn @[((get_ UserProfiles) | replace{'s', ''}  snakecase) _by_id](id: i64) {
            println!("Fetching @[Users | singular | lower] {}...", id);
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
Casing style modifiers make use of the crate [heck](https://crates.io/crates/heck).
- **PascalCase**: `pascal` , `pascalcase`, `uppercamelcase`
- **camelCase**:`lowercamelcase`,  `camelcase`,  `camel`
- **snake_case**`snakecase`,  `snake`,  `snekcase`,  `snek`
- **Title Case**: `titlecase`,  `title`
- **kebab-case**: `kebabcase`,  `kebab`
- **Train-Case**: `traincase`,  `train
- **SHOUTY-KEBAB-CASE**:`shoutykebabcase`,  `shoutykebab`  
- **SHOUTY_SNAKE_CASE**: `shoutysnakecase`,  `shoutysnake`,  `shoutysnekcase`,  `shoutysnek`

> **Limitations**
> 
> Given the nature of the syntax, when applied to tokens (as in the body or signature of a function, etc), some of these modifiers will behave a bit different.
> - `kebabcase`, `shoutykebabcase`, and `traincase` won't work,
> - `titlecase` will behave like `PascalCase`
> 
> When applied to string literals, they will all work as intended

### Advanced modifiers:
This crate comes with some modifiers that offer more complex operations that can be pretty helpful, given the right context

---

#### `replace`
 It replaces all non-overlapping occurrences of a `pattern`, with a `replacement` string.
```rust
// will render const a_small_ident = "";
weld!(const @[a long ident | replace{"long","small"}|snek] = "";);
``` 
---
#### `substr`,  `substring`
This modifier returns the part of this string from the start index up to and excluding the end index, or to the end of the string if no end index is supplied. Both indexes are optional.
```rust
// will render const a_long_ident = "";
weld!(const @[(a long identifier)| substr{,9}|snek] = "";);
```  
---
#### `reverse`, `rev`
Reverses the identifier, or literal.
```rust
// will render const tnedi_gnol_a = "";
weld!(const @[(a long identifier)| reverse] = "";);

// will render const nolem_on_nomel_on = "";
weld!(const @[(no lemon no melon)| reverse|snek] = "";);
```  
---
#### `repeat`, `rep`, `times`
Creates a new value by repeating it `n` times.
```rust
// will render const rawhide = "rolling' ,rolling' ,rolling' ";
weld!(const rawhide = @[",rolling' "| times | substr{1}];);
```  
---
#### `split`
---
#### `join`
---
#### `padstart`, `padleft`, `padl`
Padstart pads the value with a given string (repeated and/or truncated, if needed) so that the resulting string has a given length. The padding is applied from the start of this string.
---
#### `padend`, `padright`, `padr`
Padstart pads the value with a given string (repeated and/or truncated, if needed) so that the resulting string has a given length. The padding is applied from the end of this string.
---
#### `slice`
Extracts a substring from a string using start and end positions. Both parameters are optional and support negative indexing, where negative values count backwards from the end of the string. When no arguments are provided, returns the full string.---
#### `splice`
Modifies a string in place by removing a portion defined by start and end positions, optionally replacing it with new content. Returns either the removed portion or the modified string depending on the variant used. Both positions are optional and support negative indexing, where negative values count backwards from the end of the string.
---
#### `spliceout`, `splice_out`
---
#### `spliceinto`, `splice_into`