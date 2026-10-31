pub const APP_NAME: &str = "Win10 Settings";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::{APP_NAME, APP_VERSION};

    #[test]
    fn app_metadata_is_present() {
        assert_eq!(APP_NAME, "Win10 Settings");
        assert!(!APP_VERSION.is_empty());
    }
}
