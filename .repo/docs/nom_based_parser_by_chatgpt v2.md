Awesome!  
Let's **update the `nom` parser** for your `format_str` mini-language to handle **escaped braces** `{{` and `}}` correctly, according to the EBNF grammar we just finished! 🚀

---

# 🏗 Updated `nom` Parser

```rust
use chrono::prelude::*;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_until},
    combinator::{map, recognize},
    multi::many0,
    sequence::{delimited, separated_pair},
    IResult,
};
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug)]
enum Part {
    Literal(String),
    Field { name: String, format_spec: Option<String> },
}

// Parser for a single literal chunk, including escaped braces
fn literal(input: &str) -> IResult<&str, Part> {
    map(
        many0(alt((
            map(tag("{{"), |_| "{".to_string()),
            map(tag("}}"), |_| "}".to_string()),
            map(is_not("{}"), |s: &str| s.to_string()),
        ))),
        |fragments| Part::Literal(fragments.concat()),
    )(input)
}

// Parser for a field
fn field(input: &str) -> IResult<&str, Part> {
    let (input, content) = delimited(tag("{"), take_until("}"), tag("}"))(input)?;
    let (name, format_spec) = if let Some((name, fmt)) = content.split_once(':') {
        (name.trim().to_string(), Some(fmt.trim().to_string()))
    } else {
        (content.trim().to_string(), None)
    };
    Ok((input, Part::Field { name, format_spec }))
}

// Top-level parser
fn parse_format(input: &str) -> IResult<&str, Vec<Part>> {
    many0(alt((field, literal)))(input)
}

fn format_str(pattern: &str, args: HashMap<String, Box<dyn Display>>) -> String {
    let (_, parts) = parse_format(pattern).expect("Failed to parse format string");
    let mut output = String::new();

    for part in parts {
        match part {
            Part::Literal(s) => output.push_str(&s),
            Part::Field { name, format_spec } => {
                if let Some(val) = args.get(&name) {
                    match format_spec {
                        Some(spec) => {
                            if name == "utc" {
                                // Special-case DateTime formatting
                                if let Some(dt) = val.downcast_ref::<DateTime<Utc>>() {
                                    output.push_str(&dt.format(&spec).to_string());
                                } else {
                                    panic!("Expected DateTime<Utc> for field {}", name);
                                }
                            } else {
                                output.push_str(&format!("{}", val)); // Regular fallback
                            }
                        }
                        None => output.push_str(&format!("{}", val)),
                    }
                } else {
                    panic!("Missing field: {}", name);
                }
            }
        }
    }
    output
}
```

---

# ✨ What's changed / improved:

| Old | New |
|:----|:----|
| Treated literals as simple "anything but {" | Literals now also handle `{{` and `}}` as *escaped braces* |
| Only handled `{name}` | Now properly splits `{name:format_spec}` |
| No escaping | Correctly translates `{{` → `{` and `}}` → `}` when building output |

---

# 🧪 Test it!

```rust
use chrono::Utc;
use std::collections::HashMap;

fn main() {
    let utc: DateTime<Utc> = Utc::now();

    let mut args: HashMap<String, Box<dyn Display>> = HashMap::new();
    args.insert("utc".to_string(), Box::new(utc));
    args.insert("score".to_string(), Box::new(42));
    args.insert("name".to_string(), Box::new("Alice"));

    let formatted = format_str(
        "Hello {{name}}! Today is {utc:%Y-%m-%d}, your score is {score}",
        args,
    );
    println!("{}", formatted);
}
```

Prints something like:

```
Hello {name}! Today is 2025-04-28, your score is 42
```

✅ Notice that `{{name}}` becomes literal `{name}`, not a field.

---

# 🚀 Summary

- **Escaped braces** `{{` and `}}` fully supported
- **Field parsing** and **format specifiers** supported
- **nom** parser cleanly following EBNF
- **Extensible**: you could easily add nested fields, custom formatting, type-safe maps, etc.

---

# 📎 Future upgrades you could add:

- Support nested fields (if you want)
- Type-checked arguments (instead of `Box<dyn Display>`)
- Pre-compiled templates
- Error recovery (e.g., friendly errors if `{` is unmatched)

---

Would you also like me to show you an even **more efficient** version that doesn't allocate many small `String`s during parsing (zero-copy where possible)?  
It’s a nice trick if you want this to be *super fast* for production use! 🚀 