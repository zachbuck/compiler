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

- [ ] Make `Tokenizer` handle errors more gracefully
  - [ ] Backing buffer errors
  - [ ] Unknown token
  - [ ] Make the functions that automatically collect return these errors if they occur
  - [ ] Maybe have to make a custom iterator function? idk?
- [ ] Better keyword search (trie?)
- [ ] Switch everything to just operate on `&[u8]` internally. (probably more performant)
