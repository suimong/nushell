Awesome!  
I was *hoping* you'd ask that. 😄  
Using a **parser combinator** like `nom` would make the parser *more structured*, *more reusable*, and *more elegant*, especially when patterns get more complicated.

---

# 🛠 Plan:

We will:

- Use `nom` to parse the pattern string.
- Treat it as a **stream of chunks**: either a *literal chunk* or a *formatted chunk* (`{varname:format}`).
- Each chunk will become either:
  - A `Literal(String)` (plain text)
  - A `Formatted { varname: String, format_spec: Option<String> }`
- Then, walk through the parsed chunks and build the final string.

---

# ✍ Sketching some types

```rust
#[derive(Debug)]
enum Chunk {
    Literal(String),
    Formatted {
        varname: String,
        format_spec: Option<String>,
    },
}
```

---

# 📜 Now: the `nom` parser

We'll define:

- A parser for `Formatted`
- A parser for `Literal`
- A parser for a **sequence** of chunks

Here’s a full version:

```rust
use chrono::prelude::*;
use nom::{
    IResult,
    branch::alt,
    bytes::complete::{take_until, tag, take_while1},
    combinator::{map, opt},
    multi::many0,
    sequence::{delimited, separated_pair},
};
use std::collections::HashMap;

pub trait Formattable {
    fn format(&self, spec: &str) -> String;
}

// Example impls
impl Formattable for i64 {
    fn format(&self, _spec: &str) -> String { self.to_string() }
}
impl Formattable for f64 {
    fn format(&self, _spec: &str) -> String { self.to_string() }
}
impl Formattable for String {
    fn format(&self, _spec: &str) -> String { self.clone() }
}
impl Formattable for chrono::DateTime<chrono::Utc> {
    fn format(&self, spec: &str) -> String {
        self.format(spec).to_string()
    }
}

#[derive(Debug)]
enum Chunk {
    Literal(String),
    Formatted {
        varname: String,
        format_spec: Option<String>,
    },
}

// Parser for a formatted chunk: {var[:format]}
fn parse_formatted(input: &str) -> IResult<&str, Chunk> {
    let (input, (var, fmt)) = delimited(
        tag("{"),
        separated_pair(
            take_while1(|c: char| c != '}' && c != ':'),
            opt(tag(":")),
            opt(take_while1(|c: char| c != '}')),
        ),
        tag("}"),
    )(input)?;
    
    let varname = var.to_string();
    let format_spec = fmt.map(|s| s.to_string());
    Ok((input, Chunk::Formatted { varname, format_spec }))
}

// Parser for a literal text chunk (until next {)
fn parse_literal(input: &str) -> IResult<&str, Chunk> {
    map(
        take_until("{"),
        |s: &str| Chunk::Literal(s.to_string())
    )(input)
}

// The full parser: many chunks
fn parse_chunks(input: &str) -> IResult<&str, Vec<Chunk>> {
    many0(
        alt((
            parse_literal,
            parse_formatted,
        ))
    )(input)
}

// Main format_str function
pub fn format_str(pattern: &str, vars: &HashMap<&str, Box<dyn Formattable>>) -> String {
    let (_, chunks) = parse_chunks(pattern).expect("Failed to parse pattern");
    
    let mut result = String::new();
    for chunk in chunks {
        match chunk {
            Chunk::Literal(s) => result.push_str(&s),
            Chunk::Formatted { varname, format_spec } => {
                if let Some(value) = vars.get(varname.as_str()) {
                    result.push_str(&value.format(format_spec.as_deref().unwrap_or("")));
                } else {
                    // Variable not found — put back as raw
                    result.push_str(&format!("{{{}}}", varname));
                }
            }
        }
    }
    result
}
```

---

# 🎯 Usage:

```rust
fn main() {
    let utc: DateTime<Utc> = Utc::now();
    let mut vars: HashMap<&str, Box<dyn Formattable>> = HashMap::new();
    vars.insert("utc", Box::new(utc));

    let result = format_str("Today is {utc:%Y-%m-%d}", &vars);
    println!("{}", result);
}
```

✅ Outputs something like:

```
Today is 2025-04-28
```

---

# ⚡ Improvements over the imperative version:

| Aspect                | Imperative       | `nom` parser        |
|:----------------------|:-----------------|:--------------------|
| Readability           | Moderate          | Higher (structure enforced by types) |
| Extensibility         | Medium             | High (can add more constructs easily) |
| Error Handling        | Manual              | Built into combinators |
| Performance           | Good               | Very good (zero-copy parsing) |
| Fun                   | ✅                 | ✅✅✅ |

---

# 🧠 Bonus thoughts:

- If you want to support **escaped braces** `{{` and `}}`, you can extend the parser a bit.
- If you want better **error messages**, you can enrich the error types `nom` returns.
- You could even support *expression parsing inside braces* in the future (like `f"{a+b}"`) — it's totally extendable now!

---

Would you like me to **extend this** a little further —  
maybe showing how to **make it accept positional arguments** (like `{0}`, `{1}`),  
or **turn it into a derive macro** so your custom types auto-implement `Formattable`? 🚀

(It's pretty fun once we get the `nom` base working!)  
Would you want that? 🌟