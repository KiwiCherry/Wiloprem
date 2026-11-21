//! Stateless, reusable controls painted from a [`ThemePalette`].
//!
//! Widgets receive plain values, return an [`egui::Response`] (or a tiny result struct) and never
//! reach into application state, the worker channel or the database. Callers translate the
//! response into a `UiAction`.

use eframe::egui::{
    self, Align, Align2, Color32, CursorIcon, FontId, Id, Layout, Order, Pos2, Rect, Response,
    Rounding, Sense, Stroke, Ui, Vec2,
};

use super::icons::lucide;
use super::palette::ThemePalette;
use super::tokens::{layout, radius, size, space, stroke};
use super::typography::TextRole;

/// Paint a Lucide glyph centred on `center`.
pub fn paint_icon(painter: &egui::Painter, center: Pos2, glyph: &str, size: f32, color: Color32) {
    painter.text(
        center,
        Align2::CENTER_CENTER,
        glyph,
        FontId::proportional(size),
        color,
    );
}

/// Smoothly animated `0.0 ..= 1.0` hover/active factor that honours the user animation setting.
pub fn state_factor(ui: &Ui, id: Id, on: bool) -> f32 {
    ui.ctx()
        .animate_bool_with_time(id, on, ui.style().animation_time.max(0.0))
}

fn mix(a: Color32, b: Color32, t: f32) -> Color32 {
    a.lerp_to_gamma(b, t.clamp(0.0, 1.0))
}

/// Square icon button used in headers and toolbars.
pub fn icon_button(
    ui: &mut Ui,
    pal: &ThemePalette,
    glyph: &str,
    tooltip: &str,
    active: bool,
) -> Response {
    let (rect, resp) = ui.allocate_exact_size(Vec2::splat(size::CONTROL_H), Sense::click());
    let hover = state_factor(ui, resp.id.with("hover"), resp.hovered());
    let painter = ui.painter();
    let fill = if active {
        pal.bg_selected
    } else {
        mix(Color32::TRANSPARENT, pal.bg_hover, hover)
    };
    painter.rect_filled(rect, Rounding::same(radius::MD), fill);
    let fg = if active {
        pal.text_primary
    } else {
        mix(pal.text_secondary, pal.text_primary, hover)
    };
    paint_icon(painter, rect.center(), glyph, size::ICON_MD, fg);
    resp.on_hover_cursor(CursorIcon::PointingHand)
        .on_hover_text(tooltip)
}

/// Red unread pill anchored on `center`, ringed in `ring` so it cuts out of its parent.
pub fn count_badge(painter: &egui::Painter, pal: &ThemePalette, center: Pos2, count: usize, ring: Color32) {
    if count == 0 {
        return;
    }
    let label = if count > 99 {
        "99+".to_string()
    } else {
        count.to_string()
    };
    let width = match label.len() {
        1 => size::BADGE_H,
        2 => size::BADGE_H + 4.0,
        _ => size::BADGE_H + 10.0,
    };
    let rect = Rect::from_center_size(center, Vec2::new(width, size::BADGE_H));
    painter.rect_filled(
        rect.expand(stroke::EMPHASIS),
        Rounding::same(radius::FULL),
        ring,
    );
    painter.rect_filled(rect, Rounding::same(radius::FULL), pal.danger);
    painter.text(
        rect.center(),
        Align2::CENTER_CENTER,
        label,
        FontId::proportional(10.0),
        Color32::WHITE,
    );
}

/// Presence dot with a ring that cuts it out of an avatar.
pub fn status_dot(painter: &egui::Painter, center: Pos2, radius: f32, color: Color32, ring: Color32) {
    painter.circle_filled(center, radius + 2.0, ring);
    painter.circle_filled(center, radius, color);
}

/// One entry of the navigation rail.
pub struct NavRailItem<'a> {
    pub glyph: &'a str,
    pub tooltip: &'a str,
    pub selected: bool,
    pub badge: usize,
}

