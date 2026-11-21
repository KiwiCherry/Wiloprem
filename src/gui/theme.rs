use super::*;

pub(super) use super::design_system::palette::{contrast_ratio, ThemePalette};
pub use super::design_system::icons::lucide;

impl ThemePalette {
    /// Load either the built-in dark/light palette or apply custom overrides from `theme.json` (#7).
    pub(super) fn load_with_custom_override(dark_mode: bool) -> Self {
        let mut base = if dark_mode {
            Self::dark()
        } else {
            Self::light()
        };
        if let Ok(raw) = std::fs::read_to_string("theme.json") {
            base.apply_json_overrides(&raw);
        }
        base
    }

    /// Apply any `#RRGGBB` color overrides from a JSON string (`theme.json`) (#7).
    pub(super) fn apply_json_overrides(&mut self, json_text: &str) {
        let Ok(val) = serde_json::from_str::<serde_json::Value>(json_text) else {
            return;
        };
        let sub = if self.is_dark {
            val.get("dark").unwrap_or(&val)
        } else {
            val.get("light").unwrap_or(&val)
        };
        for (key, target) in [
            ("bg_app", &mut self.bg_app),
            ("bg_sidebar", &mut self.bg_sidebar),
            ("bg_chat_canvas", &mut self.bg_chat_canvas),
            ("bg_elevated", &mut self.bg_elevated),
            ("bg_hover", &mut self.bg_hover),
            ("bg_selected", &mut self.bg_selected),
            ("bg_message_hover", &mut self.bg_message_hover),
            ("bg_floating", &mut self.bg_floating),
            ("scrim", &mut self.scrim),
            ("bg_input", &mut self.bg_input),
            ("bg_input_track", &mut self.bg_input_track),
            ("border", &mut self.border),
            ("text_primary", &mut self.text_primary),
            ("text_secondary", &mut self.text_secondary),
            ("text_muted", &mut self.text_muted),
            ("primary", &mut self.primary),
            ("primary_hover", &mut self.primary_hover),
            ("accent_online", &mut self.accent_online),
            ("danger", &mut self.danger),
            ("danger_text", &mut self.danger_text),
            ("warning", &mut self.warning),
            ("reaction_star", &mut self.reaction_star),
            ("code_bg", &mut self.code_bg),
            ("link_text", &mut self.link_text),
            ("bubble_out_bg", &mut self.bubble_out_bg),
            ("bubble_out_text", &mut self.bubble_out_text),
            ("bubble_out_meta", &mut self.bubble_out_meta),
            ("bubble_in_bg", &mut self.bubble_in_bg),
            ("bubble_in_border", &mut self.bubble_in_border),
            ("bubble_in_text", &mut self.bubble_in_text),
            ("bubble_in_meta", &mut self.bubble_in_meta),
        ] {
            if let Some(hex) = sub.get(key).and_then(|v| v.as_str()) {
                if let Some(c) = parse_hex_color(hex) {
                    *target = c;
                }
            }
        }
    }

    /// Export a sample `theme.json` containing both `dark` and `light` token maps (#7).
    pub(super) fn export_template(path: &str) -> std::io::Result<()> {
        let d = Self::dark();
        let l = Self::light();
        let to_map = |p: &Self| {
            serde_json::json!({
                "bg_app": format_hex_color(p.bg_app),
                "bg_sidebar": format_hex_color(p.bg_sidebar),
                "bg_chat_canvas": format_hex_color(p.bg_chat_canvas),
                "bg_elevated": format_hex_color(p.bg_elevated),
                "bg_hover": format_hex_color(p.bg_hover),
                "bg_selected": format_hex_color(p.bg_selected),
                "bg_message_hover": format_hex_color(p.bg_message_hover),
                "bg_floating": format_hex_color(p.bg_floating),
                "scrim": format_hex_color(p.scrim),
                "bg_input": format_hex_color(p.bg_input),
                "bg_input_track": format_hex_color(p.bg_input_track),
                "border": format_hex_color(p.border),
                "text_primary": format_hex_color(p.text_primary),
                "text_secondary": format_hex_color(p.text_secondary),
                "text_muted": format_hex_color(p.text_muted),
                "primary": format_hex_color(p.primary),
                "primary_hover": format_hex_color(p.primary_hover),
                "accent_online": format_hex_color(p.accent_online),
                "danger": format_hex_color(p.danger),
                "danger_text": format_hex_color(p.danger_text),
                "warning": format_hex_color(p.warning),
                "reaction_star": format_hex_color(p.reaction_star),
                "code_bg": format_hex_color(p.code_bg),
                "link_text": format_hex_color(p.link_text),
                "bubble_out_bg": format_hex_color(p.bubble_out_bg),
                "bubble_out_text": format_hex_color(p.bubble_out_text),
                "bubble_out_meta": format_hex_color(p.bubble_out_meta),
                "bubble_in_bg": format_hex_color(p.bubble_in_bg),
                "bubble_in_border": format_hex_color(p.bubble_in_border),
                "bubble_in_text": format_hex_color(p.bubble_in_text),
                "bubble_in_meta": format_hex_color(p.bubble_in_meta),
            })
        };
        let doc = serde_json::json!({
            "_comment": "Wiloprem custom theme file (#7). Edit hex colors (#RRGGBB) under dark/light and reload in Settings.",
            "dark": to_map(&d),
            "light": to_map(&l),
        });
        let text = serde_json::to_string_pretty(&doc).unwrap_or_default();
        std::fs::write(path, text)
    }

