
pub trait StringEnum {
	fn from_label(label: &str) -> Self;
	fn to_label(&self) -> &'static str;
}

pub use macros_internal::generate_string_enum;
