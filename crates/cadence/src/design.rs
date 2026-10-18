use std::rc::Rc;

use gpui::{Anchor, App, Font, FontFallbacks, SharedString, Window, WindowAppearance, font};
use gpui_component::{Theme, ThemeConfig, ThemeConfigColors, ThemeMode};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub(crate) struct AppFonts {
    pub ui: Font,
    pub monospace: Font,
    pub lyrics: Font,
}

pub(crate) fn resolve_fonts(
    ui: &[String],
    monospace: &[String],
    lyrics: &[String],
    cx: &App,
) -> AppFonts {
    let available = cx.text_system().all_font_names();
    AppFonts {
        ui: resolve_font_chain(ui, ".SystemUIFont", &available),
        monospace: resolve_font_chain(monospace, ".SystemUIFont", &available),
        lyrics: resolve_font_chain(lyrics, ".SystemUIFont", &available),
    }
}

fn resolve_font_chain(families: &[String], default: &str, available: &[String]) -> Font {
    let mut resolved = families
        .iter()
        .filter_map(|family| {
            available
                .iter()
                .find(|available| available.eq_ignore_ascii_case(family))
                .cloned()
        })
        .collect::<Vec<_>>();
    if resolved.is_empty() {
        resolved.push(default.to_owned());
    }

    let mut resolved_font = font(resolved.remove(0));
    if !resolved.is_empty() {
        resolved_font.fallbacks = Some(FontFallbacks::from_fonts(resolved));
    }
    resolved_font
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ColorTheme {
    #[default]
    #[serde(
        alias = "catppuccin-latte",
        alias = "everforest-light",
        alias = "rose-pine-dawn",
        alias = "kanagawa-lotus",
        alias = "one-light",
        alias = "gruvbox-light",
        alias = "dracula-light",
        alias = "cadence-neutral",
        alias = "lyrune-neutral"
    )]
    AyuLight,
    #[serde(
        alias = "catppuccin-mocha",
        alias = "everforest-dark",
        alias = "rose-pine-moon",
        alias = "kanagawa-wave",
        alias = "ayu-mirage",
        alias = "one-dark",
        alias = "gruvbox-dark",
        alias = "dracula-dark"
    )]
    AyuDark,
}

impl ColorTheme {
    #[cfg(test)]
    pub const ALL: [Self; 2] = [Self::AyuLight, Self::AyuDark];

    #[cfg(test)]
    pub const fn id(self) -> &'static str {
        match self {
            Self::AyuLight => "ayu-light",
            Self::AyuDark => "ayu-dark",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::AyuLight => "Ayu Light",
            Self::AyuDark => "Ayu Dark",
        }
    }

    pub(crate) const fn palette(self) -> Palette {
        match self {
            Self::AyuLight => Palette {
                background: "#fcfcfc",
                surface: "#f8f9fa",
                sidebar: "#f8f9fa",
                outer: "#ebeef0",
                foreground: "#30363d",
                subtext_foreground: "#4a535c",
                muted: "#ebeef0",
                muted_foreground: "#4f555c",
                border: "#6b7d8f1f",
                primary: "#22a4e6",
                primary_foreground: "#ffffff",
                primary_hover: "#1a91cd",
                primary_active: "#1a91cd",
                accent: "#f3f4f5",
                accent_foreground: "#30363d",
                active: "#035bd626",
                hover: "#6b7d8f1f",
                ring: "#f29718",
                emotion: "#e65050",
                emotion_foreground: "#ffffff",
                info: "#22a4e6",
                info_foreground: "#ffffff",
                success: "#86b300",
                success_foreground: "#ffffff",
                warning: "#f29718",
                warning_foreground: "#ffffff",
                scrollbar_thumb: "#c5c5c8",
            },
            Self::AyuDark => Palette {
                background: "#0d1016",
                surface: "#16191f",
                sidebar: "#16191f",
                outer: "#090b10",
                foreground: "#b3b1ad",
                subtext_foreground: "#9da0a2",
                muted: "#1f2127",
                muted_foreground: "#73777b",
                border: "#292a2c",
                primary: "#5ac1fe",
                primary_foreground: "#1f2430",
                primary_hover: "#3daee9",
                primary_active: "#36a3d9",
                accent: "#20242b",
                accent_foreground: "#b3b1ad",
                active: "#36a3d922",
                hover: "#191f2a99",
                ring: "#ffb454",
                emotion: "#f07178",
                emotion_foreground: "#0d1016",
                info: "#5ac1fe",
                info_foreground: "#0d1016",
                success: "#aad94c",
                success_foreground: "#0d1016",
                warning: "#ffb454",
                warning_foreground: "#0d1016",
                scrollbar_thumb: "#bfbdb64c",
            },
        }
    }
}

