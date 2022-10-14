# What is this

This attempts to use https://github.com/wooorm/markdown-rs, and
adds functionality to make all internal `enum`s and `struct`s 
serializable to JSON, via `serde`

## Getting started

1. Clone
1. `cargo run .`

This will process `./src/input.mdx` and output a serializable JSON blob like

> **Warning** — `input.mdx` is _huge_ so `cargo run .` may take about 8 or more seconds.

```jsonc
{
  "children": [
    {
      "position": {
        "end": {
          "column": 59,
          "line": 1,
          "offset": 58
        },
        "start": {
          "column": 1,
          "line": 1,
          "offset": 0
        }
      },
      "type": "Html",
      "value": "<!-- This file is generated through yarn generate-docs -->"
    },
    {
      "children": [
// etc
```
