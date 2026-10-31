pub const APP_NAME: &str = "Win10 Settings";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Page {
    Home,
    System,
    Personalization,
    Apps,
    Accounts,
    TimeLanguage,
    Gaming,
    EaseOfAccess,
    Privacy,
    UpdateSecurity,
}

impl Page {
    pub const ALL: [Self; 10] = [
        Self::Home,
        Self::System,
        Self::Personalization,
        Self::Apps,
        Self::Accounts,
        Self::TimeLanguage,
        Self::Gaming,
        Self::EaseOfAccess,
        Self::Privacy,
        Self::UpdateSecurity,
    ];

    pub const fn title(self) -> &'static str {
        match self {
            Self::Home => "Home",
            Self::System => "System",
            Self::Personalization => "Personalization",
            Self::Apps => "Apps",
            Self::Accounts => "Accounts",
            Self::TimeLanguage => "Time & Language",
            Self::Gaming => "Gaming",
            Self::EaseOfAccess => "Ease of Access",
            Self::Privacy => "Privacy",
            Self::UpdateSecurity => "Update & Security",
        }
    }

    pub const fn subtitle(self) -> &'static str {
        match self {
            Self::Home => "Quick access to common settings",
            Self::System => "Display, sound, notifications, power, storage",
            Self::Personalization => "Background, colors, themes, lock screen, taskbar",
            Self::Apps => "Installed apps, startup, defaults, optional features",
            Self::Accounts => "Your info, sign-in options, family, sync",
            Self::TimeLanguage => "Date, time, region, language, speech",
            Self::Gaming => "Game Bar, captures, Game Mode",
            Self::EaseOfAccess => "Vision, hearing, interaction",
            Self::Privacy => "Location, camera, microphone, diagnostics",
            Self::UpdateSecurity => "Windows Update, recovery, developer, security",
        }
    }

    pub const fn icon(self) -> &'static str {
        match self {
            Self::Home => "home",
            Self::System => "settings",
            Self::Personalization => "palette",
            Self::Apps => "apps",
            Self::Accounts => "person",
            Self::TimeLanguage => "schedule",
            Self::Gaming => "sports_esports",
            Self::EaseOfAccess => "accessibility_new",
            Self::Privacy => "privacy_tip",
            Self::UpdateSecurity => "security",
        }
    }

    pub const fn deep_link(self) -> &'static str {
        match self {
            Self::Home => "ms-settings:",
            Self::System => "ms-settings:system",
            Self::Personalization => "ms-settings:personalization",
            Self::Apps => "ms-settings:appsfeatures",
            Self::Accounts => "ms-settings:yourinfo",
            Self::TimeLanguage => "ms-settings:dateandtime",
            Self::Gaming => "ms-settings:gaming-gamebar",
            Self::EaseOfAccess => "ms-settings:easeofaccess",
            Self::Privacy => "ms-settings:privacy",
            Self::UpdateSecurity => "ms-settings:windowsupdate",
        }
    }

    pub const fn keywords(self) -> &'static [&'static str] {
        match self {
            Self::Home => &["start", "overview"],
            Self::System => &[
                "display",
                "sound",
                "notifications",
                "power",
                "sleep",
                "storage",
                "multitasking",
                "about",
                "resolution",
            ],
            Self::Personalization => &[
                "background",
                "wallpaper",
                "colors",
                "accent",
                "themes",
                "lock screen",
                "taskbar",
                "start",
                "fonts",
            ],
            Self::Apps => &[
                "apps",
                "features",
                "startup",
                "default apps",
                "optional features",
                "uninstall",
            ],
            Self::Accounts => &[
                "account",
                "sign in",
                "sign-in",
                "family",
                "users",
                "sync",
                "email",
            ],
            Self::TimeLanguage => &[
                "time",
                "date",
                "region",
                "language",
                "speech",
                "keyboard",
                "timezone",
            ],
            Self::Gaming => &["game bar", "captures", "game mode", "xbox"],
            Self::EaseOfAccess => &[
                "accessibility",
                "narrator",
                "magnifier",
                "high contrast",
                "cursor",
                "keyboard",
                "mouse",
            ],
            Self::Privacy => &[
                "location",
                "camera",
                "microphone",
                "background apps",
                "diagnostics",
                "activity history",
            ],
            Self::UpdateSecurity => &[
                "update",
                "windows update",
                "activation",
                "recovery",
                "developers",
                "security",
                "backup",
                "troubleshoot",
            ],
        }
    }

    fn matches_lower(self, query: &str) -> bool {
        self.title().to_lowercase().contains(query)
            || self.subtitle().to_lowercase().contains(query)
            || self
                .keywords()
                .iter()
                .any(|keyword| keyword.to_lowercase().contains(query))
    }
}

pub fn search(query: &str) -> Vec<Page> {
    let normalized = query.trim().to_lowercase();

    if normalized.is_empty() {
        return Page::ALL.to_vec();
    }

    Page::ALL
        .into_iter()
        .filter(|page| page.matches_lower(&normalized))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{APP_NAME, APP_VERSION, Page, search};

    #[test]
    fn app_metadata_is_present() {
        assert_eq!(APP_NAME, "Win10 Settings");
        assert!(!APP_VERSION.is_empty());
    }

    #[test]
    fn empty_search_returns_all_pages() {
        assert_eq!(search("  "), Page::ALL);
    }

    #[test]
    fn search_matches_titles_keywords_and_is_case_insensitive() {
        assert_eq!(search("DISPLAY"), vec![Page::System]);
        assert_eq!(search("wallpaper"), vec![Page::Personalization]);
        assert_eq!(search("windows update"), vec![Page::UpdateSecurity]);
    }

    #[test]
    fn unknown_search_returns_nothing() {
        assert!(search("does not exist").is_empty());
    }
}
