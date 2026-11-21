//! Semantic color tokens for the two themes and the egui style they produce.
//!
//! Both palettes follow a Discord-like layering: a dark rail, a mid-tone sidebar, a lighter chat
//! canvas and blurple (`#5865F2`) as the single accent. Every text/background pair that carries
//! copy is held to WCAG 2.1 AA (>= 4.5:1) by the tests at the bottom of this file.

use eframe::egui::{self, Color32, Rounding, Stroke, Vec2};

use super::tokens::{radius, stroke};

/// Resolved colors for one theme. Views read these fields and never construct colors inline.
#[derive(Debug, Clone, Copy)]
pub struct ThemePalette {
    pub is_dark: bool,
    /// Navigation rail and window chrome.
    pub bg_app: Color32,
    /// Conversation sidebar and info panel.
    pub bg_sidebar: Color32,
    /// Message timeline background.
    pub bg_chat_canvas: Color32,
    /// Modals, popovers, cards and secondary buttons.
    pub bg_elevated: Color32,
    pub bg_hover: Color32,
    pub bg_selected: Color32,
    /// Tint of a message row under the cursor.
    pub bg_message_hover: Color32,
    /// Context menus and tooltips floating above everything else.
    pub bg_floating: Color32,
    /// Dimmed backdrop behind modals.
    pub scrim: Color32,
    pub bg_input: Color32,
    pub bg_input_track: Color32,
    pub border: Color32,
    pub text_primary: Color32,
    pub text_secondary: Color32,
    pub text_muted: Color32,
    pub primary: Color32,
    pub primary_hover: Color32,
    pub accent_online: Color32,
    pub danger: Color32,
    pub danger_text: Color32,
    pub warning: Color32,
    pub reaction_star: Color32,
    pub code_bg: Color32,
    pub link_text: Color32,
    pub primary_fg: Color32,
    pub bubble_out_bg: Color32,
    pub bubble_out_text: Color32,
    pub bubble_out_meta: Color32,
    pub bubble_in_bg: Color32,
    pub bubble_in_border: Color32,
    pub bubble_in_text: Color32,
    pub bubble_in_meta: Color32,
}

/// WCAG relative luminance of an sRGB color.
pub fn relative_luminance(c: Color32) -> f32 {
    let srgb_to_linear = |v: u8| -> f32 {
        let vf = v as f32 / 255.0;
        if vf <= 0.04045 {
            vf / 12.92
        } else {
            ((vf + 0.055) / 1.055).powf(2.4)
        }
    };
    0.2126 * srgb_to_linear(c.r()) + 0.7152 * srgb_to_linear(c.g()) + 0.0722 * srgb_to_linear(c.b())
}

/// WCAG contrast ratio between two opaque colors (1.0 ..= 21.0).
pub fn contrast_ratio(c1: Color32, c2: Color32) -> f32 {
    let l1 = relative_luminance(c1);
    let l2 = relative_luminance(c2);
    (l1.max(l2) + 0.05) / (l1.min(l2) + 0.05)
}

impl ThemePalette {
    /// Semantic color for conversation classification tags.
    pub fn category_tag_color(&self, label: &str) -> Color32 {
        match label {
            "Khách hàng" => self.danger,
            "Gia đình" => self.accent_online,
            "Công việc" | "Trả lời sau" => self.warning,
            "Bạn bè" => self.primary_hover,
            _ => self.link_text,
        }
    }

    /// Accent color for heart/reaction unread badges.
    pub fn reaction_badge_bg(&self) -> Color32 {
        Color32::from_rgb(235, 69, 158) // Discord Fuchsia #EB459E
    }

    /// Call card background fill for missed/failed calls vs normal calls.
    pub fn call_card_bg(&self, is_missed_or_failed: bool) -> Color32 {
        if is_missed_or_failed {
            if self.is_dark {
                Color32::from_rgb(44, 29, 34)
            } else {
                Color32::from_rgb(254, 242, 242)
            }
        } else {
            self.bg_elevated
        }
    }

    /// Call card border stroke color.
    pub fn call_card_border(&self, is_missed_or_failed: bool) -> Color32 {
        if is_missed_or_failed {
            self.danger.linear_multiply(0.55)
        } else {
            self.accent_online.linear_multiply(0.55)
        }
    }