    /// Apply custom color overrides (primary, accent, outbound/inbound bubble backgrounds) (#7, R2).
    pub(super) fn apply_custom_colors(
        &mut self,
        primary_hex: Option<&str>,
        accent_hex: Option<&str>,
        bubble_out_hex: Option<&str>,
        bubble_in_hex: Option<&str>,
    ) {
        let blend_over = |fg: Color32, bg: Color32| -> Color32 {
            if fg.a() == u8::MAX {
                return fg;
            }
            let a = fg.a() as f32 / 255.0;
            let mix = |f: u8, b: u8| -> u8 {
                ((f as f32 * a) + (b as f32 * (1.0 - a))).round().clamp(0.0, 255.0) as u8
            };
            Color32::from_rgb(mix(fg.r(), bg.r()), mix(fg.g(), bg.g()), mix(fg.b(), bg.b()))
        };

        if let Some(hex) = primary_hex {
            if let Some(c) = parse_hex_color(hex).filter(|c| c.a() > 0) {
                let effective = blend_over(c, self.bg_app);
                self.primary = c;
                self.primary_hover = Color32::from_rgba_unmultiplied(
                    c.r().saturating_add(25),
                    c.g().saturating_add(25),
                    c.b().saturating_add(25),
                    c.a(),
                );
                self.primary_fg = if contrast_ratio(Color32::WHITE, effective)
                    >= contrast_ratio(Color32::from_rgb(15, 23, 42), effective)
                {
                    Color32::WHITE
                } else {
                    Color32::from_rgb(15, 23, 42)
                };
            }
        }
        if let Some(hex) = accent_hex {
            if let Some(c) = parse_hex_color(hex).filter(|c| c.a() > 0) {
                self.accent_online = c;
            }
        }
        if let Some(hex) = bubble_out_hex {
            if let Some(c) = parse_hex_color(hex).filter(|c| c.a() > 0) {
                let effective = blend_over(c, self.bg_chat_canvas);
                self.bubble_out_bg = c;
                let dark_text = Color32::from_rgb(15, 23, 42);
                let light_text = Color32::from_rgb(235, 240, 246);
                self.bubble_out_text =
                    if contrast_ratio(dark_text, effective) >= contrast_ratio(light_text, effective)
                    {
                        dark_text
                    } else {
                        light_text
                    };

                let dark_meta = Color32::from_rgb(67, 88, 117);
                let light_meta = Color32::from_rgb(157, 178, 204);
                self.bubble_out_meta =
                    if contrast_ratio(dark_meta, effective) >= contrast_ratio(light_meta, effective)
                    {
                        dark_meta
                    } else {
                        light_meta
                    };
            }
        }
        if let Some(hex) = bubble_in_hex {
            if let Some(c) = parse_hex_color(hex).filter(|c| c.a() > 0) {
                let effective = blend_over(c, self.bg_chat_canvas);
                self.bubble_in_bg = c;
                let dark_text = Color32::from_rgb(15, 23, 42);
                let light_text = Color32::from_rgb(230, 233, 238);
                self.bubble_in_text =
                    if contrast_ratio(dark_text, effective) >= contrast_ratio(light_text, effective)
                    {
                        dark_text
                    } else {
                        light_text
                    };

                let dark_meta = Color32::from_rgb(92, 100, 112);
                let light_meta = Color32::from_rgb(150, 159, 174);
                self.bubble_in_meta =
                    if contrast_ratio(dark_meta, effective) >= contrast_ratio(light_meta, effective)
                    {
                        dark_meta
                    } else {
                        light_meta
                    };
            }
        }
    }
}

/// Animate a scalar `0.0..=1.0` smoothly when animations are enabled, or jump immediately when disabled.
pub(super) fn animate_value_scaled(
    ctx: &egui::Context,
    id: egui::Id,
    target: f32,
    base_secs: f32,
    animations_enabled: bool,
    animation_speed: f32,
) -> f32 {
    if !animations_enabled {
        ctx.animate_value_with_time(id, target, 0.0)
    } else {
        let speed = if animation_speed.is_finite() {
            animation_speed.clamp(0.5, 2.0)
        } else {
            1.0
        };
        let duration = (base_secs / speed).clamp(0.02, 0.45);
        ctx.animate_value_with_time(id, target, duration)
    }
}

/// Animate a boolean transition (`0.0` / `1.0`) respecting user animation settings.
pub(super) fn animate_bool_scaled(
    ctx: &egui::Context,
    id: egui::Id,
    value: bool,
    base_secs: f32,
    animations_enabled: bool,
    animation_speed: f32,
) -> f32 {
    animate_value_scaled(
        ctx,
        id,
        if value { 1.0 } else { 0.0 },
        base_secs,
        animations_enabled,
        animation_speed,
    )
}

/// Shared styled `egui::Frame` wrapper for standard text inputs (`Radius-Ladder` 8px).
pub(super) fn styled_input_frame(pal: &ThemePalette, focused: bool) -> egui::Frame {
    super::design_system::widgets::input_frame(pal, focused)
}

/// Font family identifier for SemiBold headings/labels (backed by `BeVietnamPro-SemiBold.ttf`).
pub(super) fn font_family_semibold() -> egui::FontFamily {
    egui::FontFamily::Name("semibold".into())
}

pub(super) fn parse_hex_color(s: &str) -> Option<Color32> {
    let hex = s.trim().trim_start_matches('#');
    if hex.len() == 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        Some(Color32::from_rgb(r, g, b))
    } else if hex.len() == 8 {
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
        Some(Color32::from_rgba_unmultiplied(r, g, b, a))
    } else {
        None
    }
}

pub(super) fn format_hex_color(c: Color32) -> String {
    let [r, g, b, a] = c.to_srgba_unmultiplied();
    if a == 255 {
        format!("#{:02X}{:02X}{:02X}", r, g, b)
    } else {
        format!("#{:02X}{:02X}{:02X}{:02X}", r, g, b, a)
    }
}

// ---------------------------------------------------------------------------
// Zero-allocation Vector Icon Painter Primitives (Replaces Emoji Icons)
// ---------------------------------------------------------------------------

pub(super) fn avatar_color_for(name: &str, is_group: bool) -> Color32 {
    const PALETTE: &[Color32] = &[
        Color32::from_rgb(64, 120, 192), // Atom Blue (#4078C0)
        Color32::from_rgb(56, 148, 160), // Atom Cyan (#3894A0)
        Color32::from_rgb(152, 88, 178), // Atom Purple (#9858B2)
        Color32::from_rgb(190, 80, 70),  // Atom Red (#BE5046)
        Color32::from_rgb(184, 118, 62), // Atom Orange (#B8763E)
        Color32::from_rgb(98, 148, 72),  // Atom Green (#629448)
        Color32::from_rgb(82, 116, 196), // Atom Indigo
        Color32::from_rgb(72, 138, 182), // Atom Cerulean
    ];
    let mut hash: usize = if is_group { 3 } else { 0 };
    for b in name.bytes() {
        hash = hash.wrapping_mul(31).wrapping_add(b as usize);
    }
    PALETTE[hash % PALETTE.len()]
}

pub(super) fn truncate_chars<'a>(s: &'a str, max_chars: usize) -> std::borrow::Cow<'a, str> {
    if let Some((byte_idx, _)) = s.char_indices().nth(max_chars) {
        std::borrow::Cow::Owned(format!("{}…", &s[..byte_idx]))
    } else {
        std::borrow::Cow::Borrowed(s)
    }
}

/// Extract up to 2 uppercase alphanumeric initials from a display name (e.g. `"Nguyễn An"` -> `"NA"`, `"Hoang"` -> `"H"`).
pub(super) fn avatar_initials(name: &str) -> String {
    let clean_words: Vec<&str> = name
        .split_whitespace()
        .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()))
        .filter(|w| !w.is_empty())
        .collect();
    if clean_words.len() >= 2 {
        let c1 = clean_words[0].chars().next().unwrap_or('Z').to_uppercase();
        let c2 = clean_words[clean_words.len() - 1]
            .chars()
            .next()
            .unwrap_or('U')
            .to_uppercase();
        format!("{c1}{c2}")
    } else if let Some(first) = clean_words.first() {
        first
            .chars()
            .next()
            .unwrap_or('Z')
            .to_uppercase()
            .to_string()
    } else {
        "Z".to_string()
    }
}

