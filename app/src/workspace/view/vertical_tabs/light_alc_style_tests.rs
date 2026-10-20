use super::*;
use crate::themes::default_themes::{light_alc, light_theme};
use crate::themes::theme::Fill;

#[test]
fn detects_light_alc_by_theme_name() {
    assert!(is_light_alc_theme(&light_alc()));
    assert!(!is_light_alc_theme(&light_theme()));
}

#[test]
fn neutral_selected_fill_only_for_light_alc() {
    assert_eq!(
        light_alc_neutral_selected_fill(&light_alc()),
        Some(Fill::Solid(LIGHT_ALC_PILL_BG))
    );
    assert_eq!(light_alc_neutral_selected_fill(&light_theme()), None);
}

#[test]
fn spacing_helpers_double_only_for_light_alc() {
    assert_eq!(
        group_item_spacing(&light_alc(), 4.),
        LIGHT_ALC_GROUP_ITEM_SPACING
    );
    assert_eq!(group_item_spacing(&light_theme(), 4.), 4.);
}

#[test]
fn flush_spacing_keeps_air_only_for_light_alc() {
    assert_eq!(
        inter_row_spacing(&light_alc(), 4., true),
        LIGHT_ALC_GROUP_ITEM_SPACING
    );
    assert_eq!(inter_row_spacing(&light_theme(), 4., true), 0.);
}

#[test]
fn neutral_pill_skips_colored_pane_and_group() {
    assert!(should_use_neutral_selected_pill(&light_alc(), false, false));
    assert!(!should_use_neutral_selected_pill(&light_alc(), true, false));
    assert!(!should_use_neutral_selected_pill(&light_alc(), false, true));
    assert!(!should_use_neutral_selected_pill(&light_theme(), false, false));
}