    /// Discord-style dark theme.
    pub fn dark() -> Self {
        Self {
            is_dark: true,
            bg_app: Color32::from_rgb(30, 31, 34),           // #1E1F22
            bg_sidebar: Color32::from_rgb(43, 45, 49),       // #2B2D31
            bg_chat_canvas: Color32::from_rgb(49, 51, 56),   // #313338
            bg_elevated: Color32::from_rgb(56, 58, 64),      // #383A40
            bg_hover: Color32::from_rgb(53, 55, 60),         // #35373C
            bg_selected: Color32::from_rgb(64, 66, 73),      // #404249
            bg_message_hover: Color32::from_rgb(46, 48, 53), // #2E3035
            bg_floating: Color32::from_rgb(17, 18, 20),      // #111214
            scrim: Color32::from_rgba_unmultiplied(0, 0, 0, 170),
            bg_input: Color32::from_rgb(56, 58, 64),         // #383A40
            bg_input_track: Color32::from_rgb(30, 31, 34),   // #1E1F22
            border: Color32::from_rgb(63, 65, 71),           // #3F4147
            text_primary: Color32::from_rgb(242, 243, 245),  // #F2F3F5
            text_secondary: Color32::from_rgb(196, 201, 206), // #C4C9CE
            text_muted: Color32::from_rgb(160, 166, 176),    // #A0A6B0
            primary: Color32::from_rgb(88, 101, 242),        // #5865F2 (4.61:1 with white)
            primary_hover: Color32::from_rgb(148, 156, 247), // #949CF7
            accent_online: Color32::from_rgb(52, 199, 115),  // #34C773
            danger: Color32::from_rgb(218, 55, 60),          // #DA373C (4.57:1 with white)
            danger_text: Color32::from_rgb(255, 140, 145),   // #FF8C91
            warning: Color32::from_rgb(240, 178, 50),        // #F0B232
            reaction_star: Color32::from_rgb(240, 178, 50),  // #F0B232
            code_bg: Color32::from_rgb(43, 45, 49),          // #2B2D31
            link_text: Color32::from_rgb(0, 168, 252),       // #00A8FC
            primary_fg: Color32::WHITE,
            bubble_out_bg: Color32::from_rgb(71, 82, 196),     // #4752C4
            bubble_out_text: Color32::from_rgb(242, 243, 255), // #F2F3FF
            bubble_out_meta: Color32::from_rgb(214, 218, 255), // #D6DAFF
            bubble_in_bg: Color32::from_rgb(43, 45, 49),       // #2B2D31
            bubble_in_border: Color32::from_rgb(52, 54, 59),   // #34363B
            bubble_in_text: Color32::from_rgb(242, 243, 245),  // #F2F3F5
            bubble_in_meta: Color32::from_rgb(160, 166, 176),  // #A0A6B0
        }
    }

    /// Discord-style light theme.
    pub fn light() -> Self {
        Self {
            is_dark: false,
            bg_app: Color32::from_rgb(227, 229, 232),         // #E3E5E8
            bg_sidebar: Color32::from_rgb(242, 243, 245),     // #F2F3F5
            bg_chat_canvas: Color32::WHITE,                   // #FFFFFF
            bg_elevated: Color32::WHITE,                      // #FFFFFF
            bg_hover: Color32::from_rgb(235, 237, 239),       // #EBEDEF
            bg_selected: Color32::from_rgb(222, 225, 229),    // #DEE1E5
            bg_message_hover: Color32::from_rgb(247, 247, 248), // #F7F7F8
            bg_floating: Color32::WHITE,
            scrim: Color32::from_rgba_unmultiplied(0, 0, 0, 120),
            bg_input: Color32::from_rgb(235, 237, 239),       // #EBEDEF
            bg_input_track: Color32::from_rgb(227, 229, 232), // #E3E5E8
            border: Color32::from_rgb(213, 217, 222),         // #D5D9DE
            text_primary: Color32::from_rgb(49, 51, 56),      // #313338
            text_secondary: Color32::from_rgb(78, 80, 88),    // #4E5058
            text_muted: Color32::from_rgb(92, 94, 102),       // #5C5E66
            primary: Color32::from_rgb(88, 101, 242),         // #5865F2
            primary_hover: Color32::from_rgb(71, 82, 196),    // #4752C4
            accent_online: Color32::from_rgb(26, 127, 75),    // #1A7F4B
            danger: Color32::from_rgb(200, 40, 45),           // #C8282D
            danger_text: Color32::from_rgb(200, 40, 45),      // #C8282D
            warning: Color32::from_rgb(138, 90, 0),           // #8A5A00
            reaction_star: Color32::from_rgb(138, 90, 0),     // #8A5A00
            code_bg: Color32::from_rgb(242, 243, 245),        // #F2F3F5
            link_text: Color32::from_rgb(0, 96, 214),         // #0060D6 (5.16:1 on #F2F3F5)
            primary_fg: Color32::WHITE,
            bubble_out_bg: Color32::from_rgb(224, 228, 255),   // #E0E4FF
            bubble_out_text: Color32::from_rgb(49, 51, 56),    // #313338
            bubble_out_meta: Color32::from_rgb(78, 84, 150),   // #4E5496
            bubble_in_bg: Color32::from_rgb(242, 243, 245),    // #F2F3F5
            bubble_in_border: Color32::from_rgb(227, 229, 232), // #E3E5E8
            bubble_in_text: Color32::from_rgb(49, 51, 56),     // #313338
            bubble_in_meta: Color32::from_rgb(92, 94, 102),    // #5C5E66
        }
    }

