
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

### List `[]` and single value `()` groups
Inside the the brackets, you can organize it in groups, and apply specific modifiers to that group.

#### List Group
When creating a group using `[]` and applying mods to it, each modification will be handled as in a collection, instead of a single concatenated value. 
```rust

weld!(
    // will render: const super_duper = "";
	const @[([er sup] | reverse ) _duper] = ""; 
);

```

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

And inside a list group, you can have other groups, either single value or lists.

```rust
weld!(
    /* this will render:
        struct SuperDuper {
            pub id: i64,
        };
    */
	struct @[([([er sup] |reverse )-duper]|camel)| pascal ] {
        pub id: i64,
    }; 
);
```
When modifying a list group, these modifiers will be applied to each item:
- `singular`,
- `plural`,
- casing modifiers    
    - `lowercase`,
    - `uppercase`,
    - `pascalcase`,
    - `camelcase`,
    - `snakecase`,
    - `titlecase`,
    - `kebabcase`,
    - `traincase`,
    - `shoutykebabcase`,
    - `shoutysnakecase`,
- string specific
    - `replace`
    - `substring`
    - `padstart`
    - `padend`

While other modifiers will behave as handling a vector, 
    - `reverse`: reverses the order of items, not the items themselves, 
    - `repeat`: repeats the items N times, 
    - `splice`: replaces the specified range in the vector with the given value (if the value is informed) and either yield the removed items, if used with `out`, or the modified vector if used with `into`,
    - `slice`: slices the vector, returning the the values whithn the range,

> Using `split` in this mode will split all the items that can be separated, adding them to the collection in the same point

> `join` flattens the collection into a single value (a String), placing a given separator between each (if informed).


#### Single Value group
When creating a group using `()` and applying mods to it, each modification will be applied as in a single value. This group concatenates all the values with no separator. 

```rust
weld!(
    // will render: const super_duper = "";
	const @[((er pus) | reverse ) _duper] = ""; 
);

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
- **Train-Case**: `traincase`,  `train`
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


#### `replace`
 It replaces all non-overlapping occurrences of a `pattern`, with a `replacement` string.
```rust
// will render: const a_small_ident = "";
weld!(const @[(a long ident) | replace{"long","small"}|snek] = "";);
``` 
---
#### `substr`,  `substring`
This modifier returns the part of this string from the start index up to and excluding the end index, or to the end of the string if no end index is supplied. Both indexes are optional.
```rust
// will render: const a_long_ident = "";
weld!(const @[(a long identifier)| substr{,9}|snek] = "";);
```  
---
#### `reverse`, `rev`
Reverses the identifier, or literal.
```rust
// will render: const tnedi_gnol_a = "";
weld!(const @[(a long identifier)| reverse] = "";);

// will render: const nolem_on_nomel_on = "";
weld!(const @[(no lemon no melon)| reverse|snek] = "";);
```  
---
#### `repeat`, `rep`, `times`
Creates a new value by repeating it `n` times.
```rust
// will render: const rawhide = "rolling' ,rolling' ,rolling' ";
weld!(const rawhide = @[",rolling' "| times | substr{1}];);
```  
---
#### `split`
This modifier will either break the whole value if applied to a single value group, or break each item in a list, if applied to a list group. This modifier accepts a `char`, a `string` or a number higher than 0.

(The second case will render a different result because it will have all items splitted)

Splitting by a char, or a string:
```rust
// will render: const val = "get, onetwo, 3, 4struct";
weld!( const val = @[(("get-one" two - "3-4" Struct) | split{'-'} | lower | join{", "})]) = ""; 

// will render: const val = "get, one, two, 3, 4, struct";
weld!( const val = @[["get-one" two - "3-4" Struct] | split{"-"} | lower | join{", "}]);                
```

Splitting by an index:
```rust
// will render: const val = "get-te_st-struct";
weld!( const val = @[(("get-" Test - Struct) | split{6} | lower | join{"_"})] );

