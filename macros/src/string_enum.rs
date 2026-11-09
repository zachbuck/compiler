
//! This module contains the macro for emulating a string backed enum in rust.
//! Its only member is the macro. See its documentation for details.

/// This macro generates implementations of `TryFrom<&str>` and `Display`
/// for a given enum based on pairing each enum variant to a string literal
/// that should be treated as it's equivalent
/// 
/// ## Example
/// 
/// ```
/// # use macros::string_enum::generate_string_enum;
/// generate_string_enum! {
/// 	#[derive(PartialEq)]
/// 	enum Fruit {
/// 		Apple = "apple!",
/// 		Orange = "orange?",
/// 	}
/// }
/// 
/// println!("{}", Fruit::Apple); // prints 'apple!'
/// 
/// let fruit: Fruit = "orange?".try_into().unwrap();
/// assert!(Fruit::Orange == fruit);
/// 
/// let fruit: Result<Fruit, _> = "bannana".try_into();
/// assert!(Err(()) == fruit);
/// ```
/// 
/// ## Notes
/// 
/// This crate automatically generates implementations of `Clone`, `Copy`, 
/// `Display`, and `TryFrom<&str>`. If you implement these yourself for your
/// enum it could cause errors. In addition, using function-like macros inside
/// of this macro will not work and will cause errors. This may be fixed at
/// a future point.
pub use macros_internal::generate_string_enum as generate_string_enum;