/// Fold Vietnamese text to lowercase ASCII for diacritic-insensitive search
/// (e.g. `"Nguyễn Văn An"` -> `"nguyen van an"`).
pub(super) fn fold_vn_search(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for raw in s.chars() {
        for c in raw.to_lowercase() {
            let mapped = match c {
                'à' | 'á' | 'ạ' | 'ả' | 'ã' | 'â' | 'ầ' | 'ấ' | 'ậ' | 'ẩ' | 'ẫ' | 'ă' | 'ằ'
                | 'ắ' | 'ặ' | 'ẳ' | 'ẵ' => 'a',
                'è' | 'é' | 'ẹ' | 'ẻ' | 'ẽ' | 'ê' | 'ề' | 'ế' | 'ệ' | 'ể' | 'ễ' => {
                    'e'
                }
                'ì' | 'í' | 'ị' | 'ỉ' | 'ĩ' => 'i',
                'ò' | 'ó' | 'ọ' | 'ỏ' | 'õ' | 'ô' | 'ồ' | 'ố' | 'ộ' | 'ổ' | 'ỗ' | 'ơ' | 'ờ'
                | 'ớ' | 'ợ' | 'ở' | 'ỡ' => 'o',
                'ù' | 'ú' | 'ụ' | 'ủ' | 'ũ' | 'ư' | 'ừ' | 'ứ' | 'ự' | 'ử' | 'ữ' => {
                    'u'
                }
                'ỳ' | 'ý' | 'ỵ' | 'ỷ' | 'ỹ' => 'y',
                'đ' => 'd',
                other => other,
            };
            out.push(mapped);
        }
    }
    out
}

/// Check whether `haystack` matches `query_lower` (or `query_folded` without Vietnamese accents).
pub(super) fn matches_vn_query(haystack: &str, query_lower: &str, query_folded: &str) -> bool {
    if query_lower.is_empty() {
        return true;
    }
    if haystack.to_lowercase().contains(query_lower) {
        return true;
    }
    !query_folded.is_empty() && fold_vn_search(haystack).contains(query_folded)
}

fn paint_circular_image(
    painter: &egui::Painter,
    center: Pos2,
    radius: f32,
    tex: &TextureHandle,
) {
    let [tw, th] = tex.size();
    let (uv_min, uv_max) = if tw == th || tw == 0 || th == 0 {
        (Pos2::ZERO, Pos2::new(1.0, 1.0))
    } else if tw > th {
        let crop = ((tw - th) as f32 / (2.0 * tw as f32)).clamp(0.0, 0.49);
        (Pos2::new(crop, 0.0), Pos2::new(1.0 - crop, 1.0))
    } else {
        let crop = ((th - tw) as f32 / (2.0 * th as f32)).clamp(0.0, 0.49);
        (Pos2::new(0.0, crop), Pos2::new(1.0, 1.0 - crop))
    };

    let mut mesh = egui::Mesh::with_texture(tex.id());
    let n_pts = 48;
    mesh.vertices.push(egui::epaint::Vertex {
        pos: center,
        uv: Pos2::new((uv_min.x + uv_max.x) * 0.5, (uv_min.y + uv_max.y) * 0.5),
        color: Color32::WHITE,
    });
    for i in 0..=n_pts {
        let angle = (i as f32) / (n_pts as f32) * std::f32::consts::TAU;
        let (sin_a, cos_a) = angle.sin_cos();
        let pos = Pos2::new(center.x + radius * cos_a, center.y + radius * sin_a);
        let u = uv_min.x + (cos_a * 0.5 + 0.5) * (uv_max.x - uv_min.x);
        let v = uv_min.y + (sin_a * 0.5 + 0.5) * (uv_max.y - uv_min.y);
        mesh.vertices.push(egui::epaint::Vertex {
            pos,
            uv: Pos2::new(u, v),
            color: Color32::WHITE,
        });
        if i > 0 {
            mesh.indices.push(0);
            mesh.indices.push(i as u32);
            mesh.indices.push((i + 1) as u32);
        }
    }
    painter.add(egui::Shape::mesh(mesh));
}

fn paint_single_circle_avatar(
    painter: &egui::Painter,
    center: Pos2,
    radius: f32,
    name: &str,
    is_group: bool,
    avatar_url: Option<&str>,
    textures: &HashMap<String, TextureHandle>,
    loading: &mut HashSet<String>,
) {
    if let Some(url) = avatar_url.map(str::trim).filter(|u| {
        !u.is_empty()
            && !u.ends_with("/default")
            && !u.contains("default_avatar")
            && !u.contains("avatar_default")
            && (u.starts_with("http://") || u.starts_with("https://"))
    }) {
        let file_or_base = crate::chat::avatar_file_key(Some(url))
            .unwrap_or_else(|| url.split('?').next().unwrap_or(url).to_string());
        let hash = crate::crypto::md5_hex(&file_or_base);
        // Issue #131: Use `avt_` key prefix so `worker.rs` decodes the full uncropped square
        // avatar instead of applying the legacy `ava_` 160x160 center cutout crop (which zoomed
        // 240x240 / 480x480 Zalo avatars by 1.5x–3x).
        let key = format!("avt_{hash}");
        if let Some(tex) = textures.get(&key) {
            paint_circular_image(painter, center, radius, tex);
            painter.circle_stroke(
                center,
                radius,
                Stroke::new(1.0_f32, Color32::from_rgba_unmultiplied(255, 255, 255, 34)),
            );
            return;
        }
        if loading.insert(key.clone()) {
            if let Some(tx) = WORKER_TX.get() {
                let cache_dir = crate::account_manager::media_cache_dir();
                let legacy_cache = cache_dir.join(format!("ava_{hash}.png"));
                let new_cache = cache_dir.join(format!("avt_{hash}.png"));
                let fetch_url = if !new_cache.is_file() && legacy_cache.is_file() {
                    format!("local://{}", legacy_cache.display())
                } else {
                    url.to_string()
                };
                let _ = tx.send(WorkerCmd::FetchStickerImage {
                    key,
                    url: fetch_url,
                });
            }
        }
    }

    let av_col = avatar_color_for(name, is_group);
    painter.circle_filled(center, radius, av_col);
    let initial = if radius >= 15.5 {
        avatar_initials(name)
    } else {
        name.trim_matches(|c: char| !c.is_alphanumeric())
            .chars()
            .next()
            .unwrap_or('Z')
            .to_uppercase()
            .to_string()
    };
    painter.text(
        center,
        egui::Align2::CENTER_CENTER,
        initial,
        FontId::proportional((radius * 0.76).max(8.5)),
        Color32::WHITE,
    );
}

