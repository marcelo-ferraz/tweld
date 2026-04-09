# Tweld

[![Crates.io](https://img.shields.io/crates/v/tweld.svg)](https://crates.io/crates/tweld)
[![Docs.rs](https://docs.rs/tweld/badge.svg)](https://docs.rs/tweld)
[![License](https://img.shields.io/crates/l/tweld.svg)](https://choosealicense.com/licenses/)
[![CI](https://github.com/marcelo-ferraz/tweld/actions/workflows/ci.yml/badge.svg)](https://github.com/marcelo-ferraz/tweld/actions/workflows/ci.yml)

> *You can read it as tiny-weld, token-weld, or just tweld. The important thing is that it compiles.*
 
Tweld is a procedural macro toolkit and naming DSL for Rust. It lets you dynamically generate, modify, and compose identifiers directly in your source code using a clean `@[]` syntax — because sometimes the identifier you need doesn't quite exist yet, and writing a full proc-macro just to rename a function feels like bringing a freight train to post a letter.
One can only hope the syntax is clean and intuitive enough.

```rust
weld!("## @[(the idea | title)]");
```
 
The name comes from the idea of fusing tokens together. It started as a tool for writing macros for macros (which sounds recursive, because it is), and then grew somewhat beyond its original remit.

---
 
## Installation
 
Add it to your `Cargo.toml`:
 
```toml
[dependencies]
tweld = "0.1.0-rc.2"  # check crates.io for the latest
```
 
Or, if you prefer:
 
```sh
cargo add tweld
```
 
Then bring it into scope:
 
```rust
use tweld::weld;
```
 
---

## The Core Idea
 
Everything in tweld revolves around one macro — `weld!` — and one syntax: `@[...]`, I fondly call it the interpolator, but the final name is pending.
 
Anything inside `@[...]` gets transformed, *welded* and fused (not literally — the implications would be unwieldy), and emitted as either an identifier or the content of a string literal. A chain of modifiers separated by `|` transforms the value step by step.
 
```rust
weld!(
    fn @[(get user profiles) | snek]() { ... }
    // renders: fn get_user_profiles() { ... }
);
```
 
The modifiers don't need to produce something sensible at every intermediate step. They just need to produce something valid by the end. What happens in between is your own affair.
 
---

 
## Groups
 
Inside `@[...]`, you can organise tokens into named groups and apply modifiers to them. There are two kinds.
 
### Single-value groups `(...)`
 
Tokens inside `()` are concatenated into a single value before any modifiers are applied. Think of it as: *everything in here is one thing*.
 
```rust
weld!(
    const @[(_super Duper) | snek] = "";
    // renders: const super_duper = "";
);
```
 
### List groups `[...]`
 
Tokens inside `[]` are kept as a *collection*. Modifiers that work on individual values (casing, replace, trim, etc.) are applied to each item independently. Modifiers that work on structure (reverse, join, slice, etc.) operate on the collection as a whole.
 
```rust
weld!(
    const @[([er sup] | reverse) _duper] = "";
    // renders: const super_duper = "";
    // ('reverse' flips the order of items in the list, not the characters within them)
);
```
 
Groups can be nested, and modifiers chain naturally across levels:
 
```rust
weld!(
    struct @[([([er sup] | reverse) -duper] | camel) | pascal] {
        pub id: i64,
    };
    // renders: struct SuperDuper { ... }
);
```
 
This is a contrived example — but the point is that you can compose arbitrarily, and as long as the final result is a valid Rust identifier, the compiler will be perfectly happy and won't ask any questions.
 


## Raw Indentifiers

Raw identifiers are handled automatically. If a token is a reserved word, you can pass it in using Rust's `r#` prefix — `r#loop`, `r#type`, and so on — and `weld!` will accept it without complaint. In the other direction, if the result of a modifier chain happens to land on a reserved word, the `r#` prefix will be added for you. If you passed a raw identifier in but the transformations produced something that no longer needs it, the prefix is quietly removed. You shouldn't have to think about it either way.

 
 
## Modifiers
 
Modifiers are chained with `|` inside a group. Each one receives the output of the previous step.
 
### Casing
 
Casing modifiers use the [`heck`](https://docs.rs/heck) crate under the hood.
 
| Modifier          | Aliases                     | Example output |
|-------------------|-----------------------------|----------------|
| `lowercase`       | `lower`                     | `hello_world`  |
| `uppercase`       | `upper`                     | `HELLO_WORLD`  |
| `pascalcase`      | `pascal`, `uppercamelcase`  | `HelloWorld`   |
| `camelcase`       | `camel`, `lowercamelcase`   | `helloWorld`   |
| `snakecase`       | `snek`, `snake`, `snekcase` | `hello_world`  |
| `titlecase`       | `title`                     | `Hello World`  |
| `kebabcase`       | `kebab`                     | `hello-world`  |
| `traincase`       | `train`                     | `Hello-World`  |
| `shoutykebabcase` | `shoutykebab`               | `HELLO-WORLD`  |
| `shoutysnakecase` | `shoutysnake`, `shoutysnek` | `HELLO_WORLD`  |
 
> **A note on tokens vs strings:** When applied to identifiers (function names, struct names, etc.), `kebabcase`, `traincase`, and `shoutykebabcase` won't work — hyphenated identifiers aren't valid Rust. `titlecase` will behave like `pascalcase` in that context. When applied to string literals, all of them work as intended.
 
### `singular` / `plural`
 
`singular` strips a trailing `s`; `plural` adds one. No linguistic analysis is happening here — it's string manipulation wearing a vocabulary waistcoat.
 
```rust
weld!(
    pub struct @[Users | singular | pascal] { pub id: i64 }
    // renders: pub struct User { ... }
);
```
 
---
 
### `replace{pattern, replacement}`
 
Replaces all non-overlapping occurrences of a pattern with a replacement string.
 
```rust
weld!(const @[(a long ident) | replace{"long", "small"} | snek] = "";);
// renders: const a_small_ident = "";
```
 
---
 
### `substr{start?, end?}` / `substring`
 
Returns the substring from `start` up to (not including) `end`. Both are optional. Indexes are zero-based.
 
```rust
weld!(const @[(a long identifier) | substr{, 9} | snek] = "";);
// renders: const a_long_ident = "";
```
 
---
 
### `reverse` / `rev`
 
On a single value: reverses the characters.
On a list group: reverses the order of items (not the characters within them).
 
```rust
weld!(const @[(no lemon no melon) | reverse | snek] = "";);
// renders: const nolem_on_nomel_on = "";
```
 
---
 
### `repeat{n}` / `rep` / `times`
 
Creates a new value by repeating the current value `n` times.
 
```rust
weld!(const rawhide = @[",rolling' " | times{3} | substr{1}]);
// renders: const rawhide = "rolling' ,rolling' ,rolling' ";
```
 
---
 
### `split{separator}`
 
Splits the value by a character, string, or index (any integer > 0).
 
The behaviour differs between group types:
- In a **single-value group** `(...)`: splits the concatenated value into pieces.
- In a **list group** `[...]`: splits each item individually, adding the results back into the collection at that position.
 
**Splitting by character or string:**
 
```rust
// Single-value group: splits on '-', lowercases each part, joins with ", "
weld!(const val = @[(("get-one" two - "3-4" Struct) | split{'-'} | lower | join{", "})]);
// renders: const val = "get, onetwo, 3, 4struct";
 
// List group: each item is split individually
weld!(const val = @[["get-one" two - "3-4" Struct] | split{"-"} | lower | join{", "}]);
// renders: const val = "get, one, two, 3, 4, struct";
```
 
**Splitting by index:**
 
Splits the value every N characters. If the index is larger than the value's length, the argument is ignored.
 
```rust
weld!(const val = @[(("get-" Test - Struct) | split{6} | lower | join{"_"})]);
// renders: const val = "get-te_st-struct";
 
weld!(const val = @[["get-" Test - Struct] | split{2} | lower | join{","}]);
// renders: const val = "ge,t-,te,st,-,st,ruct";
```
 
---
 
### `join{separator?}`
 
Flattens a list into a single value, with an optional separator between items. If the current value is already a single value, it passes through unchanged.
 
```rust
weld!(const val = @[["get-" Test - Struct] | join{","}]);
// renders: const val = "get-,Test,-,Struct";
```
 
---
 
### `padstart{length, pad}` / `padleft` / `padl`
 
Pads from the **start** of the value until it reaches `length` characters. The pad string is repeated and/or truncated as needed. If the value is already at or beyond `length`, it's returned unchanged.
 
```rust
weld!(const val = @[("get-" Test-Struct) | padleft{20, "-"}]);
// renders: const val = "-----get-Test-Struct"
//          (total length 20, padded with '-' on the left)
```
 
---
 
### `padend{length, pad}` / `padright` / `padr`
 
Same as `padstart`, but pads from the **end**.
 
```rust
weld!(const val = @[("get-" Test-Struct) | padright{20, "-"}]);
// renders: const val = "get-Test-Struct-----"
```
 
---
 
### `slice{start?, end?}`
 
Extracts a portion of the string. Both positions are optional and support **negative indexing** (counting backwards from the end). If `start` is greater than `end`, returns an empty value.
 
```rust
weld!(const val = @["get_" Test_Struct | slice{5}]);
// renders: const val = "get_Struct"
 
weld!(const val = @[("_get_" Test_Struct) | slice{1, -4}]);
// renders: const val = "get_Test_St"
 
weld!(const val = @[("_get_" Test_Struct) | slice{-6, -4}]);
// renders: const val = "St"
 
weld!(const val = @["get_" Test_Struct | slice{-4, -6}]);
// renders: const val = ""  (start > end)
```
 
---
 
### `splice{mode?, start?, end?, replacement?}`
 
The most involved modifier. Removes a range from the value, optionally replaces it with new content, and returns either the modified value or the removed portion — depending on the mode.
 
**Modes:**
 
| Mode keywords          | Returns                                               |
|------------------------|-------------------------------------------------------|
| `into`, `val`, `value` | The modified string (with the range removed/replaced) |
| `out`, `removed`, `rm` | The removed portion                                   |
 
**Basic removal:**
 
```rust
// Remove from position 1 onwards → return the result
weld!(const val = @[("get_" Test_Struct) | splice{into, 1}]);
// renders: const val = "g"
 
// Remove from position 1 onwards → return what was removed
weld!(const val = @[("get_" Test_Struct) | splice{out, 1}]);
// renders: const val = "et_Test_Struct"
```
 
**Removing a range:**
 
```rust
weld!(const val = @[("get_" Test_Struct) | splice{into, 1, 4}]);
// renders: const val = "gTest_Struct"
 
weld!(const val = @[("get_" Test_Struct) | splice{out, 1, 4}]);
// renders: const val = "et_"
```
 
**Replacing a range:**
 
```rust
weld!(const val = @[("get_" Test_Struct) | splice{value, 1, 4, "ot_"}]);
// renders: const val = "got_Test_Struct"
 
// Omit start to replace from the beginning
weld!(const val = @[("get_" Test_Struct) | splice{val, , 4, "got_"}]);
// renders: const val = "got_Test_Struct"
 
// Omit end to replace to the end of the string
weld!(const val = @[("get_" Test_Struct) | splice{value, 1, , "ot_"}]);
// renders: const val = "got_"
 
// Omit both to replace the entire value
weld!(const val = @[("get_" Test_Struct) | splice{val, , , "new"}]);
// renders: const val = "new"
```
 
**Negative positions** count from the end:
 
```rust
weld!(const val = @[("get_" Test_Struct) | splice{into, -4}]);
// renders: const val = "get_Test_St"
 
weld!(const val = @[("get_" Test_Struct) | splice{into, -4, -1, "<->"}]);
// renders: const val = "get_Test_St<->t"
```
 
**Aliases:** `splice_into` and `splice_out` are shorthand for `splice{into, ...}` and `splice{out, ...}`:
 
```rust
weld!(const val = @[("get_" Test_Struct) | splice_out{-4, -1}]);
// renders: const val = "ruc"
 
weld!(const val = @[("get_" Test_Struct) | splice_into{-4, -1, "<->"}]);
// renders: const val = "get_Test_St<->t"
```
 
---
 
## A More Complete Example
 
Here's what the modifier chain looks like when used for something you might actually want to do:
 
```rust
use tweld::weld;
 
weld! {
    pub struct @[Users | singular | pascal] {
        pub id: i64,
    }
 
    impl @[Users | singular | pascal] {
 
        // Generates: pub fn get_user_profile_by_id(id: i64)
        pub fn @[((get_ UserProfiles) | replace{"s", ""} | snek) _by_id](id: i64) {
            println!("Fetching @[Users | singular | lower] {}...", id);
        }
    }
}
```
 
The modifiers don't need to be tidy on the inside. They just need to produce something valid on the outside — which is, when you think about it, a reasonable standard to hold most things to.
 
---
 
## Status
 
Tweld is currently in **alpha (RC3)**. The feature set for `1.0` is complete and testing is still an ongoing endeavor, some will say is a mess, I call it home. The API may still shift before stabilisation.
 
Bug reports, feature requests, and strong opinions about identifier naming are all welcome.
 
---
 
## License
 
Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.
 