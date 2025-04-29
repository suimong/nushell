# Q
Wonderful! Now I like Python's design and implementation on string formatting, which is both powerful, and modular, and extensible. I want to port that idea to rust. Here's how it works in my head: i had this rust program, that takes in a string, and parses it to a few different rust types, think string, integer, float, boolean (these are some regular types), and some custom types e.g. filesize, datetime (e.g. the string "1mb" will be parsed and deserialized into the filesize type that represents the idea of 1 megabyte).  Now i want to have a rust function that works like python's fstring and `str.format`, in that it takes a string pattern as the first argument, and also accepts variadic arguments as arguments to the string pattern, and it returns the formatted string. It should use the python f-string syntax, i.e. use curly braces to denote the interpolation part; within the curly braces, characters before a colon (":") is used to identify argument bindings, and characters after colon are considered formatting string. let's say such function is called `format_str`, and i call it like this ```rust
use chrono::prelude::*;
let utc: DateTime<Utc> = Utc::now();
let utc_str = format_str("{utc:%Y-%m-%d}".to_string(), utc)"
```
and `utc_str` is a string in the format of "%Y-%m-%d". 

Can you help me ?


**Yes!**  
I love the design you're going for — it's very thoughtful: a *Pythonic* string interpolation engine, but in Rust, typed properly, and still very ergonomic.

Let's start designing it together.  
First, **let me restate your goal** clearly so we're fully in sync:

---

# 📜 Your design goal:

- A Rust function (say, `format_str`) that:
  - Takes a string pattern (containing `{}` interpolations with optional `:format` parts),
  - Takes arguments (dynamically?) that are inserted into `{}` placeholders,
  - If the placeholder contains a format spec (after `:`), it should format accordingly (e.g., `DateTime` should use `strftime` if formatting is given),
  - Works similar to **Python f-strings**,
  - Should support **custom types** (like your `FileSize`).

---

# 🛠 First thoughts:

- Rust does **NOT** have native variadic functions (without macros), so normal `format!()` uses *macros* to fake it.
- You probably want a **trait** like:

```rust
trait Formattable {
    fn format(&self, spec: &str) -> String;
}
```

- Then, your `format_str` function could accept a map (e.g. a `HashMap<String, Box<dyn Formattable>>`) of bindings — mapping from variable names to values.

---

# 🛤 Rough sketch:

First, **the trait**:

```rust
pub trait Formattable {
    fn format(&self, spec: &str) -> String;
}
```

Now, **some impls**:

```rust
impl Formattable for i64 {
    fn format(&self, _spec: &str) -> String {
        self.to_string()
    }
}

impl Formattable for f64 {
    fn format(&self, _spec: &str) -> String {
        self.to_string()
    }
}

impl Formattable for String {
    fn format(&self, _spec: &str) -> String {
        self.clone()
    }
}

impl Formattable for chrono::DateTime<chrono::Utc> {
    fn format(&self, spec: &str) -> String {
        self.format(spec).to_string()
    }
}
```

(You can implement it for your `FileSize` too.)

---

Now, **your main function**:

```rust
use std::collections::HashMap;

pub fn format_str(pattern: &str, vars: &HashMap<&str, Box<dyn Formattable>>) -> String {
    let mut result = String::new();
    let mut chars = pattern.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c == '{' {
            if let Some(&'{') = chars.peek() {
                // Escaped {{
                chars.next();
                result.push('{');
                continue;
            }
            
            // parse variable
            let mut var_name = String::new();
            let mut format_spec = String::new();
            let mut in_format = false;
            while let Some(&ch) = chars.peek() {
                if ch == '}' {
                    chars.next();
                    break;
                }
                if ch == ':' && !in_format {
                    in_format = true;
                    chars.next();
                    continue;
                }
                if in_format {
                    format_spec.push(ch);
                } else {
                    var_name.push(ch);
                }
                chars.next();
            }
            
            if let Some(value) = vars.get(var_name.trim()) {
                result.push_str(&value.format(format_spec.trim()));
            } else {
                result.push_str(&format!("{{{}}}", var_name));
            }
        } else if c == '}' {
            if let Some(&'}') = chars.peek() {
                // Escaped }}
                chars.next();
                result.push('}');
            } else {
                panic!("Unmatched }} in pattern!");
            }
        } else {
            result.push(c);
        }
    }
    
    result
}
```

---

# 🚀 Usage example:

```rust
use chrono::prelude::*;
use std::collections::HashMap;

fn main() {
    let utc: DateTime<Utc> = Utc::now();
    
    let mut vars: HashMap<&str, Box<dyn Formattable>> = HashMap::new();
    vars.insert("utc", Box::new(utc));
    
    let utc_str = format_str("{utc:%Y-%m-%d}", &vars);
    println!("{}", utc_str);
}
```

---

# 🧠 Things to note:

- We manually parse `{}` and extract `varname` + `format_spec`.
- Escaped `{{` and `}}` are supported.
- If the variable is not found, it just leaves it as-is (you can change this to error if you prefer).
- You can implement `Formattable` for any type you want (your own `FileSize`, `bool`, whatever).

---

# 🧨 Future Improvements

- Error handling: give proper error if `}` is missing.
- Maybe use a macro to allow calling like `format_str!("{utc:%Y-%m-%d}", utc)` directly without needing to build the `HashMap` manually.
- Caching the parsing stage (parse once, interpolate multiple times).
- Better performance (avoid many allocations).

---

# 🎯 Summary

You are **porting** Python's **"object + __format__"** design, in a **very Rusty**, **modular**, **zero-magic** way.  
This approach is super clean.

---

Would you like me to also show a nicer **macro version** that makes calling it more ergonomic, like how Python f-strings don't need to manually build the variables map? 🚀 (it's very fun and still quite readable!)  
Would you? 🎯