/// Paint a circular user or group avatar:
/// - For 1-1 contacts or groups with a custom uploaded group photo (`http(s)://...`), renders the circular image.
/// - For groups without a single custom photo (`multi:...` or `None`), renders Zalo PC's `zavatar-multi`
///   composite layout (3-member triangle or 2x2 member grid with optional member count badge).
pub(super) fn paint_user_avatar(
    painter: &egui::Painter,
    center: Pos2,
    radius: f32,
    name: &str,
    is_group: bool,
    avatar_url: Option<&str>,
    textures: &HashMap<String, TextureHandle>,
    loading: &mut HashSet<String>,
) {
    let raw_ava = avatar_url.map(str::trim).unwrap_or("");
    let has_single_custom_photo = !raw_ava.is_empty()
        && !raw_ava.starts_with("multi:")
        && !raw_ava.ends_with("/default")
        && (raw_ava.starts_with("http://") || raw_ava.starts_with("https://"));

    if !is_group || has_single_custom_photo {
        paint_single_circle_avatar(
            painter, center, radius, name, is_group, avatar_url, textures, loading,
        );
        return;
    }

    // Parse composite group member slots (`multi:<total>;<name1>|<url1>;<name2>|<url2>;...`)
    let mut slots: Vec<(String, String)> = Vec::new();
    let mut total_members: usize = 0;
    if let Some(rest) = raw_ava.strip_prefix("multi:") {
        for (idx, part) in rest.split(';').enumerate() {
            if idx == 0 && !part.contains('|') {
                total_members = part.trim().parse::<usize>().unwrap_or(0);
                continue;
            }
            if let Some((m_name, m_url)) = part.split_once('|') {
                slots.push((m_name.trim().to_string(), m_url.trim().to_string()));
            } else if !part.trim().is_empty() {
                slots.push((part.trim().to_string(), String::new()));
            }
        }
    }

    // Nếu không có slots hoặc tất cả các slot đều không có ảnh URL:
    // Vẽ avatar nhóm chuẩn đơn với màu sắc và tên viết tắt đẹp mắt, không cắt ghép chữ li ti
    let has_any_photo = slots.iter().any(|(_, u)| !u.is_empty());
    if !has_any_photo {
        paint_single_circle_avatar(
            painter, center, radius, name, true, None, textures, loading,
        );
        return;
    }

    if total_members < slots.len() {
        total_members = slots.len();
    }

    let border_col = Color32::from_rgb(26, 30, 38);
    // Vẽ vòng tròn nền lớn chuẩn bán kính R để toàn bộ avatar nhóm nằm gọn trong hình tròn
    painter.circle_filled(center, radius, border_col);
    painter.circle_stroke(
        center,
        radius,
        Stroke::new(1.0_f32, Color32::from_rgba_unmultiplied(255, 255, 255, 24)),
    );

    let n_slots = slots.len().min(4);
    if n_slots == 2 {
        let sub_r = radius * 0.46;
        let delta = radius * 0.25;
        let p0 = Pos2::new(center.x - delta, center.y + delta);
        let p1 = Pos2::new(center.x + delta, center.y - delta);
        for &(pos, idx) in &[(p1, 1usize), (p0, 0usize)] {
            if let Some((ref m_name, ref m_url)) = slots.get(idx) {
                painter.circle_filled(pos, sub_r + 1.2, border_col);
                paint_single_circle_avatar(
                    painter,
                    pos,
                    sub_r,
                    m_name,
                    false,
                    if m_url.is_empty() { None } else { Some(m_url.as_str()) },
                    textures,
                    loading,
                );
            }
        }
    } else if n_slots == 3 && total_members <= 3 {
        // Zalo PC `zavatar-multi.zavatar-3`: 1 top-center, 1 bottom-left, 1 bottom-right
        let sub_r = radius * 0.42;
        let p_top = Pos2::new(center.x, center.y - radius * 0.40);
        let p_bl = Pos2::new(center.x - radius * 0.38, center.y + radius * 0.34);
        let p_br = Pos2::new(center.x + radius * 0.38, center.y + radius * 0.34);
        for &(pos, idx) in &[(p_br, 2usize), (p_bl, 1usize), (p_top, 0usize)] {
            if let Some((ref m_name, ref m_url)) = slots.get(idx) {
                painter.circle_filled(pos, sub_r + 1.2, border_col);
                paint_single_circle_avatar(
                    painter,
                    pos,
                    sub_r,
                    m_name,
                    false,
                    if m_url.is_empty() { None } else { Some(m_url.as_str()) },
                    textures,
                    loading,
                );
            }
        }
    } else {
        // Zalo PC `zavatar-multi.zavatar-4`: 2x2 grid (với ô đếm số thành viên còn lại ở góc dưới-phải nếu > 4)
        let sub_r = radius * 0.38;
        let delta = radius * 0.38;
        let p_tl = Pos2::new(center.x - delta, center.y - delta);
        let p_tr = Pos2::new(center.x + delta, center.y - delta);
        let p_bl = Pos2::new(center.x - delta, center.y + delta);
        let p_br = Pos2::new(center.x + delta, center.y + delta);

        let show_counter = total_members > 4;
        if show_counter {
            painter.circle_filled(p_br, sub_r + 1.2, border_col);
            painter.circle_filled(p_br, sub_r, Color32::from_rgb(52, 60, 76));
            let cnt_txt = if total_members > 99 {
                "99+".to_string()
            } else {
                format!("+{}", total_members.saturating_sub(3))
            };
            painter.text(
                p_br,
                egui::Align2::CENTER_CENTER,
                cnt_txt,
                FontId::proportional((sub_r * 0.76).max(8.0)),
                Color32::from_rgb(220, 228, 242),
            );
        } else if let Some((ref m_name, ref m_url)) = slots.get(3) {
            painter.circle_filled(p_br, sub_r + 1.2, border_col);
            paint_single_circle_avatar(
                painter,
                p_br,
                sub_r,
                m_name,
                false,
                if m_url.is_empty() { None } else { Some(m_url.as_str()) },
                textures,
                loading,
            );
        }

        for &(pos, idx) in &[(p_bl, 2usize), (p_tr, 1usize), (p_tl, 0usize)] {
            if let Some((ref m_name, ref m_url)) = slots.get(idx) {
                painter.circle_filled(pos, sub_r + 1.2, border_col);
                paint_single_circle_avatar(
                    painter,
                    pos,
                    sub_r,
                    m_name,
                    false,
                    if m_url.is_empty() { None } else { Some(m_url.as_str()) },
                    textures,
                    loading,
                );
            }
        }
    }
}

pub(super) fn draw_lucide_icon(
    painter: &egui::Painter,
    center: Pos2,
    icon: &str,
    size: f32,
    color: Color32,
) {
    super::design_system::widgets::paint_icon(painter, center, icon, size, color);
}

pub(super) fn draw_icon_search(painter: &egui::Painter, center: Pos2, size: f32, color: Color32) {
    draw_lucide_icon(painter, center, lucide::SEARCH, size, color);
}

pub(super) fn draw_icon_refresh(painter: &egui::Painter, center: Pos2, size: f32, color: Color32) {
    draw_lucide_icon(painter, center, lucide::REFRESH, size, color);
}

pub(super) fn draw_icon_sticker(painter: &egui::Painter, center: Pos2, size: f32, color: Color32) {
    draw_lucide_icon(painter, center, lucide::SMILE, size, color);
}

pub(super) fn draw_icon_send(painter: &egui::Painter, center: Pos2, size: f32, color: Color32) {
    draw_lucide_icon(painter, center, lucide::SEND, size, color);
}

pub(super) fn draw_icon_phone(painter: &egui::Painter, center: Pos2, size: f32, color: Color32) {
    draw_lucide_icon(painter, center, lucide::PHONE, size, color);
}

pub(super) fn draw_icon_video(painter: &egui::Painter, center: Pos2, size: f32, color: Color32) {
    draw_lucide_icon(painter, center, lucide::VIDEO, size, color);
}

pub(super) fn draw_icon_image(painter: &egui::Painter, center: Pos2, size: f32, color: Color32) {
    draw_lucide_icon(painter, center, lucide::IMAGE, size, color);
}

pub(super) fn draw_icon_contacts(painter: &egui::Painter, center: Pos2, size: f32, color: Color32) {
    draw_lucide_icon(painter, center, lucide::USERS, size, color);
}

