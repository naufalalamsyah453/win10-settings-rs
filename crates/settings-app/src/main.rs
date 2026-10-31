fn main() {
    println!(
        "{} {} (phase 0 scaffold) on {}",
        settings_core::APP_NAME,
        settings_core::APP_VERSION,
        settings_platform::platform_name()
    );
}
