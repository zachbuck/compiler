
pub trait StringEnum : Sized + Into<usize> + 'static {
	const ALL: &'static [Self];
	const LABELS: &'static [&'static str];

	fn from_string(label: &str) -> Self;
}
