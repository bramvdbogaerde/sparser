S-expression parser in Rust
=================================

[![Crates.io](https://img.shields.io/badge/crates.io-0.1.0-brightgreen.svg)](https://crates.io/crates/sparser)
[![Documentation](https://docs.rs/sparser/badge.svg)](https://docs.rs/sparser)

This library provides a way to easily parse S-expressions into a linked list. 
It tries to mimic the `read` function in `r5rs` scheme.

The library is based on the [pest](https://github.com/pest-parser/pest) parser, the syntax can be easily expanded by editing the pest grammar file and by adding extra types to the enum in order to convert the result of pest into the AST.

Examples can be found in the documentation and in the tests module.

## License
Mozilla Public License 2.0
