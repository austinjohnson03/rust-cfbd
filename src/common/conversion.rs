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

pub trait IntoOptional<T> {
    fn into_optional(self) -> Option<T>;
}

impl<T> IntoOptional<T> for T {
    fn into_optional(self) -> Option<T> {
        Some(self)
    }
}

impl<T> IntoOptional<T> for Option<T> {
    fn into_optional(self) -> Option<T> {
        self
    }
}