/// Discord-style rail button: the shape morphs from a circle to a rounded square and turns
/// accent-colored when hovered or selected, with a pill indicator on the left edge.
pub fn nav_rail_item(ui: &mut Ui, pal: &ThemePalette, rail_w: f32, item: &NavRailItem<'_>) -> Response {
    let row_h = layout::RAIL_ITEM + space::SM;
    let (row, resp) = ui.allocate_exact_size(Vec2::new(rail_w, row_h), Sense::click());
    let hover = state_factor(ui, resp.id.with("hover"), resp.hovered());
    let selected = state_factor(ui, resp.id.with("selected"), item.selected);
    let emphasis = hover.max(selected);

    let button = Rect::from_center_size(row.center(), Vec2::splat(layout::RAIL_ITEM));
    let corner = layout::RAIL_ITEM * 0.5 + (radius::LG - layout::RAIL_ITEM * 0.5) * emphasis;
    let painter = ui.painter();
    painter.rect_filled(
        button,
        Rounding::same(corner),
        mix(pal.bg_sidebar, pal.primary, emphasis),
    );
    paint_icon(
        painter,
        button.center(),
        item.glyph,
        size::ICON_LG,
        mix(pal.text_secondary, pal.primary_fg, emphasis),
    );

    let pill_h = 8.0 + 28.0 * selected + 12.0 * hover * (1.0 - selected);
    let pill = Rect::from_center_size(
        Pos2::new(row.left() + space::XXS, row.center().y),
        Vec2::new(space::XS, pill_h),
    );
    painter.rect_filled(
        pill,
        Rounding {
            nw: 0.0,
            sw: 0.0,
            ne: radius::XS,
            se: radius::XS,
        },
        pal.text_primary.linear_multiply(emphasis.max(0.0)),
    );

    count_badge(
        painter,
        pal,
        Pos2::new(button.right() - 2.0, button.bottom() - 2.0),
        item.badge,
        pal.bg_app,
    );
    resp.on_hover_cursor(CursorIcon::PointingHand)
        .on_hover_text(item.tooltip)
}

/// Thin separator between rail groups.
pub fn rail_separator(ui: &mut Ui, pal: &ThemePalette, rail_w: f32) {
    let (row, _) = ui.allocate_exact_size(Vec2::new(rail_w, space::MD), Sense::hover());
    let line = Rect::from_center_size(row.center(), Vec2::new(layout::RAIL_ITEM * 0.6, 2.0));
    ui.painter()
        .rect_filled(line, Rounding::same(1.0), pal.border);
}

/// Result of [`search_field`].
pub struct SearchFieldOutput {
    pub response: Response,
    /// The clear button was pressed this frame; the field text has already been emptied.
    pub cleared: bool,
}

