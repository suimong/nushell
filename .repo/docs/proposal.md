# A mini templating language for Nushell

While working on #xxx and #xxx, and having inspired by rose's work and darren, it occured to me that a mini templating language could be a quite useful addition to Nushell.

## Goals

The primary consumer of this mini-language is `format pattern`, but it would be a net win if we can extend its usage to other commands where appropriate, e.g. `parse`.

The mini-language is largely inspired by Python's [format specification mini-language](https://docs.python.org/3/library/string.html#format-specification-mini-language),[f-string](https://docs.python.org/3/reference/lexical_analysis.html#formatted-string-literals), and GNU `printf` program.

Overall, the goal for this mini language is to be easy and intuitive to use, taking advantage of Nushell's distinct features,while maintaining roughly the same power as Python's format specification.




## Features & non-features

1. Curly brace `{}` based replacement field syntax (a.k.a "argument", TODO add more similar terms, ask LLM). This is already the case for `format pattern`, and I see absolutely no reason to change to `printf` style.

2. Support formatting for most of Nushell's atomic data types:
    - number: `int`, `float`
    - string
    - date
    - duration
    - filesize
    - bytes
3. Composite types i.e. `list` and `reocrd` are not supported for a few reasons:
    - syntactically it will lead to ambiguity (TODO: add example). Thought this could be worked around by providing a flag to `format pattern`, but since this this mini language can be used in other commands e.g. `parse`, we need to think carefully whether this ambiguity can be resovled in all cases.
    - the scope of "formatting" that we could reasonably do to lists and records seems out of scope of the generally accepted idea of "formatting". For example, we could hypothetically support "formatting" a record as json string: 

    ```nushell
    [{foo: 0, bar: "a"}, {foo: 1, bar: "b"}] | format pattern '{:jsonl}'
    
    ```
    This is essentially identical to the `to json` command, but much more awkward.
4. 


## Current status

We have a 

## Implementation Plan

- Merge in rose's work on accepting more input to `format pattern`.