#[derive(Clone, Copy)]
pub struct Palette {
    pub background: &'static str,
    pub surface: &'static str,
    pub sidebar: &'static str,
    pub outer: &'static str,
    pub foreground: &'static str,
    pub subtext_foreground: &'static str,
    pub muted: &'static str,
    pub muted_foreground: &'static str,
    pub border: &'static str,
    pub primary: &'static str,
    pub primary_foreground: &'static str,
    pub primary_hover: &'static str,
    pub primary_active: &'static str,
    pub accent: &'static str,
    pub accent_foreground: &'static str,
    pub active: &'static str,
    pub hover: &'static str,
    pub ring: &'static str,
    pub emotion: &'static str,
    pub emotion_foreground: &'static str,
    pub info: &'static str,
    pub info_foreground: &'static str,
    pub success: &'static str,
    pub success_foreground: &'static str,
    pub warning: &'static str,
    pub warning_foreground: &'static str,
    pub scrollbar_thumb: &'static str,
}

pub(crate) fn apply(
    light_theme: ColorTheme,
    dark_theme: ColorTheme,
    mode: ThemeMode,
    follow_system: bool,
    glass: bool,
    glass_opacity: u32,
    fonts: &AppFonts,
    window: Option<&mut Window>,
    cx: &mut App,
) {
    let glass_alpha = glass.then_some(glass_opacity.min(100) as f32 / 100.);
    let light_config = Rc::new(theme_config(
        light_theme.label(),
        light_theme.palette(),
        ThemeMode::Light,
        fonts,
        glass_alpha,
    ));
    let dark_config = Rc::new(theme_config(
        dark_theme.label(),
        dark_theme.palette(),
        ThemeMode::Dark,
        fonts,
        glass_alpha,
    ));
    let theme = Theme::global_mut(cx);
    theme.light_theme = light_config;
    theme.dark_theme = dark_config;
    cx.set_window_appearance(if follow_system {
        None
    } else {
        Some(match mode {
            ThemeMode::Light => WindowAppearance::Light,
            ThemeMode::Dark => WindowAppearance::Dark,
        })
    });
    Theme::change(mode, window, cx);
    Theme::global_mut(cx).list.active_highlight = false;
    let notification = &mut Theme::global_mut(cx).notification;
    notification.placement = Anchor::BottomCenter;
}