pub(super) fn draw_icon_pin(painter: &egui::Painter, center: Pos2, size: f32, color: Color32) {
    draw_lucide_icon(painter, center, lucide::PIN, size, color);
}

pub(super) fn draw_icon_copy(painter: &egui::Painter, center: Pos2, size: f32, color: Color32) {
    draw_lucide_icon(painter, center, lucide::COPY, size, color);
}

pub(super) fn draw_icon_user_plus(
    painter: &egui::Painter,
    center: Pos2,
    size: f32,
    color: Color32,
) {
    draw_lucide_icon(painter, center, lucide::USER_PLUS, size, color);
}

/// Draw a crisp diagonal vector pencil (avoids missing `✎` glyph boxes across system fonts).
pub(super) fn draw_icon_pencil(
    painter: &egui::Painter,
    center: Pos2,
    size: f32,
    color: Color32,
) {
    let r = (size * 0.38).max(3.0);
    let sw = (size * 0.13).clamp(1.2, 2.0);
    let tip = Pos2::new(center.x - r * 0.85, center.y + r * 0.85);
    let tail = Pos2::new(center.x + r * 0.78, center.y - r * 0.78);
    let body_start = Pos2::new(center.x - r * 0.35, center.y + r * 0.35);
    painter.line_segment([body_start, tail], Stroke::new(sw * 1.35, color));
    painter.line_segment([tip, body_start], Stroke::new(sw * 0.9, color));
    let cap_a = Pos2::new(center.x + r * 0.42, center.y - r * 0.95);
    let cap_b = Pos2::new(center.x + r * 0.95, center.y - r * 0.42);
    painter.line_segment([cap_a, cap_b], Stroke::new(sw * 0.95, color));
}

/// Draw a crisp filled right-pointing play triangle (avoids missing `▶` glyph boxes).
pub(super) fn draw_icon_play(
    painter: &egui::Painter,
    center: Pos2,
    size: f32,
    color: Color32,
) {
    let r = (size * 0.38).max(3.5);
    let pts = vec![
        Pos2::new(center.x - r * 0.55, center.y - r * 0.85),
        Pos2::new(center.x + r * 0.95, center.y),
        Pos2::new(center.x - r * 0.55, center.y + r * 0.85),
    ];
    painter.add(egui::Shape::convex_polygon(pts, color, Stroke::NONE));
}

/// Draw a crisp vector pause icon (two vertical bars).
#[allow(dead_code)]
pub(super) fn draw_icon_pause(
    painter: &egui::Painter,
    center: Pos2,
    size: f32,
    color: Color32,
) {
    let bar_w = (size * 0.18).clamp(2.0, 4.0);
    let bar_h = (size * 0.65).max(6.0);
    let gap = (size * 0.22).max(3.0);
    let b1 = Rect::from_center_size(Pos2::new(center.x - gap, center.y), Vec2::new(bar_w, bar_h));
    let b2 = Rect::from_center_size(Pos2::new(center.x + gap, center.y), Vec2::new(bar_w, bar_h));
    painter.rect_filled(b1, Rounding::same(1.0), color);
    painter.rect_filled(b2, Rounding::same(1.0), color);
}

/// Draw a crisp vector clock icon (avoids missing `🕒` glyph boxes).
pub(super) fn draw_icon_clock(
    painter: &egui::Painter,
    center: Pos2,
    size: f32,
    color: Color32,
) {
    let r = (size * 0.40).max(4.5);
    let sw = (size * 0.10).clamp(1.2, 1.8);
    painter.circle_stroke(center, r, Stroke::new(sw, color));
    painter.line_segment([center, Pos2::new(center.x, center.y - r * 0.52)], Stroke::new(sw, color));
    painter.line_segment(
        [center, Pos2::new(center.x + r * 0.42, center.y + r * 0.18)],
        Stroke::new(sw, color),
    );
}

/// Draw a crisp vector warning triangle with exclamation mark (avoids missing `⚠` glyph boxes).
pub(super) fn draw_icon_warning(
    painter: &egui::Painter,
    center: Pos2,
    size: f32,
    color: Color32,
) {
    let r = (size * 0.44).max(4.5);
    let sw = (size * 0.11).clamp(1.2, 1.8);
    let top = Pos2::new(center.x, center.y - r * 0.85);
    let left = Pos2::new(center.x - r * 0.92, center.y + r * 0.78);
    let right = Pos2::new(center.x + r * 0.92, center.y + r * 0.78);
    painter.line_segment([top, right], Stroke::new(sw, color));
    painter.line_segment([right, left], Stroke::new(sw, color));
    painter.line_segment([left, top], Stroke::new(sw, color));
    painter.line_segment(
        [
            Pos2::new(center.x, center.y - r * 0.25),
            Pos2::new(center.x, center.y + r * 0.26),
        ],
        Stroke::new(sw, color),
    );
    painter.circle_filled(Pos2::new(center.x, center.y + r * 0.50), sw * 0.55, color);
}

pub const REACTION_LIST: [(&str, &str, &str); 6] = [
    ("👍", "like", "Thích"),
    ("❤️", "heart", "Yêu thích"),
    ("😆", "smile", "Haha"),
    ("😮", "surprised", "Wow"),
    ("😢", "cry", "Buồn"),
    ("😠", "rage", "Phẫn nộ"),
];

pub fn emoji_to_tex_key(emoji: &str) -> &'static str {
    match emoji.trim() {
        "👍" | "/-strong" => "like",
        "❤️" | "❤" | "/-heart" => "heart",
        "😆" | ":>" => "smile",
        "😮" | ":o" => "surprised",
        "😢" | ":-((" => "cry",
        "😠" | ":-h" => "rage",
        _ => "",
    }
}

pub(super) const APP_ICON_PNG: &[u8] = include_bytes!("../assets/app_icon.png");

/// Decode the embedded 256x256 RGBA application icon for the OS window/taskbar (`ViewportBuilder::with_icon`).
pub fn load_app_icon() -> Option<egui::IconData> {
    let img = image::load_from_memory(APP_ICON_PNG).ok()?.into_rgba8();
    let (width, height) = img.dimensions();
    Some(egui::IconData {
        rgba: img.into_raw(),
        width,
        height,
    })
}