    /// Fill for the "off" state of toggles and other inactive track controls.
    pub fn toggle_off(&self) -> Color32 {
        self.text_muted.linear_multiply(0.55)
    }

    /// Install this palette as the egui style for its theme.
    ///
    /// `animation_speed` is clamped to 0.5 ..= 2.0 and `animations_enabled == false` makes every
    /// egui animation instantaneous, which the design-system widgets inherit through
    /// `Style::animation_time`.
    pub fn apply_to_ctx_with_anim(
        &self,
        ctx: &egui::Context,
        animations_enabled: bool,
        animation_speed: f32,
    ) {
        let target_theme = if self.is_dark {
            egui::Theme::Dark
        } else {
            egui::Theme::Light
        };
        let mut style = (*ctx.style_of(target_theme)).clone();
        let mut visuals = if self.is_dark {
            egui::Visuals::dark()
        } else {
            egui::Visuals::light()
        };
        visuals.dark_mode = self.is_dark;

        visuals.panel_fill = self.bg_app;
        visuals.window_fill = self.bg_elevated;
        visuals.window_stroke = Stroke::new(stroke::HAIRLINE, self.border);
        visuals.window_rounding = Rounding::same(radius::LG);
        visuals.menu_rounding = Rounding::same(radius::MD);
        visuals.window_shadow = egui::epaint::Shadow {
            offset: Vec2::new(0.0, 8.0),
            blur: 24.0,
            spread: 0.0,
            color: Color32::from_black_alpha(if self.is_dark { 120 } else { 40 }),
        };
        visuals.popup_shadow = egui::epaint::Shadow {
            offset: Vec2::new(0.0, 4.0),
            blur: 12.0,
            spread: 0.0,
            color: Color32::from_black_alpha(if self.is_dark { 100 } else { 32 }),
        };
        visuals.extreme_bg_color = self.bg_input;
        visuals.faint_bg_color = self.bg_hover;
        visuals.code_bg_color = self.code_bg;
        visuals.hyperlink_color = self.link_text;
        visuals.warn_fg_color = self.warning;
        visuals.error_fg_color = self.danger_text;
        visuals.override_text_color = Some(self.text_primary);
        visuals.selection.bg_fill =
            self.primary
                .linear_multiply(if self.is_dark { 0.45 } else { 0.22 });
        visuals.selection.stroke = Stroke::new(stroke::EMPHASIS, self.primary);

        let r = Rounding::same(radius::MD);
        // Dark controls rely on fill contrast alone; light controls keep a hairline so
        // white-on-white buttons stay visible.
        let rest_stroke = if self.is_dark {
            Stroke::NONE
        } else {
            Stroke::new(stroke::HAIRLINE, self.border)
        };
        visuals.widgets.noninteractive.weak_bg_fill = self.bg_app;
        visuals.widgets.noninteractive.bg_fill = self.bg_app;
        visuals.widgets.noninteractive.bg_stroke = Stroke::new(stroke::HAIRLINE, self.border);
        visuals.widgets.noninteractive.fg_stroke = Stroke::new(stroke::HAIRLINE, self.text_primary);
        visuals.widgets.noninteractive.rounding = r;

        visuals.widgets.inactive.bg_fill = self.bg_elevated;
        visuals.widgets.inactive.weak_bg_fill = self.bg_elevated;
        visuals.widgets.inactive.bg_stroke = rest_stroke;
        visuals.widgets.inactive.fg_stroke = Stroke::new(stroke::HAIRLINE, self.text_primary);
        visuals.widgets.inactive.rounding = r;

        visuals.widgets.hovered.bg_fill = self.bg_hover;
        visuals.widgets.hovered.weak_bg_fill = self.bg_hover;
        visuals.widgets.hovered.bg_stroke = rest_stroke;
        visuals.widgets.hovered.fg_stroke = Stroke::new(stroke::HAIRLINE, self.text_primary);
        visuals.widgets.hovered.rounding = r;

        visuals.widgets.active.bg_fill = self.primary;
        visuals.widgets.active.weak_bg_fill = self.primary;
        visuals.widgets.active.bg_stroke = Stroke::new(stroke::EMPHASIS, self.primary);
        visuals.widgets.active.fg_stroke = Stroke::new(stroke::EMPHASIS, self.primary_fg);
        visuals.widgets.active.rounding = r;

        visuals.widgets.open.bg_fill = self.bg_selected;
        visuals.widgets.open.weak_bg_fill = self.bg_selected;
        visuals.widgets.open.rounding = r;

        style.spacing.item_spacing = Vec2::new(8.0, 6.0);
        style.spacing.button_padding = Vec2::new(10.0, 6.0);
        style.spacing.scroll = egui::style::ScrollStyle::solid();
        style.spacing.scroll.bar_width = 6.0;
        style.animation_time = if animations_enabled {
            (0.12 / animation_speed.clamp(0.5, 2.0)).clamp(0.03, 0.35)
        } else {
            0.0
        };
        style.text_styles.insert(
            egui::TextStyle::Heading,
            egui::FontId::proportional(super::typography::TextRole::Display.size()),
        );
        style.text_styles.insert(
            egui::TextStyle::Body,
            super::typography::TextRole::Body.font_id(),
        );
        style.text_styles.insert(
            egui::TextStyle::Button,
            egui::FontId::proportional(13.5),
        );
        style.text_styles.insert(
            egui::TextStyle::Small,
            super::typography::TextRole::Caption.font_id(),
        );
        style.text_styles.insert(
            egui::TextStyle::Monospace,
            super::typography::TextRole::Mono.font_id(),
        );
        style.visuals = visuals;

        // Bind the active egui::Theme, its style slot and the current style so an OS theme change
        // or persisted state never reverts widgets to the stock visuals.
        ctx.set_theme(target_theme);
        ctx.set_style_of(target_theme, style.clone());
        ctx.set_style(style);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_themes_meet_wcag_aa_for_text_pairs() {
        for pal in [ThemePalette::dark(), ThemePalette::light()] {
            let pairs = [
                ("primary_fg/primary", pal.primary_fg, pal.primary),
                ("primary_fg/danger", Color32::WHITE, pal.danger),
                ("text_primary/bg_app", pal.text_primary, pal.bg_app),
                ("text_primary/bg_sidebar", pal.text_primary, pal.bg_sidebar),
                ("text_primary/canvas", pal.text_primary, pal.bg_chat_canvas),
                ("text_primary/elevated", pal.text_primary, pal.bg_elevated),
                ("text_primary/selected", pal.text_primary, pal.bg_selected),
                ("text_secondary/canvas", pal.text_secondary, pal.bg_chat_canvas),
                ("text_secondary/sidebar", pal.text_secondary, pal.bg_sidebar),
                ("text_muted/canvas", pal.text_muted, pal.bg_chat_canvas),
                ("text_muted/sidebar", pal.text_muted, pal.bg_sidebar),
                ("text_muted/elevated", pal.text_muted, pal.bg_elevated),
                ("text_muted/input", pal.text_muted, pal.bg_input),
                ("accent_online/elevated", pal.accent_online, pal.bg_elevated),
                ("danger_text/elevated", pal.danger_text, pal.bg_elevated),
                ("danger_text/canvas", pal.danger_text, pal.bg_chat_canvas),
                ("warning/elevated", pal.warning, pal.bg_elevated),
                ("link/canvas", pal.link_text, pal.bg_chat_canvas),
                ("link/sidebar", pal.link_text, pal.bg_sidebar),
                ("bubble_out_text", pal.bubble_out_text, pal.bubble_out_bg),
                ("bubble_out_meta", pal.bubble_out_meta, pal.bubble_out_bg),
                ("bubble_in_text", pal.bubble_in_text, pal.bubble_in_bg),
                ("bubble_in_meta", pal.bubble_in_meta, pal.bubble_in_bg),
            ];
            for (name, fg, bg) in pairs {
                let ratio = contrast_ratio(fg, bg);
                assert!(
                    ratio >= 4.5,
                    "dark={} {name}: contrast {ratio:.2} < 4.5",
                    pal.is_dark
                );
            }
        }
    }

    #[test]
    fn dark_layers_get_progressively_lighter() {
        let p = ThemePalette::dark();
        let l = |c| relative_luminance(c);
        assert!(l(p.bg_app) < l(p.bg_sidebar));
        assert!(l(p.bg_sidebar) < l(p.bg_chat_canvas));
        assert!(l(p.bg_chat_canvas) < l(p.bg_elevated));
    }

    #[test]
    fn disabling_animations_zeroes_style_animation_time() {
        let ctx = egui::Context::default();
        ThemePalette::dark().apply_to_ctx_with_anim(&ctx, false, 1.0);
        assert_eq!(ctx.style().animation_time, 0.0);
        ThemePalette::dark().apply_to_ctx_with_anim(&ctx, true, 1.0);
        assert!(ctx.style().animation_time > 0.0);
    }
}
