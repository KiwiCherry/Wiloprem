//! Typographic hierarchy.
//!
//! Be Vietnam Pro (Regular and SemiBold) carries every Vietnamese diacritic, Lucide provides
//! glyph icons and the monospace role falls back to Be Vietnam Pro for accented characters.
//! See `build_fonts` in `src/gui/theme.rs` for how the families are registered.

use eframe::egui::{Color32, FontFamily, FontId, RichText};

/// Named font family holding the SemiBold face.
pub fn semibold_family() -> FontFamily {
    FontFamily::Name("semibold".into())
}

/// Semantic text roles. Views pick a role instead of a point size.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextRole {
    /// Screen and modal headlines.
    Display,
    /// Panel and conversation titles.
    Title,
    /// Section headers and emphasised labels.
    Subtitle,
    /// Message text and list content.
    Body,
    /// Controls, tabs and secondary labels.
    Label,
    /// Timestamps, helper text and badges.
    Caption,
    /// Code blocks and identifiers.
    Mono,
}

impl TextRole {
    /// Font size in points.
    pub const fn size(self) -> f32 {
        match self {
            Self::Display => 20.0,
            Self::Title => 15.0,
            Self::Subtitle => 14.0,
            Self::Body => 13.5,
            Self::Label => 12.5,
            Self::Caption => 11.5,
            Self::Mono => 13.0,
        }
    }

    /// Font family for the role.
    pub fn family(self) -> FontFamily {
        match self {
            Self::Display | Self::Title | Self::Subtitle => semibold_family(),
            Self::Body | Self::Label | Self::Caption => FontFamily::Proportional,
            Self::Mono => FontFamily::Monospace,
        }
    }

    /// Complete `FontId` for painter and layout calls.
    pub fn font_id(self) -> FontId {
        FontId::new(self.size(), self.family())
    }

    /// Styled rich text in this role.
    pub fn rich(self, text: impl Into<String>, color: Color32) -> RichText {
        RichText::new(text).font(self.font_id()).color(color)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hierarchy_is_strictly_ordered_by_size() {
        let ordered = [
            TextRole::Caption,
            TextRole::Label,
            TextRole::Mono,
            TextRole::Body,
            TextRole::Subtitle,
            TextRole::Title,
            TextRole::Display,
        ];
        for pair in ordered.windows(2) {
            assert!(pair[0].size() < pair[1].size(), "{:?} >= {:?}", pair[0], pair[1]);
        }
    }

    #[test]
    fn headings_use_the_semibold_family() {
        assert_eq!(TextRole::Title.family(), semibold_family());
        assert_eq!(TextRole::Body.family(), FontFamily::Proportional);
        assert_eq!(TextRole::Mono.family(), FontFamily::Monospace);
    }
}
