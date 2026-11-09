# TODO

## Additional Packages

- [ ] Custom Regex (compile-time)
- [ ] Custom Trie (compile-time)
- [ ] File buffer for them to draw from (avoid max token hard cap)

## macros

- `string_enum`:
  - [ ] add support for macros in the macro so it generally plays nicer
  - [ ] implement faster compile-time data structure for `TryFrom<&str>` (e.g., a compile-time trie?)
  - [ ] figure out unit tests of some kind

## tokenizer

- [ ] Make LiteralType more data driven, have it automatically derive TryFrom backing type.
