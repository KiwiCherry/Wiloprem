//! Spacing, shape, stroke, size and layout tokens.
//!
//! Every value is expressed in logical points. Views must reference these constants
//! instead of writing literals, so a change here restyles the whole app.

#![allow(dead_code)]

/// Spacing scale (4pt grid).
pub mod space {
    pub const XXS: f32 = 2.0;
    pub const XS: f32 = 4.0;
    pub const SM: f32 = 8.0;
    pub const MD: f32 = 12.0;
    pub const LG: f32 = 16.0;
    pub const XL: f32 = 24.0;
    pub const XXL: f32 = 32.0;
}

/// Corner radius ladder. Avatars are always circles and use [`FULL`].
pub mod radius {
    pub const XS: f32 = 4.0;
    pub const SM: f32 = 6.0;
    pub const MD: f32 = 8.0;
    pub const LG: f32 = 12.0;
    pub const XL: f32 = 16.0;
    /// Large enough to turn any control into a capsule.
    pub const FULL: f32 = 999.0;
}

/// Stroke widths.
pub mod stroke {
    pub const HAIRLINE: f32 = 1.0;
    pub const EMPHASIS: f32 = 1.5;
    pub const FOCUS: f32 = 2.0;
}

/// Control and icon sizes.
pub mod size {
    pub const ICON_SM: f32 = 14.0;
    pub const ICON_MD: f32 = 18.0;
    pub const ICON_LG: f32 = 22.0;

    pub const CONTROL_H_SM: f32 = 28.0;
    pub const CONTROL_H: f32 = 32.0;
    pub const CONTROL_H_LG: f32 = 40.0;

    pub const AVATAR_XS: f32 = 24.0;
    pub const AVATAR_SM: f32 = 32.0;
    pub const AVATAR_MD: f32 = 40.0;
    pub const AVATAR_LG: f32 = 64.0;

    pub const BADGE_H: f32 = 16.0;
}

/// Window-level layout metrics and responsive breakpoints.
pub mod layout {
    /// Below this width the info panel becomes an overlay.
    pub const BREAKPOINT_COMPACT: f32 = 880.0;
    /// Below this width the conversation list collapses to avatars only.
    pub const BREAKPOINT_NARROW: f32 = 620.0;

    pub const RAIL_W: f32 = 60.0;
    pub const RAIL_W_NARROW: f32 = 52.0;
    pub const RAIL_ITEM: f32 = 40.0;

    pub const SIDEBAR_W_DEFAULT: f32 = 288.0;
    pub const SIDEBAR_W_MIN: f32 = 232.0;
    pub const SIDEBAR_W_MAX: f32 = 480.0;
    pub const SIDEBAR_W_COLLAPSED: f32 = 68.0;

    pub const HEADER_H: f32 = 48.0;
    pub const THREAD_ROW_H: f32 = 56.0;
    pub const INFO_PANEL_W: f32 = 300.0;

    /// Widest a single message body may grow, as a fraction of the timeline width.
    pub const MESSAGE_MAX_FRACTION: f32 = 0.72;
}

/// Hover / state transition durations, in seconds, before the user speed setting applies.
pub mod motion {
    pub const FAST: f32 = 0.10;
    pub const BASE: f32 = 0.15;
    pub const SLOW: f32 = 0.25;
}