fn theme_config(
    name: &'static str,
    palette: Palette,
    mode: ThemeMode,
    fonts: &AppFonts,
    glass_alpha: Option<f32>,
) -> ThemeConfig {
    let panel = |color: &'static str, extra: f32| -> SharedString {
        match glass_alpha {
            Some(alpha) => with_alpha(color, alpha + extra),
            None => color.into(),
        }
    };
    let table = |color: &'static str| -> SharedString {
        if glass_alpha.is_some() {
            "#00000000".into()
        } else {
            color.into()
        }
    };
    let mut colors = ThemeConfigColors::default();
    colors.background = Some(panel(palette.background, 0.));
    colors.foreground = Some(palette.foreground.into());
    colors.border = Some(palette.border.into());
    colors.input = Some(palette.border.into());
    colors.ring = Some(palette.ring.into());
    colors.caret = Some(palette.primary.into());
    colors.selection = Some(palette.active.into());
    colors.link = Some(palette.primary.into());
    colors.link_hover = Some(palette.primary_hover.into());
    colors.link_active = Some(palette.primary_active.into());
    colors.muted = Some(panel(palette.muted, 0.04));
    colors.muted_foreground = Some(palette.muted_foreground.into());
    colors.accent = Some(panel(palette.accent, 0.04));
    colors.accent_foreground = Some(palette.accent_foreground.into());
    colors.primary = Some(palette.primary.into());
    colors.primary_foreground = Some(palette.primary_foreground.into());
    colors.primary_hover = Some(palette.primary_hover.into());
    colors.primary_active = Some(palette.primary_active.into());
    colors.secondary = Some(panel(palette.muted, 0.04));
    colors.secondary_foreground = Some(palette.subtext_foreground.into());
    colors.secondary_hover = Some(palette.hover.into());
    colors.secondary_active = Some(panel(palette.accent, 0.04));
    colors.danger = Some(palette.emotion.into());
    colors.danger_foreground = Some(palette.emotion_foreground.into());
    colors.danger_hover = Some(palette.emotion.into());
    colors.danger_active = Some(palette.emotion.into());
    colors.info = Some(palette.info.into());
    colors.info_foreground = Some(palette.info_foreground.into());
    colors.info_hover = Some(palette.info.into());
    colors.info_active = Some(palette.info.into());
    colors.success = Some(palette.success.into());
    colors.success_foreground = Some(palette.success_foreground.into());
    colors.success_hover = Some(palette.success.into());
    colors.success_active = Some(palette.success.into());
    colors.warning = Some(palette.warning.into());
    colors.warning_foreground = Some(palette.warning_foreground.into());
    colors.warning_hover = Some(palette.warning.into());
    colors.warning_active = Some(palette.warning.into());
    colors.button = Some(panel(palette.surface, 0.06));
    colors.button_foreground = Some(palette.foreground.into());
    colors.button_hover = Some(palette.hover.into());
    colors.button_active = Some(panel(palette.accent, 0.04));
    colors.button_primary = Some(palette.primary.into());
    colors.button_primary_foreground = Some(palette.primary_foreground.into());
    colors.button_primary_hover = Some(palette.primary_hover.into());
    colors.button_primary_active = Some(palette.primary_active.into());
    colors.button_secondary = Some(panel(palette.muted, 0.04));
    colors.button_secondary_foreground = Some(palette.foreground.into());
    colors.button_secondary_hover = Some(palette.hover.into());
    colors.button_secondary_active = Some(panel(palette.accent, 0.04));
    colors.group_box = Some(panel(palette.surface, 0.06));
    colors.group_box_foreground = Some(palette.foreground.into());
    colors.popover = Some(panel(palette.surface, 0.06));
    colors.popover_foreground = Some(palette.foreground.into());
    colors.sidebar = Some(panel(palette.sidebar, 0.06));
    colors.sidebar_foreground = Some(palette.foreground.into());
    colors.sidebar_border = Some(palette.border.into());
    colors.sidebar_primary = Some(palette.primary.into());
    colors.sidebar_primary_foreground = Some(palette.primary_foreground.into());
    colors.list = Some(panel(palette.sidebar, 0.06));
    colors.list_active = Some(palette.active.into());
    colors.list_active_border = Some("#00000000".into());
    colors.list_hover = Some(palette.hover.into());
    colors.list_even = Some(panel(palette.sidebar, 0.06));
    colors.list_head = Some(panel(palette.sidebar, 0.06));
    colors.table = Some(table(palette.background));
    colors.table_head = Some(table(palette.background));
    colors.table_head_foreground = Some(palette.muted_foreground.into());
    colors.table_hover = Some(palette.hover.into());
    colors.table_active = Some(palette.active.into());
    colors.table_active_border = Some("#00000000".into());
    colors.table_even = Some(table(palette.background));
    colors.table_row_border = Some("#00000000".into());
    colors.slider_bar = Some(palette.primary.into());
    colors.slider_thumb = Some(palette.primary.into());
    colors.progress_bar = Some(palette.primary.into());
    colors.scrollbar = Some("#00000000".into());
    colors.scrollbar_thumb = Some(palette.scrollbar_thumb.into());
    colors.scrollbar_thumb_hover = Some(palette.muted_foreground.into());
    colors.title_bar = Some(panel(palette.outer, 0.12));
    colors.title_bar_border = Some(palette.border.into());
    colors.status_bar = Some(panel(palette.outer, 0.12));
    colors.status_bar_border = Some(palette.border.into());
    colors.window_border = Some(palette.border.into());

    ThemeConfig {
        is_default: true,
        name: name.into(),
        mode,
        font_size: Some(14.),
        font_family: Some(fonts.ui.family.clone()),
        mono_font_family: Some(fonts.monospace.family.clone()),
        radius: Some(8),
        radius_lg: Some(12),
        shadow: Some(true),
        colors,
        ..Default::default()
    }
}