/// Render a dynamic 64x64 RGBA window/taskbar icon with a crisp anti-aliased red unread badge
/// (`1`..`99+`) using a 3x5 bitmap pixel font renderer.
pub fn load_app_icon_with_badge(unread_count: usize) -> Option<egui::IconData> {
    if unread_count == 0 {
        return load_app_icon();
    }
    let base = image::load_from_memory(APP_ICON_PNG).ok()?;
    let size: u32 = 64;
    let mut rgba = image::imageops::resize(
        &base.to_rgba8(),
        size,
        size,
        image::imageops::FilterType::Lanczos3,
    )
    .into_raw();

    let label = if unread_count > 99 {
        "99+".to_string()
    } else {
        unread_count.to_string()
    };
    let chars: Vec<char> = label.chars().collect();

    let blend_pixel = |buf: &mut [u8], x: i32, y: i32, r: u8, g: u8, b: u8, alpha: u8| {
        if x < 0 || y < 0 || x >= size as i32 || y >= size as i32 || alpha == 0 {
            return;
        }
        let idx = ((y as u32 * size + x as u32) * 4) as usize;
        let src_a = alpha as f32 / 255.0;
        let dst_a = buf[idx + 3] as f32 / 255.0;
        let out_a = src_a + dst_a * (1.0 - src_a);
        if out_a <= 0.0 {
            return;
        }
        buf[idx] = (((r as f32) * src_a + (buf[idx] as f32) * dst_a * (1.0 - src_a)) / out_a)
            .round()
            .clamp(0.0, 255.0) as u8;
        buf[idx + 1] = (((g as f32) * src_a + (buf[idx + 1] as f32) * dst_a * (1.0 - src_a))
            / out_a)
            .round()
            .clamp(0.0, 255.0) as u8;
        buf[idx + 2] = (((b as f32) * src_a + (buf[idx + 2] as f32) * dst_a * (1.0 - src_a))
            / out_a)
            .round()
            .clamp(0.0, 255.0) as u8;
        buf[idx + 3] = (out_a * 255.0).round().clamp(0.0, 255.0) as u8;
    };

    let scale: f32 = 2.0;
    let badge_radius: f32 = 6.5 * scale;
    let half_span: f32 = if chars.len() >= 3 {
        4.0 * scale
    } else if chars.len() == 2 {
        2.0 * scale
    } else {
        0.0
    };
    let border_w: f32 = 1.2 * scale;
    let cx: f32 = (size as f32) - badge_radius - half_span - border_w.ceil();
    let cy: f32 = badge_radius + border_w.floor();

    let min_y = ((cy - badge_radius - border_w - 1.0).floor() as i32).max(0);
    let max_y = ((cy + badge_radius + border_w + 2.0).ceil() as i32).min(size as i32);
    let min_x = ((cx - half_span - badge_radius - border_w - 1.0).floor() as i32).max(0);
    let max_x = ((cx + half_span + badge_radius + border_w + 2.0).ceil() as i32).min(size as i32);

    for y in min_y..max_y {
        for x in min_x..max_x {
            let fx = x as f32;
            let fy = y as f32;
            let lo_x = cx - half_span;
            let hi_x = (cx + half_span).max(lo_x);
            let clamped_x = fx.clamp(lo_x, hi_x);
            let dist = (fx - clamped_x).hypot(fy - cy);
            if dist <= badge_radius + border_w + 0.6 {
                if dist <= badge_radius - 0.5 {
                    blend_pixel(&mut rgba, x, y, 239, 56, 56, 255);
                } else if dist <= badge_radius + 0.5 {
                    let t = dist - (badge_radius - 0.5);
                    let r = (239.0 * (1.0 - t) + 255.0 * t).round() as u8;
                    let g = (56.0 * (1.0 - t) + 255.0 * t).round() as u8;
                    let b = (56.0 * (1.0 - t) + 255.0 * t).round() as u8;
                    blend_pixel(&mut rgba, x, y, r, g, b, 255);
                } else if dist <= badge_radius + border_w - 0.3 {
                    blend_pixel(&mut rgba, x, y, 255, 255, 255, 255);
                } else {
                    let a = ((badge_radius + border_w + 0.6 - dist) * 255.0)
                        .round()
                        .clamp(0.0, 255.0) as u8;
                    blend_pixel(&mut rgba, x, y, 255, 255, 255, a);
                }
            }
        }
    }

    let glyph_rows = |ch: char| -> [u8; 5] {
        match ch {
            '0' => [0b111, 0b101, 0b101, 0b101, 0b111],
            '1' => [0b010, 0b110, 0b010, 0b010, 0b111],
            '2' => [0b111, 0b001, 0b111, 0b100, 0b111],
            '3' => [0b111, 0b001, 0b111, 0b001, 0b111],
            '4' => [0b101, 0b101, 0b111, 0b001, 0b001],
            '5' => [0b111, 0b100, 0b111, 0b001, 0b111],
            '6' => [0b111, 0b100, 0b111, 0b101, 0b111],
            '7' => [0b111, 0b001, 0b010, 0b010, 0b010],
            '8' => [0b111, 0b101, 0b111, 0b101, 0b111],
            '9' => [0b111, 0b101, 0b111, 0b001, 0b111],
            '+' => [0b000, 0b010, 0b111, 0b010, 0b000],
            _ => [0; 5],
        }
    };

    let px_scale: i32 = 2;
    let glyph_w: i32 = 3 * px_scale;
    let glyph_h: i32 = 5 * px_scale;
    let gap: i32 = px_scale;
    let total_w: i32 = (chars.len() as i32) * glyph_w + ((chars.len() as i32 - 1).max(0)) * gap;
    let start_x: i32 = (cx - (total_w as f32) * 0.5).round() as i32;
    let start_y: i32 = (cy - (glyph_h as f32) * 0.5).round() as i32;

    for (idx, ch) in chars.iter().enumerate() {
        let rows = glyph_rows(*ch);
        let gx = start_x + (idx as i32) * (glyph_w + gap);
        for (r, mask) in rows.iter().enumerate() {
            for c in 0..3_i32 {
                if (mask >> (2 - c)) & 1 == 1 {
                    for dy in 0..px_scale {
                        for dx in 0..px_scale {
                            blend_pixel(
                                &mut rgba,
                                gx + c * px_scale + dx,
                                start_y + (r as i32) * px_scale + dy,
                                255,
                                255,
                                255,
                                255,
                            );
                        }
                    }
                }
            }
        }
    }

    Some(egui::IconData {
        rgba,
        width: size,
        height: size,
    })
}

pub(super) fn load_app_logo_texture(ctx: &egui::Context) -> Option<TextureHandle> {
    let img = image::load_from_memory(APP_ICON_PNG).ok()?.into_rgba8();
    let (w, h) = img.dimensions();
    let color_img =
        egui::ColorImage::from_rgba_unmultiplied([w as usize, h as usize], img.as_raw());
    Some(ctx.load_texture("zalo_app_logo", color_img, TextureOptions::LINEAR))
}

pub(super) fn load_default_emoji_textures(ctx: &egui::Context) -> HashMap<String, TextureHandle> {
    let raw: [(&str, &[u8]); 6] = [
        ("like", include_bytes!("../assets/emojis/like.png")),
        ("heart", include_bytes!("../assets/emojis/heart.png")),
        ("smile", include_bytes!("../assets/emojis/smile.png")),
        (
            "surprised",
            include_bytes!("../assets/emojis/surprised.png"),
        ),
        ("cry", include_bytes!("../assets/emojis/cry.png")),
        ("rage", include_bytes!("../assets/emojis/rage.png")),
    ];
    let mut map = HashMap::new();
    for (name, bytes) in raw {
        if let Ok(img) = image::load_from_memory(bytes) {
            let rgba = img.to_rgba8();
            let (w, h) = rgba.dimensions();
            let color_img =
                egui::ColorImage::from_rgba_unmultiplied([w as usize, h as usize], rgba.as_raw());
            let tex = ctx.load_texture(
                format!("zalo_emoji_{name}"),
                color_img,
                TextureOptions::LINEAR,
            );
            map.insert(name.to_string(), tex);
        }
    }
    map
}