/// Pill-shaped search input with a leading magnifier and a trailing clear button.
pub fn search_field(
    ui: &mut Ui,
    pal: &ThemePalette,
    id: Id,
    text: &mut String,
    hint: &str,
    width: f32,
) -> SearchFieldOutput {
    let (outer, _) = ui.allocate_exact_size(Vec2::new(width, size::CONTROL_H), Sense::hover());
    let has_text = !text.is_empty();
    let lead = 30.0;
    let trail = if has_text { 28.0 } else { space::MD };
    let edit_rect = Rect::from_min_max(
        Pos2::new(outer.left() + lead, outer.top() + space::XS),
        Pos2::new(
            (outer.right() - trail).max(outer.left() + lead + 24.0),
            outer.bottom() - space::XS,
        ),
    );

    // The field is painted before the editor so the TextEdit renders above the fill and the
    // focus ring is drawn afterwards from its response.
    ui.painter()
        .rect_filled(outer, Rounding::same(radius::MD), pal.bg_input_track);

    let mut edit_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(edit_rect)
            .layout(Layout::left_to_right(Align::Center)),
    );
    edit_ui.set_clip_rect(edit_rect.intersect(ui.clip_rect()));
    let response = edit_ui.add(
        egui::TextEdit::singleline(text)
            .id(id)
            .frame(false)
            .clip_text(true)
            .hint_text(TextRole::Body.rich(hint, pal.text_muted))
            .desired_width(edit_rect.width()),
    );

    let focused = response.has_focus();
    if focused {
        ui.painter().rect_stroke(
            outer,
            Rounding::same(radius::MD),
            Stroke::new(stroke::EMPHASIS, pal.primary),
        );
    }
    paint_icon(
        ui.painter(),
        Pos2::new(outer.left() + 16.0, outer.center().y),
        lucide::SEARCH,
        size::ICON_SM,
        if focused { pal.text_primary } else { pal.text_muted },
    );

    let mut cleared = false;
    if has_text {
        let clear_rect = Rect::from_center_size(
            Pos2::new(outer.right() - 15.0, outer.center().y),
            Vec2::splat(20.0),
        );
        let clear = ui.interact(clear_rect, id.with("clear"), Sense::click());
        if clear.hovered() {
            ui.painter()
                .circle_filled(clear_rect.center(), 9.0, pal.bg_hover);
        }
        paint_icon(
            ui.painter(),
            clear_rect.center(),
            lucide::X,
            12.0,
            if clear.hovered() { pal.text_primary } else { pal.text_muted },
        );
        if clear
            .on_hover_cursor(CursorIcon::PointingHand)
            .on_hover_text("XÃ³a tÃ¬m kiáº¿m")
            .clicked()
        {
            text.clear();
            cleared = true;
        }
    }
    SearchFieldOutput { response, cleared }
}

/// Rounded filter chip (e.g. "Táº¥t cáº£", "CÃ¡ nhÃ¢n", "NhÃ³m") with an optional unread count.
pub fn filter_chip(
    ui: &mut Ui,
    pal: &ThemePalette,
    label: &str,
    active: bool,
    badge: usize,
) -> Response {
    let font = TextRole::Label.font_id();
    let galley = ui
        .painter()
        .layout_no_wrap(label.to_owned(), font.clone(), pal.text_primary);
    let badge_w = if badge > 0 { 22.0 } else { 0.0 };
    let width = galley.size().x + space::LG + badge_w;
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(width, size::CONTROL_H_SM - 2.0), Sense::click());
    let hover = state_factor(ui, resp.id.with("hover"), resp.hovered());
    let active_f = state_factor(ui, resp.id.with("active"), active);
    let fill = mix(
        mix(Color32::TRANSPARENT, pal.bg_hover, hover),
        pal.bg_selected,
        active_f,
    );
    let painter = ui.painter();
    painter.rect_filled(rect, Rounding::same(radius::SM), fill);
    let fg = mix(
        mix(pal.text_secondary, pal.text_primary, hover),
        pal.text_primary,
        active_f,
    );
    let text_x = rect.left() + space::SM + galley.size().x * 0.5;
    painter.text(
        Pos2::new(text_x, rect.center().y),
        Align2::CENTER_CENTER,
        label,
        font,
        fg,
    );
    if badge > 0 {
        let center = Pos2::new(rect.right() - space::SM - 5.0, rect.center().y);
        count_badge(painter, pal, center, badge, fill);
    }
    resp.on_hover_cursor(CursorIcon::PointingHand)
}

/// Visual weight of a [`button`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ButtonKind {
    Primary,
    Secondary,
    Danger,
    /// Text-only, for low-emphasis actions.
    Ghost,
}

