//! Decoupled UI actions emitted by stateless view functions (`Left Nav Rail`, `Sidebar`, `Chat Header`, `Modals`).
//!
//! View closures read from clean view state and push lightweight `UiAction` variants rather than
//! mutating backend state, spawning threads, or writing to SQLite directly inside nested `|ui| { ... }` closures.

use crate::gui::stickers::ThreadFilter;

/// Aspect-ratio scaling mode for customizable chat background wallpapers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub(crate) enum WallpaperFitMode {
    /// Fill the entire chat viewport without distortion by cropping excess UV edges.
    #[default]
    Cover,
    /// Fit the entire image inside the chat viewport while preserving aspect ratio.
    Contain,
    /// Stretch the image to match the exact chat viewport dimensions.
    Stretch,
}

impl WallpaperFitMode {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Cover => "cover",
            Self::Contain => "contain",
            Self::Stretch => "stretch",
        }
    }

    pub(crate) fn from_str_lossy(s: &str) -> Self {
        match s.trim().to_ascii_lowercase().as_str() {
            "contain" => Self::Contain,
            "stretch" => Self::Stretch,
            _ => Self::Cover,
        }
    }

    pub(crate) fn label_vi(self) -> &'static str {
        match self {
            Self::Cover => "Phủ kín (Cover)",
            Self::Contain => "Vừa khung (Contain)",
            Self::Stretch => "Kéo giãn (Stretch)",
        }
    }
}

/// Lightweight UI intent emitted by the App Shell, Navigation Rail, Sidebar, Header, or Modals.
#[derive(Debug, Clone)]
pub(crate) enum UiAction {
    /// Toggle the user profile / account switcher popover.
    ToggleProfileMenu,
    /// Switch to the main Messages workspace (`ThreadFilter::All`).
    OpenMessagesNav,
    /// Toggle the unified Contacts & Groups view.
    ToggleContactsNav,
    /// Jump directly to the personal "Cloud của tôi" conversation.
    OpenCloudThread,
    /// Open the Social Feed / Timeline page.
    OpenFeedNav,
    /// Trigger phone message history synchronization with the mobile device.
    TriggerPhoneSync,
    /// Toggle the Settings modal.
    ToggleSettingsModal,
    /// Open the phone lookup / add friend modal (optionally pre-filling a phone query).
    OpenPhoneLookupModal(Option<String>),
    /// Clear the sidebar search query.
    ClearSidebarSearch,
    /// Switch the sidebar thread filter tab (`All`, `Direct`, `Groups`).
    SetThreadFilter(ThreadFilter),
    /// Select a Chat Folder tab (`""` = All).
    SetActiveChatFolder(String),
    /// Open the Chat Folder management modal.
    OpenFolderModal,
    /// Select a conversation thread by index, thread ID, and group flag.
    SelectThread {
        index: usize,
        thread_id: String,
        is_group: bool,
    },
    /// Jump to a conversation from a global message search hit and open in-chat search.
    SelectSearchMessageHit {
        index: usize,
        query: String,
    },
    /// Toggle the right-hand Conversation Info Drawer.
    ToggleInfoDrawer {
        persist: bool,
    },
    /// Toggle the in-chat message search bar.
    ToggleInChatSearch,
    /// Toggle the vanish (disappearing messages) picker popover.
    ToggleVanishPicker,
    /// Start a voice or video call with the active conversation.
    StartCall {
        thread_id: String,
        thread_name: String,
        is_video: bool,
        is_group: bool,
    },
    /// Open the local Contact Alias (`Tên gợi nhớ`) editor modal for `user_id`.
    OpenAliasModal {
        user_id: String,
        original_name: String,
    },
    /// Set or update a local-only contact alias (`Tên gợi nhớ`) in SQLite and in-memory cache.
    SetContactAlias {
        user_id: String,
        alias: String,
    },
    /// Clear a local-only contact alias (`Tên gợi nhớ`), reverting to the original Zalo display name.
    ClearContactAlias {
        user_id: String,
    },
    /// Open native image picker to set global wallpaper (`chat_id: None`) or per-chat override (`chat_id: Some(id)`).
    PickWallpaper {
        chat_id: Option<String>,
    },
    /// Update global wallpaper rendering style (`dim_opacity`, `panel_opacity`, `fit_mode`).
    UpdateWallpaperStyle {
        dim_opacity: f32,
        panel_opacity: f32,
        fit_mode: WallpaperFitMode,
    },
    /// Update per-conversation wallpaper dim overlay opacity (`0.0..=0.95`).
    UpdateChatWallpaperDim {
        chat_id: String,
        dim_opacity: f32,
    },
    /// Clear the custom wallpaper for a specific conversation (`Some(id)`) or globally (`None`).
    ClearWallpaper {
        chat_id: Option<String>,
    },
}