/// Pill toggle switch; thin wrapper over the design-system widget.
pub(super) fn render_toggle_switch(
    ui: &mut egui::Ui,
    value: &mut bool,
    pal: &ThemePalette,
) -> egui::Response {
    super::design_system::widgets::toggle_switch(ui, pal, value)
}

static CJK_LOADED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static ACTIVE_FONT_CHOICE: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());

pub(crate) fn is_cjk(c: char) -> bool {
    matches!(c as u32,
        0x1100..=0x11FF | 0x3040..=0x30FF | 0x3130..=0x318F | 0x3400..=0x4DBF
        | 0x4E00..=0x9FFF | 0xAC00..=0xD7AF | 0xF900..=0xFAFF)
}

/// Load the CJK fallback the first time Han / Kana / Hangul text shows up. msyh.ttc alone is
/// ~20 MB held in memory for the whole run, which a Vietnamese-only user never needs.
pub(super) fn ensure_cjk_font<'a>(ctx: &egui::Context, mut texts: impl Iterator<Item = &'a str>) {
    use std::sync::atomic::Ordering;
    if CJK_LOADED.load(Ordering::Relaxed) || !texts.any(|t| t.chars().any(is_cjk)) {
        return;
    }
    CJK_LOADED.store(true, Ordering::Relaxed);
    let choice = ACTIVE_FONT_CHOICE
        .lock()
        .map(|c| c.clone())
        .unwrap_or_else(|_| "default".into());
    build_fonts(ctx, &choice, true);
}

pub(super) fn setup_custom_fonts_with_choice(ctx: &egui::Context, font_choice: &str) {
    if let Ok(mut active) = ACTIVE_FONT_CHOICE.lock() {
        *active = font_choice.to_string();
    }
    let with_cjk = CJK_LOADED.load(std::sync::atomic::Ordering::Relaxed);
    build_fonts(ctx, font_choice, with_cjk);
}

#[allow(dead_code)]
pub(super) fn setup_custom_fonts(ctx: &egui::Context) {
    setup_custom_fonts_with_choice(ctx, "default");
}

