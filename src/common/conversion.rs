pub trait IntoOptionalString {
    fn into_optional_string(self) -> Option<String>;
}

impl IntoOptionalString for String {
    fn into_optional_string(self) -> Option<String> {
        Some(self)
    }
}

impl IntoOptionalString for &str {
    fn into_optional_string(self) -> Option<String> {
        Some(self.to_string())
    }
}

impl IntoOptionalString for Option<String> {
    fn into_optional_string(self) -> Option<String> {
        self
    }
}

impl IntoOptionalString for Option<&str> {
    fn into_optional_string(self) -> Option<String> {
        self.map(|s| s.to_string())
    }
}
