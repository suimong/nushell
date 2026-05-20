# A mini templating language for Nushell

While working on #xxx and #xxx, and having inspired by rose's work and darren, it occured to me that a mini templating language could be a quite useful addition to Nushell.

## Goals

The primary consumer of this mini-language is `format pattern`, but it would be a net win if we can extend its usage to other commands where appropriate, e.g. `parse`.

The mini-language is largely inspired by Python's [format specification mini-language](https://docs.python.org/3/library/string.html#format-specification-mini-language),[f-string](https://docs.python.org/3/reference/lexical_analysis.html#formatted-string-literals), and GNU `printf` program.

Overall, the goal for this mini language is to be easy and intuitive to use, taking advantage of Nushell's distinct features,while maintaining roughly the same power as Python's format specification.




## Features & non-features

1. Curly brace `{}` based replacement field syntax (a.k.a "argument", TODO add more similar terms, ask LLM). This is already the case for `format pattern`, and I see absolutely no reason to change to `printf` style.

2. Support formatting for all of Nushell's atomic data types:
    - number: `int`, `float`
    - string
    - date
    - duration
    - filesize
    - cell-path?
    - glob?
3. Composite types i.e. `list` and `reocrd` are **not** supported for a few reasons:
    - syntactically it will lead to ambiguity (TODO: add example). Thought this could be worked around by providing a flag to `format pattern`, but since this this mini language can be used in other commands e.g. `parse`, we need to think carefully whether this ambiguity can be resovled in all cases.
    - the scope of "formatting" that we could reasonably do to lists and records seems out of scope of the generally accepted idea of "formatting". For example, we could hypothetically support "formatting" a record as json string: 

    ```nushell
    [{foo: 0, bar: "a"}, {foo: 1, bar: "b"}] | format pattern '{:jsonl}'
    
    ```
    This is essentially identical to the `to json` command, but much more awkward.
4. Support nested replacement field, something like `{value: 1.5, width: 5, precision: 4} | format pattern "result: {value:{width}.{precision}}"`

## Showcase



## Current status

We have a 

## Implementation Plan

- Merge in rose's work on accepting more input to `format pattern`.
- Write a PEG grammar, generate a parser using `pest`, 
- Implement `Formattable` trait
- Write test cases against the `pest` based parser, this makes fixing and prototyping easy.
- Use `nom` to rewrite the parser, and we now have a `trusted` test suite to be tested against.
    - `nom` is already a transitive dependency.
- See if `parse` can take advantage of this.

## Caveats

1. if the input's type is an atomic type, we face two variants of the pattern string:
    1. the bare specification, which is those 
    2. include variable substitution, then which syntax to use? 1. `{0}` or `{0:<format_spec>}`; 2. `{}`, or `{:<format_spec>}`; 3. nushell convention `{$in}` or `{$in:<format_spec>}`; 4. others...
2. accessing a field from record, which is itself a record/list, should just use `to text` to serialize, without any formatting operations
3. `[{a: 1}, {b: 2}] | format pattern "{a} -> {b}"`, should raise an error, but should we have a formatting for "non-existent" column value? something like `[{a: 1}, {b: 2}] | format pattern "{a:?} -> {b:?}"` results in `['1 -> null', 'null -> 2']`

## Details

## Compare with Python

