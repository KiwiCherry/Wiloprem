//! Decoupled UI actions emitted by stateless view functions (`Left Nav Rail`, `Sidebar`, `Chat Header`, `Modals`).
//!
//! View closures read from clean view state and push lightweight `UiAction` variants rather than
//! mutating backend state, spawning threads, or writing to SQLite directly inside nested `|ui| { ... }` closures.

use crate::gui::stickers::ThreadFilter;

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
}