pub(super) fn build_fonts(ctx: &egui::Context, font_choice: &str, with_cjk: bool) {
    let mut fonts = egui::FontDefinitions::default();

    // 1. Embed official Lucide Icon Font (vector glyphs for all UI icons)
    const LUCIDE_BYTES: &[u8] = include_bytes!("../assets/fonts/lucide.ttf");
    fonts.font_data.insert(
        "lucide_font".to_owned(),
        egui::FontData::from_static(LUCIDE_BYTES),
    );

    // 2. Bundled Vietnamese sans-serif (Regular + SemiBold) so diacritics & bold headings
    //    render identically on every OS. System fonts below are only an optional override.
    const BUNDLED_SANS: &[u8] = include_bytes!("../assets/fonts/BeVietnamPro-Regular.ttf");
    const BUNDLED_SANS_SEMIBOLD: &[u8] = include_bytes!("../assets/fonts/BeVietnamPro-SemiBold.ttf");
    fonts.font_data.insert(
        "bundled_sans".to_owned(),
        egui::FontData::from_static(BUNDLED_SANS),
    );
    fonts.font_data.insert(
        "bundled_sans_semibold".to_owned(),
        egui::FontData::from_static(BUNDLED_SANS_SEMIBOLD),
    );

    // 3. Optional user-selected system or custom font family (Requirement R2)
    let mut selected_font_loaded = false;
    let choice_norm = font_choice.trim().to_lowercase();
    match choice_norm.as_str() {
        "segoe_ui" | "segoe" => {
            let candidates = [
                "C:\\Windows\\Fonts\\segoeui.ttf",
                "/usr/share/fonts/truetype/segoeui.ttf",
            ];
            for path in candidates {
                if let Ok(font_data) = std::fs::read(path) {
                    fonts.font_data.insert(
                        "selected_font".to_owned(),
                        egui::FontData::from_owned(font_data),
                    );
                    selected_font_loaded = true;
                    break;
                }
            }
        }
        "arial" => {
            let candidates = [
                "C:\\Windows\\Fonts\\arial.ttf",
                "/System/Library/Fonts/Supplemental/Arial.ttf",
                "/Library/Fonts/Arial.ttf",
                "/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf",
                "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
            ];
            for path in candidates {
                if let Ok(font_data) = std::fs::read(path) {
                    fonts.font_data.insert(
                        "selected_font".to_owned(),
                        egui::FontData::from_owned(font_data),
                    );
                    selected_font_loaded = true;
                    break;
                }
            }
        }
        "tahoma" => {
            let candidates = [
                "C:\\Windows\\Fonts\\tahoma.ttf",
                "/System/Library/Fonts/Supplemental/Tahoma.ttf",
            ];
            for path in candidates {
                if let Ok(font_data) = std::fs::read(path) {
                    fonts.font_data.insert(
                        "selected_font".to_owned(),
                        egui::FontData::from_owned(font_data),
                    );
                    selected_font_loaded = true;
                    break;
                }
            }
        }
        "default" | "" => {
            // Uses bundled_sans directly at priority index 0
        }
        _custom_path => {
            if let Ok(font_data) = std::fs::read(font_choice.trim()) {
                fonts.font_data.insert(
                    "selected_font".to_owned(),
                    egui::FontData::from_owned(font_data),
                );
                selected_font_loaded = true;
            }
        }
    }

    // Also load generic system vietnamese_font as supplementary candidate if available
    let font_candidates = [
        "C:\\Windows\\Fonts\\segoeui.ttf",
        "C:\\Windows\\Fonts\\arial.ttf",
        "C:\\Windows\\Fonts\\tahoma.ttf",
        "/System/Library/Fonts/Supplemental/Arial.ttf",
        "/System/Library/Fonts/Supplemental/Tahoma.ttf",
        "/Library/Fonts/Arial.ttf",
        "/usr/share/fonts/dejavu-sans-fonts/DejaVuSans.ttf",
        "/usr/share/fonts/google-noto/NotoSans-Regular.ttf",
        "/usr/share/fonts/truetype/noto/NotoSans-Regular.ttf",
        "/usr/share/fonts/noto/NotoSans-Regular.ttf",
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
        "/usr/share/fonts/TTF/DejaVuSans.ttf",
        "/usr/share/fonts/truetype/ubuntu/Ubuntu-R.ttf",
        "/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf",
    ];

    for path in font_candidates {
        if let Ok(font_data) = std::fs::read(path) {
            fonts.font_data.insert(
                "vietnamese_font".to_owned(),
                egui::FontData::from_owned(font_data),
            );
            break;
        }
    }

    // 4. Optional CJK (Chinese / Japanese / Korean) system font fallback (#5, #6)
    let cjk_candidates = [
        "C:\\Windows\\Fonts\\msyh.ttc",
        "C:\\Windows\\Fonts\\simsun.ttc",
        "C:\\Windows\\Fonts\\malgun.ttf",
        "C:\\Windows\\Fonts\\YuGothR.ttc",
        "/System/Library/Fonts/PingFang.ttc",
        "/System/Library/Fonts/Hiragino Sans GB.ttc",
        "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
        "/usr/share/fonts/google-noto-cjk/NotoSansCJK-Regular.ttc",
        "/usr/share/fonts/noto-cjk/NotoSansCJK-Regular.ttc",
        "/usr/share/fonts/wqy-zenhei/wqy-zenhei.ttc",
    ];
    for path in cjk_candidates.into_iter().filter(|_| with_cjk) {
        if let Ok(font_data) = std::fs::read(path) {
            fonts
                .font_data
                .insert("cjk_font".to_owned(), egui::FontData::from_owned(font_data));
            break;
        }
    }

    // 5. Fallback priority order:
    // If a custom or system font is selected, place it at index 0.
    // Index 1 (or 0 for default) is bundled_sans, guaranteeing full Vietnamese diacritic coverage.
    let prop = fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default();
    if selected_font_loaded {
        prop.insert(0, "selected_font".to_owned());
        prop.insert(1, "bundled_sans".to_owned());
    } else {
        prop.insert(0, "bundled_sans".to_owned());
    }
    if fonts.font_data.contains_key("vietnamese_font") && !selected_font_loaded {
        prop.push("vietnamese_font".to_owned());
    }
    if fonts.font_data.contains_key("cjk_font") {
        prop.push("cjk_font".to_owned());
    }
    prop.push("lucide_font".to_owned());

    let mono = fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default();
    mono.push("bundled_sans".to_owned());
    if fonts.font_data.contains_key("vietnamese_font") {
        mono.push("vietnamese_font".to_owned());
    }
    if fonts.font_data.contains_key("cjk_font") {
        mono.push("cjk_font".to_owned());
    }
    mono.push("lucide_font".to_owned());

    let semibold = fonts.families.entry(font_family_semibold()).or_default();
    if selected_font_loaded {
        semibold.push("selected_font".to_owned());
    }
    semibold.push("bundled_sans_semibold".to_owned());
    semibold.push("bundled_sans".to_owned());
    if fonts.font_data.contains_key("vietnamese_font") {
        semibold.push("vietnamese_font".to_owned());
    }
    if fonts.font_data.contains_key("cjk_font") {
        semibold.push("cjk_font".to_owned());
    }
    semibold.push("lucide_font".to_owned());

    ctx.set_fonts(fonts);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_format_hex_color() {
        assert_eq!(
            parse_hex_color("#0068FF"),
            Some(Color32::from_rgb(0, 104, 255))
        );
        assert_eq!(
            parse_hex_color("528BFF"),
            Some(Color32::from_rgb(82, 139, 255))
        );
        assert_eq!(
            parse_hex_color("#0068FF80"),
            Some(Color32::from_rgba_unmultiplied(0, 104, 255, 128))
        );
        assert_eq!(parse_hex_color("invalid"), None);
        assert_eq!(parse_hex_color("#123"), None);

        let col = Color32::from_rgb(0, 104, 255);
        assert_eq!(format_hex_color(col), "#0068FF");
    }

    #[test]
    fn test_palette_apply_custom_colors() {
        let mut pal = ThemePalette::light();
        pal.apply_custom_colors(
            Some("#059669"),
            Some("#7C3AED"),
            Some("#E11D48"),
            Some("#FFFFFF"),
        );
        assert_eq!(pal.primary, Color32::from_rgb(5, 150, 105));
        assert_eq!(pal.accent_online, Color32::from_rgb(124, 58, 237));
        assert_eq!(pal.bubble_out_bg, Color32::from_rgb(225, 29, 72));
        assert_eq!(pal.bubble_in_bg, Color32::WHITE);
        assert_eq!(pal.bubble_in_text, Color32::from_rgb(15, 23, 42));
    }

    #[test]
    fn test_palette_dark_mode_bubble_contrast() {
        let mut pal = ThemePalette::dark();
        pal.apply_custom_colors(
            None,
            None,
            Some("#1E293B"),
            None,
        );
        assert_eq!(pal.bubble_out_bg, Color32::from_rgb(30, 41, 59));
        assert_eq!(pal.bubble_out_text, Color32::from_rgb(235, 240, 246));
    }

    #[test]
    fn test_wcag_aa_contrast_matrix_dark_and_light() {
        for pal in [ThemePalette::dark(), ThemePalette::light()] {
            assert!(contrast_ratio(pal.primary_fg, pal.primary) >= 4.5);
            assert!(contrast_ratio(pal.text_primary, pal.bg_app) >= 4.5);
            assert!(contrast_ratio(pal.text_primary, pal.bg_elevated) >= 4.5);
            assert!(contrast_ratio(pal.text_secondary, pal.bg_chat_canvas) >= 4.5);
            assert!(contrast_ratio(pal.text_muted, pal.bg_chat_canvas) >= 4.5);
            assert!(contrast_ratio(pal.text_muted, pal.bg_elevated) >= 4.5);
            assert!(contrast_ratio(pal.accent_online, pal.bg_elevated) >= 4.5);
            assert!(contrast_ratio(pal.danger_text, pal.bg_elevated) >= 4.5);
            assert!(contrast_ratio(pal.warning, pal.bg_elevated) >= 4.5);
            assert!(contrast_ratio(pal.link_text, pal.bg_chat_canvas) >= 4.5);
            assert!(contrast_ratio(pal.bubble_out_text, pal.bubble_out_bg) >= 4.5);
            assert!(contrast_ratio(pal.bubble_out_meta, pal.bubble_out_bg) >= 4.5);
            assert!(contrast_ratio(pal.bubble_in_text, pal.bubble_in_bg) >= 4.5);
            assert!(contrast_ratio(pal.bubble_in_meta, pal.bubble_in_bg) >= 4.5);
        }
    }

    #[test]
    fn test_emoji_to_tex_key_unknown_returns_empty() {
        assert_eq!(emoji_to_tex_key("👍"), "like");
        assert_eq!(emoji_to_tex_key("/-heart"), "heart");
        assert_eq!(emoji_to_tex_key(":-(("), "cry");
        assert_eq!(emoji_to_tex_key("🔥"), "");
        assert_eq!(emoji_to_tex_key("🎉"), "");
    }

    #[test]
    fn test_apply_to_ctx_syncs_dark_and_light_visuals() {
        let ctx = egui::Context::default();
        let dark = ThemePalette::dark();
        dark.apply_to_ctx_with_anim(&ctx, true, 1.0);
        assert_eq!(ctx.theme(), egui::Theme::Dark);
        let dark_style = ctx.style();
        assert!(dark_style.visuals.dark_mode);
        assert_eq!(dark_style.visuals.extreme_bg_color, dark.bg_input);
        assert_eq!(dark_style.visuals.widgets.inactive.weak_bg_fill, dark.bg_elevated);
        assert_eq!(dark_style.visuals.panel_fill, dark.bg_app);

        let light = ThemePalette::light();
        light.apply_to_ctx_with_anim(&ctx, false, 1.0);
        assert_eq!(ctx.theme(), egui::Theme::Light);
        let light_style = ctx.style();
        assert!(!light_style.visuals.dark_mode);
        assert_eq!(light_style.visuals.extreme_bg_color, light.bg_input);
        assert_eq!(light_style.visuals.widgets.inactive.weak_bg_fill, light.bg_elevated);
        assert_eq!(light_style.visuals.panel_fill, light.bg_app);
    }
}
