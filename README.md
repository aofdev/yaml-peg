# yaml-peg

![Check lint, format and tests](https://github.com/aofdev/yaml-peg/actions/workflows/main.yml/badge.svg)

PEG parser ([pest](https://pest.rs/)) for YAML written in Rust 🦀

## Quick Start ⚡️

```bash
# Run
cargo run -- --file example_files/test.yaml

# Output
{"pi": "3.14159", "test1": "false", "french-hens": "3", "hello": "hello", "xmas": "true", "birds": "[\"huey-2\", \"dewey-2\"]", "calling-birds": "[\"huey\", \"dewey\", \"louie\", \"fred\"]", "ray": "a drop of golden sun", "array-test": "[\"DFASf\", \"2222\"]", "doe": "a deer, a female deer"}
```

## TODO

- [x] One level
- [ ] Nested levels
- [ ] YAML 1.2
- [ ] Change binary to library
