use pathfinder_color::ColorU;
use pathfinder_geometry::vector::vec2f;
use warp_core::ui::theme::{Fill, WarpTheme};
use warpui::elements::DropShadow;

pub(crate) const LIGHT_ALC_THEME_NAME: &str = "light-alc";
pub(crate) const LIGHT_ALC_PANEL_BG: ColorU = ColorU {
    r: 0xf2,
    g: 0xf4,
    b: 0xf6,
    a: 0xff,
};
pub(crate) const LIGHT_ALC_PILL_BG: ColorU = ColorU {
    r: 0xff,
    g: 0xff,
    b: 0xff,
    a: 0xff,
};
pub(crate) const LIGHT_ALC_PILL_RADIUS: f32 = 8.;
pub(crate) const LIGHT_ALC_GROUP_ITEM_SPACING: f32 = 8.;
pub(crate) const LIGHT_ALC_TABS_MODE_ITEM_SPACING: f32 = 8.;
pub(crate) const LIGHT_ALC_GROUP_HEADER_VERTICAL_PADDING: f32 = 8.;
pub(crate) const LIGHT_ALC_GROUP_BODY_BOTTOM_PADDING: f32 = 12.;

pub(crate) fn is_light_alc_theme(theme: &WarpTheme) -> bool {
    theme.name().as_deref() == Some(LIGHT_ALC_THEME_NAME)
}

/// White selected pill only for light-alc rows without a pane/group color.
pub(crate) fn should_use_neutral_selected_pill(
    theme: &WarpTheme,
    has_pane_color: bool,
    parent_group_colored: bool,
) -> bool {
    is_light_alc_theme(theme) && !has_pane_color && !parent_group_colored
}

pub(crate) fn light_alc_neutral_selected_fill(theme: &WarpTheme) -> Option<Fill> {
    is_light_alc_theme(theme).then_some(Fill::Solid(LIGHT_ALC_PILL_BG))
}

pub(crate) fn light_alc_pill_drop_shadow() -> DropShadow {
    DropShadow {
        color: ColorU::new(0, 0, 0, 40),
        offset: vec2f(0., 1.),
        blur_radius: 3.,
        spread_radius: 0.,
    }
}

/// When `flush` would normally collapse gaps (GroupedTabs Panes), light-alc still
/// keeps air; other themes keep a hard `0`.
pub(crate) fn inter_row_spacing(theme: &WarpTheme, default: f32, flush: bool) -> f32 {
    if flush && !is_light_alc_theme(theme) {
        0.
    } else {
        group_item_spacing(theme, default)
    }
}

pub(crate) fn inter_tab_spacing(theme: &WarpTheme, default: f32, flush: bool) -> f32 {
    if flush && !is_light_alc_theme(theme) {
        0.
    } else {
        tabs_mode_item_spacing(theme, default)
    }
}

pub(crate) fn group_item_spacing(theme: &WarpTheme, default: f32) -> f32 {
    if is_light_alc_theme(theme) {
        LIGHT_ALC_GROUP_ITEM_SPACING
    } else {
        default
    }
}

pub(crate) fn tabs_mode_item_spacing(theme: &WarpTheme, default: f32) -> f32 {
    if is_light_alc_theme(theme) {
        LIGHT_ALC_TABS_MODE_ITEM_SPACING
    } else {
        default
    }
}

pub(crate) fn group_header_vertical_padding(theme: &WarpTheme, default: f32) -> f32 {
    if is_light_alc_theme(theme) {
        LIGHT_ALC_GROUP_HEADER_VERTICAL_PADDING
    } else {
        default
    }
}

pub(crate) fn group_body_bottom_padding(theme: &WarpTheme, default: f32) -> f32 {
    if is_light_alc_theme(theme) {
        LIGHT_ALC_GROUP_BODY_BOTTOM_PADDING
    } else {
        default
    }
}

pub(crate) fn selected_row_corner_radius(theme: &WarpTheme, default: f32) -> f32 {
    if is_light_alc_theme(theme) {
        LIGHT_ALC_PILL_RADIUS
    } else {
        default
    }
}

#[cfg(test)]
#[path = "light_alc_style_tests.rs"]
mod tests;
