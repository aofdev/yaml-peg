# yaml-peg

![Check lint, format and tests](https://github.com/aofdev/yaml-peg/actions/workflows/main.yml/badge.svg)

PEG parser ([pest](https://pest.rs/)) for YAML written in Rust 🦀

## Quick Start ⚡️

```bash
# Run
cargo run -- --file example_files/test.yaml

# Output
{
  "xmas": "true",
  "calling-birds": "[\"huey\", \"dewey\", \"louie\", \"fred\"]",
  "birds": "[\"huey-2\", \"dewey-2\"]",
  "french-hens": "3",
  "ray": "a drop of golden sun",
  "test1": "false",
  "doe": "a deer, a female deer",
  "pi": "3.14159",
  "array-test": "[\"DFASf\", \"2222\"]",
  "str-folded-style": "  that is folded  into two lines  and it is not indented  into three lines\n",
  "str-literal-style": "  this is my very very very\n  long string\n  that is folded\n  into two lines\n  and it is not indented\n"
}
```

## TODO

- [x] One level
- [x] String multiline (folded-style, literal-style)
- [ ] Nested levels
- [ ] YAML 1.2
- [ ] Change binary to library
