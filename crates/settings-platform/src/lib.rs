pub fn platform_name() -> &'static str {
    if cfg!(windows) { "windows" } else { "unsupported" }
}

#[cfg(test)]
mod tests {
    use super::platform_name;

    #[test]
    fn platform_is_known() {
        assert!(["windows", "unsupported"].contains(&platform_name()));
    }
}
