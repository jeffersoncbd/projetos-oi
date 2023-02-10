pub fn string_to_static(value: String) -> &'static str {
    let value: &'static str = Box::leak(value.into_boxed_str());
    value
}
