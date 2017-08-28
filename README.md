S-expression parser in Rust
=================================

This library provides a way to easily parse S-expressions into a linked list. 
It tries to mimic the `read` function in `r5rs` scheme.

The library is based on the [pest]() parser, the syntax can be easily expanded by editing the pest grammar file and by adding extra types to the enum in order to convert the result of pest into the AST.

Examples can be found in the documentation and in the tests module.

## License
Mozilla Public License 2.0