fn with_alpha(color: &'static str, alpha: f32) -> SharedString {
    let rgb =
        u32::from(gpui::Rgba::try_from(color).expect("palette colors are valid hex")) & 0xffffff00;
    let alpha = (alpha.clamp(0., 1.) * 255.).round() as u32;
    format!("#{:08x}", rgb | alpha).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_builtin_theme_has_a_stable_serialized_id() {
        for (theme, id) in [
            (ColorTheme::AyuLight, "ayu-light"),
            (ColorTheme::AyuDark, "ayu-dark"),
        ] {
            let json = serde_json::to_string(&theme).expect("serialize color theme");
            let restored: ColorTheme =
                serde_json::from_str(&json).expect("deserialize color theme");
            assert_eq!(restored, theme);
            assert_eq!(json, format!("\"{id}\""));
        }
    }

    #[test]
    fn ayu_palettes_use_valid_colors() {
        for theme in [ColorTheme::AyuLight, ColorTheme::AyuDark] {
            let palette = theme.palette();
            for color in [
                palette.background,
                palette.surface,
                palette.sidebar,
                palette.outer,
                palette.muted,
                palette.accent,
            ] {
                assert!(gpui::Rgba::try_from(color).is_ok(), "invalid color {color}");
            }
        }
    }

    #[test]
    fn glass_colors_clamp_alpha() {
        assert_eq!(with_alpha("#112233", 0.5).as_ref(), "#11223380");
        assert_eq!(with_alpha("#112233", 2.).as_ref(), "#112233ff");
        assert_eq!(with_alpha("#112233", -1.).as_ref(), "#11223300");
    }

    #[test]
    fn removed_themes_migrate_to_ayu() {
        let light: ColorTheme =
            serde_json::from_str("\"rose-pine-dawn\"").expect("deserialize removed light theme");
        assert_eq!(light, ColorTheme::AyuLight);
        let dark: ColorTheme =
            serde_json::from_str("\"one-dark\"").expect("deserialize removed dark theme");
        assert_eq!(dark, ColorTheme::AyuDark);
        let legacy: ColorTheme =
            serde_json::from_str("\"cadence-neutral\"").expect("deserialize legacy theme");
        assert_eq!(legacy, ColorTheme::AyuLight);
    }

    #[test]
    fn font_chain_skips_missing_families_and_preserves_order() {
        let configured = vec![
            "Missing Font".to_owned(),
            "inter".to_owned(),
            "Noto Sans CJK SC".to_owned(),
        ];
        let available = vec!["Inter".to_owned(), "Noto Sans CJK SC".to_owned()];
        let resolved = resolve_font_chain(&configured, ".SystemUIFont", &available);

        assert_eq!(resolved.family.as_ref(), "Inter");
        assert_eq!(
            resolved
                .fallbacks
                .expect("configured fallback")
                .fallback_list(),
            ["Noto Sans CJK SC"]
        );
    }
}