// will render: const val = "ge,t-,te,st,-,st,ruct";
weld!( const val = @[["get-" Test - Struct] | split{2} | lower | join{","}] );
```
When splitting by index, if the value is bigger than the length, the argument will be ignored
```rust
// will render: const val = "get-,test,-,stru,ct";
weld!( const val = @[["get-" Test - Struct] | split{4} | lower | join{","}] );
```

---
#### `join`

Flattens the list of values into a single value, placing a given separator between each. The separator can be a `string` or a `char`. (If used in a scenario where the previous result is a single value, it wont have any impact).
```rust
// will render: const val = "get-,Test,-,Struct";
weld!( const val = @[["get-" Test - Struct] | join{","}] );
```

---
#### `padstart`, `padleft`, `padl`
Padstart pads the value with a given string (repeated and/or truncated, if needed) so that the resulting string has a given length. The padding is applied from the start of this string.

---
#### `padend`, `padright`, `padr`
Padstart pads the value with a given string (repeated and/or truncated, if needed) so that the resulting string has a given length. The padding is applied from the end of this string.

---
#### `slice`
Extracts a substring from a string using start and end positions. Both parameters are optional and support negative indexing, where negative values count backwards from the end of the string. When no arguments are provided, returns the full string.

---
#### `splice`
Modifies a string in place by removing a portion defined by start and end positions, optionally replacing it with new content. Returns either the removed portion or the modified string depending on the variant used. Both positions are optional and support negative indexing, where negative values count backwards from the end of the string.

#### Returning the altered value (`value`, `val`, `into`)
```rust
// will render: const val = "g";
weld!( const val = @[("get_" Test_Struct)| splice{value, 1}] );
weld!( const val = @[("get_" Test_Struct)| splice{val, 1}] );
weld!( const val = @[("get_" Test_Struct)| splice{into, 1}] );
```
#### Returning the removed value `out`, `removed`, `rm`
```rust
// will render: const val = "et_Test_Struct";
weld!( const val = @[("get_" Test_Struct)| splice{out, 1}] );
weld!( const val = @[("get_" Test_Struct)| splice{removed, 1}] );
weld!( const val = @[("get_" Test_Struct)| splice{rm, 1}] );
```

#### Removing
```rust
// will render: const val = "gTest_Struct";
weld!( const val = @[("get_" Test_Struct)| splice{into, 1, 4}] );

// will render: const val = "et";
weld!( const val = @[("get_" Test_Struct)| splice{out, 1, 4}] );
```
#### Replacing
```rust
// will render: const val = "got_Test_Struct";
weld!( const val = @[("get_" Test_Struct)| splice{value, 1, 4, "ot_"}] );

// will render: const val = "got_Test_Struct";
weld!( const val = @[("get_" Test_Struct)| splice{val,, 4, "got_"}] );

// will render: const val = "got_Test_Struct";
weld!( const val = @[("get_" Test_Struct)| splice{into,, 4, "got_"}] );

// will render: const val = "got_";
weld!( const val = @[("get_" Test_Struct)| splice{value, 1,, "ot_"}] );

// will render: const val = "new";
weld!( const val = @[("get_" Test_Struct)| splice{val,,, "new"}] );
```
#### Using negative start
Will start counting from the end of the string
```rust
// will render: const val = "get_Test_St";
weld!( const val = @[("get_" Test_Struct)| splice{into, -4 }] );

// will render: const val = "get_Test_Stru";
weld!( const val = @[("get_" Test_Struct)| splice{out, -2 }] );
```
#### Using negative end
Will count from the end of the string
```rust
// will render: const val = "get_Test_Stt";
weld!( const val = @[("get_" Test_Struct)| splice{into, -4, -1 }] );

// will render: const val = "get_Test_Strut";
weld!( const val = @[("get_" Test_Struct)| splice{out, -2, -1 }] );

// will render: const val = "get_Test_St<->t";
weld!( const val = @[("get_" Test_Struct)| splice{into, -4, -1, "<->3" }] );
```

---
#### `spliceout`, `splice_out`
Alias for `splice` with `out` argument.
```rust
// will render: const val = "ruc";
weld!( const val = @[("get_" Test_Struct)| splice_out {-4, -1, "<->3" }] );
```

---
#### `spliceinto`, `splice_into`
Alias for `splice` with the `into` argument.
```rust
// will render: const val = "get_Test_St<->t";
weld!( const val = @[("get_" Test_Struct)| splice_into {-4, -1, "<->3" }] );
```