/// Standard push button. Disabled buttons ignore clicks and render muted.
pub fn button(ui: &mut Ui, pal: &ThemePalette, kind: ButtonKind, label: &str, enabled: bool) -> Response {
    let font = FontId::proportional(TextRole::Subtitle.size());
    let galley = ui
        .painter()
        .layout_no_wrap(label.to_owned(), font.clone(), pal.text_primary);
    let size = Vec2::new(galley.size().x + space::XL, size::CONTROL_H);
    let sense = if enabled { Sense::click() } else { Sense::hover() };
    let (rect, resp) = ui.allocate_exact_size(size, sense);
    let hover = if enabled {
        state_factor(ui, resp.id.with("hover"), resp.hovered())
    } else {
        0.0
    };
    let (base, hot, fg) = match kind {
        ButtonKind::Primary => (pal.primary, pal.primary_hover_fill(), pal.primary_fg),
        ButtonKind::Secondary => (pal.bg_elevated, pal.bg_selected, pal.text_primary),
        ButtonKind::Danger => (pal.danger, pal.danger.gamma_multiply(0.85), Color32::WHITE),
        ButtonKind::Ghost => (Color32::TRANSPARENT, pal.bg_hover, pal.link_text),
    };
    let fill = mix(base, hot, hover);
    let (fill, fg) = if enabled {
        (fill, fg)
    } else {
        (fill.linear_multiply(0.5), fg.linear_multiply(0.6))
    };
    let painter = ui.painter();
    painter.rect_filled(rect, Rounding::same(radius::MD), fill);
    painter.text(rect.center(), Align2::CENTER_CENTER, label, font, fg);
    if enabled {
        resp.on_hover_cursor(CursorIcon::PointingHand)
    } else {
        resp
    }
}

impl ThemePalette {
    /// Hover fill for primary buttons: lighter in dark mode, deeper in light mode.
    pub fn primary_hover_fill(&self) -> Color32 {
        if self.is_dark {
            self.primary.gamma_multiply(0.88)
        } else {
            self.primary_hover
        }
    }
}

/// Pill switch. Flips `value` on click and marks the response as changed.
pub fn toggle_switch(ui: &mut Ui, pal: &ThemePalette, value: &mut bool) -> Response {
    let (rect, mut resp) = ui.allocate_exact_size(Vec2::new(40.0, 22.0), Sense::click());
    if resp.clicked() {
        *value = !*value;
        resp.mark_changed();
    }
    let t = state_factor(ui, resp.id.with("on"), *value);
    let track = mix(pal.toggle_off(), pal.accent_online, t);
    let painter = ui.painter();
    painter.rect_filled(rect, Rounding::same(radius::FULL), track);
    let knob_r = 8.0;
    let travel = rect.width() - 2.0 * (knob_r + 3.0);
    painter.circle_filled(
        Pos2::new(rect.left() + knob_r + 3.0 + travel * t, rect.center().y),
        knob_r,
        Color32::WHITE,
    );
    resp.on_hover_cursor(CursorIcon::PointingHand)
}

/// Frame shared by single-line inputs and the message composer.
pub fn input_frame(pal: &ThemePalette, focused: bool) -> egui::Frame {
    egui::Frame::none()
        .fill(pal.bg_input)
        .stroke(Stroke::new(
            stroke::HAIRLINE,
            if focused { pal.primary } else { Color32::TRANSPARENT },
        ))
        .rounding(Rounding::same(radius::MD))
        .inner_margin(egui::Margin::symmetric(space::MD, space::SM))
}

/// Uppercase section label ("TIN NHáº®N", "NHÃ“M") used above list groups.
pub fn section_label(ui: &mut Ui, pal: &ThemePalette, text: &str) {
    ui.label(TextRole::Caption.rich(text.to_uppercase(), pal.text_muted).strong());
}

/// Full-width 1px divider.
pub fn hairline(ui: &mut Ui, pal: &ThemePalette) {
    let (rect, _) = ui.allocate_exact_size(
        Vec2::new(ui.available_width(), stroke::HAIRLINE),
        Sense::hover(),
    );
    ui.painter().rect_filled(rect, Rounding::ZERO, pal.border);
}

