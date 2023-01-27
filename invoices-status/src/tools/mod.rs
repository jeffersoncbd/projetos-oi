pub mod date;

pub fn format_adapter(value: String) -> &'static str {
    let value: &'static str = Box::leak(value.into_boxed_str());
    value
}