/// Frame for modals and popovers.
pub fn modal_frame(pal: &ThemePalette) -> egui::Frame {
    egui::Frame::none()
        .fill(pal.bg_elevated)
        .stroke(Stroke::new(stroke::HAIRLINE, pal.border))
        .rounding(Rounding::same(radius::LG))
        .inner_margin(egui::Margin::same(space::XL))
        .shadow(egui::epaint::Shadow {
            offset: Vec2::new(0.0, 12.0),
            blur: 32.0,
            spread: 0.0,
            color: Color32::from_black_alpha(if pal.is_dark { 140 } else { 50 }),
        })
}

/// Which way a modal was dismissed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModalClose {
    /// X button, Escape key or backdrop click.
    Dismissed,
}

/// Show a centred modal with a dimmed, click-to-dismiss backdrop and a unified header.
///
/// Returns `Some` when the user dismissed it this frame so the caller can emit a close action.
/// `body` only receives the `Ui`; it must not capture the modal's own open flag.
pub fn show_modal(
    ctx: &egui::Context,
    pal: &ThemePalette,
    id: &str,
    title: &str,
    width: f32,
    body: impl FnOnce(&mut Ui),
) -> Option<ModalClose> {
    let mut closed = false;

    let screen = ctx.screen_rect();
    let scrim = egui::Area::new(Id::new((id, "scrim")))
        .order(Order::Middle)
        .fixed_pos(screen.min)
        .show(ctx, |ui| {
            let (rect, resp) = ui.allocate_exact_size(screen.size(), Sense::click());
            ui.painter().rect_filled(rect, Rounding::ZERO, pal.scrim);
            resp
        });
    if scrim.inner.clicked() {
        closed = true;
    }

    egui::Window::new(title)
        .id(Id::new((id, "window")))
        .order(Order::Foreground)
        .title_bar(false)
        .collapsible(false)
        .resizable(false)
        .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
        .fixed_size(Vec2::new(width, 0.0))
        .frame(modal_frame(pal))
        .show(ctx, |ui| {
            ui.set_width(width - 2.0 * space::XL);
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(title)
                        .size(TextRole::Title.size())
                        .color(pal.text_primary)
                        .strong(),
                );
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if icon_button(ui, pal, lucide::X, "ÄÃ³ng", false).clicked() {
                        closed = true;
                    }
                });
            });
            ui.add_space(space::MD);
            body(ui);
        });

    if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
        closed = true;
    }
    closed.then_some(ModalClose::Dismissed)
}

/// Whether `mode_w` falls in the compact or narrow responsive band.
pub fn responsive_band(width: f32) -> ResponsiveBand {
    if width < layout::BREAKPOINT_NARROW {
        ResponsiveBand::Narrow
    } else if width < layout::BREAKPOINT_COMPACT {
        ResponsiveBand::Compact
    } else {
        ResponsiveBand::Wide
    }
}

/// Width class of the window.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponsiveBand {
    Narrow,
    Compact,
    Wide,
}

impl ResponsiveBand {
    pub fn rail_width(self) -> f32 {
        match self {
            Self::Narrow => layout::RAIL_W_NARROW,
            _ => layout::RAIL_W,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn responsive_bands_follow_breakpoints() {
        assert_eq!(responsive_band(480.0), ResponsiveBand::Narrow);
        assert_eq!(responsive_band(700.0), ResponsiveBand::Compact);
        assert_eq!(responsive_band(1020.0), ResponsiveBand::Wide);
        assert_eq!(responsive_band(layout::BREAKPOINT_NARROW), ResponsiveBand::Compact);
        assert_eq!(responsive_band(layout::BREAKPOINT_COMPACT), ResponsiveBand::Wide);
    }

    #[test]
    fn rail_is_narrower_on_small_windows() {
        assert!(ResponsiveBand::Narrow.rail_width() < ResponsiveBand::Wide.rail_width());
    }
}
