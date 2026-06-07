//! Copyright © 2026 ComfyHome™
//! All rights reserved.
//!
//! Licensed under the ComfyGit SA-PS:DA License
//!
//! For details, see the LICENSE file in the repository root.

use ratatui_core::buffer::Buffer;
use ratatui_core::layout::Rect;
use ratatui_core::style::{Color, Style};
use ratatui_core::widgets::StatefulWidget;

use crate::config::{
    HorizontalPosition, OverflowPolicy, TabBarAlign, TabBarEnd, TabMargin, TabOrientation,
    TabPadding, TabReorderPolicy, TabWheelDirection, VerticalPosition,
};
use crate::layout::{
    auto_horizontal_tab_width, auto_vertical_tab_height, compute_viewport, effective_margin,
    effective_padding,
};
use crate::nav::TabNav;
use crate::reorder::try_reorder;
use crate::state::{TabNavState, TabReorderDrag};
use crate::{DEFAULT_INDICATOR, vertical_label};

fn draw(nav: TabNav<'_>, area: Rect, buf: &mut Buffer) {
    ratatui_core::widgets::Widget::render(nav, area, buf);
}

fn render_horizontal(tabs: &[&str], selected: usize, width: u16) -> Buffer {
    let area = Rect::new(0, 0, width, 3);
    let mut buf = Buffer::empty(area);
    draw(TabNav::new(tabs, selected), area, &mut buf);
    buf
}

fn render_vertical(tabs: &[&str], selected: usize, width: u16, height: u16) -> Buffer {
    let area = Rect::new(0, 0, width, height);
    let mut buf = Buffer::empty(area);
    draw(
        TabNav::new(tabs, selected).orientation(TabOrientation::Vertical),
        area,
        &mut buf,
    );
    buf
}

#[test]
fn vertical_label_helper() {
    assert_eq!(vertical_label("ROTATED"), "R\nO\nT\nA\nT\nE\nD");
    assert_eq!(vertical_label(""), "");
    assert_eq!(vertical_label("Hi"), "H\ni");
}

#[test]
fn default_margin_and_padding() {
    let horizontal = TabNav::new(&["Tab"], 0);
    assert_eq!(effective_margin(&horizontal), TabMargin::ZERO);
    assert_eq!(
        effective_padding(&horizontal),
        TabPadding::horizontal_default()
    );

    let vertical = TabNav::new(&["T\na\nb"], 0).orientation(TabOrientation::Vertical);
    assert_eq!(effective_margin(&vertical), TabMargin::vertical_default());
    assert_eq!(effective_padding(&vertical), TabPadding::vertical_default());
}

#[test]
fn margin_and_padding_overrides() {
    let nav = TabNav::new(&["Tab"], 0)
        .margin(TabMargin::horizontal(2, 4))
        .padding(TabPadding::uniform(1));
    assert_eq!(effective_margin(&nav), TabMargin::horizontal(2, 4));
    assert_eq!(effective_padding(&nav), TabPadding::uniform(1));
}

#[test]
fn empty_tabs_renders_nothing() {
    let area = Rect::new(0, 0, 40, 3);
    let mut buf = Buffer::empty(area);
    let expected = buf.clone();
    draw(TabNav::new(&[], 0), area, &mut buf);
    assert_eq!(buf, expected);
}

#[test]
fn insufficient_height_renders_nothing() {
    let area = Rect::new(0, 0, 40, 2);
    let mut buf = Buffer::empty(area);
    let expected = buf.clone();
    draw(TabNav::new(&["Tab"], 0), area, &mut buf);
    assert_eq!(buf, expected);
}

#[test]
fn insufficient_width_renders_nothing_vertical() {
    let area = Rect::new(0, 0, 2, 20);
    let mut buf = Buffer::empty(area);
    let expected = buf.clone();
    draw(
        TabNav::new(&["T\na\nb"], 0).orientation(TabOrientation::Vertical),
        area,
        &mut buf,
    );
    assert_eq!(buf, expected);
}

#[test]
fn single_tab_renders_three_rows() {
    let buf = render_horizontal(&["Hi"], 0, 30);
    let top_line = line_str(&buf, 0);
    let mid_line = line_str(&buf, 1);
    let bot_line = line_str(&buf, 2);

    assert!(top_line.starts_with("╭"));
    assert!(top_line.contains("╮"));
    assert!(mid_line.contains("Hi"));
    assert!(mid_line.contains("▸"));
    assert!(bot_line.starts_with("╯"));
}

#[test]
fn inactive_tab_has_junction_corners() {
    let buf = render_horizontal(&["A", "B"], 1, 30);
    let bot_line = line_str(&buf, 2);
    assert!(bot_line.starts_with("┴"));
}

#[test]
fn indicator_appears_on_active_tab() {
    let buf = render_horizontal(&["Tab"], 0, 20);
    let mid_line = line_str(&buf, 1);
    assert!(mid_line.contains("▸"));
}

#[test]
fn no_indicator_when_disabled() {
    let area = Rect::new(0, 0, 20, 3);
    let mut buf = Buffer::empty(area);
    draw(TabNav::new(&["Tab"], 0).indicator(None), area, &mut buf);
    let mid_line = line_str(&buf, 1);
    assert!(!mid_line.contains("▸"));
}

#[test]
fn overflow_tabs_are_omitted() {
    let area = Rect::new(0, 0, 20, 3);
    let mut buf = Buffer::empty(area);
    draw(
        TabNav::new(&["Long", "Overflow"], 0).overflow(OverflowPolicy::Truncate),
        area,
        &mut buf,
    );
    let mid_line = line_str(&buf, 1);
    assert!(mid_line.contains("Long"));
    assert!(!mid_line.contains("Overflow"));
}

#[test]
fn square_borders() {
    let area = Rect::new(0, 0, 20, 3);
    let mut buf = Buffer::empty(area);
    draw(
        TabNav::new(&["Tab"], 0).border_set(crate::tab_border::Sqr),
        area,
        &mut buf,
    );
    let top_line = line_str(&buf, 0);
    assert!(top_line.starts_with("┌"));
}

#[test]
fn horizontal_tab_width_calculation() {
    let pad = TabPadding::horizontal_default();
    assert_eq!(auto_horizontal_tab_width("Hi", pad, false), 10);
    assert_eq!(auto_horizontal_tab_width("", pad, false), 8);
    assert_eq!(auto_horizontal_tab_width("Nodes", pad, false), 13);
}

#[test]
fn vertical_tab_height_calculation() {
    let pad = TabPadding::vertical_default();
    assert_eq!(auto_vertical_tab_height("Hi", pad), 5);
    assert_eq!(auto_vertical_tab_height("", pad), 4);
    assert_eq!(auto_vertical_tab_height("A\nB\nC", pad), 7);
}

#[test]
fn horizontal_margin_shifts_tabs() {
    let area = Rect::new(0, 0, 20, 3);
    let mut buf = Buffer::empty(area);
    draw(
        TabNav::new(&["Tab"], 0).margin(TabMargin::horizontal(2, 0)),
        area,
        &mut buf,
    );
    let top_line = line_str(&buf, 0);
    assert!(top_line.starts_with("  ╭"));
}

#[test]
fn two_active_tabs_layout() {
    let buf = render_horizontal(&["A", "B"], 0, 30);
    let mid_line = line_str(&buf, 1);
    assert!(mid_line.contains("A"));
    assert!(mid_line.contains("B"));
}

#[test]
fn horizontal_tab_bar_end_sqr_selected() {
    let area = Rect::new(0, 0, 20, 3);
    let mut buf = Buffer::empty(area);
    draw(
        TabNav::new(&["Tab"], 0).tab_bar_end(TabBarEnd::Sqr),
        area,
        &mut buf,
    );
    let bot_line = line_str(&buf, 2);
    assert!(bot_line.starts_with('│'));
    assert!(bot_line.ends_with('┐'));
}

#[test]
fn horizontal_tab_bar_end_sqr_inactive_first() {
    let area = Rect::new(0, 0, 40, 3);
    let mut buf = Buffer::empty(area);
    draw(
        TabNav::new(&["One", "Two"], 1).tab_bar_end(TabBarEnd::Sqr),
        area,
        &mut buf,
    );
    let bot_line = line_str(&buf, 2);
    assert!(bot_line.starts_with('├'));
    assert!(bot_line.ends_with('┐'));
}

#[test]
fn horizontal_tab_bar_end_rnd_selected() {
    let area = Rect::new(0, 0, 20, 3);
    let mut buf = Buffer::empty(area);
    draw(
        TabNav::new(&["Tab"], 0).tab_bar_end(TabBarEnd::Rnd),
        area,
        &mut buf,
    );
    let bot_line = line_str(&buf, 2);
    assert!(bot_line.starts_with('│'));
    assert!(bot_line.ends_with('╮'));
}

#[test]
fn horizontal_tab_bar_end_sqr_bottom_selected() {
    let area = Rect::new(0, 0, 20, 5);
    let mut buf = Buffer::empty(area);
    draw(
        TabNav::new(&["Tab"], 0)
            .horizontal_position(HorizontalPosition::Bottom)
            .tab_bar_end(TabBarEnd::Sqr),
        area,
        &mut buf,
    );
    let strip_top = area.bottom() - 3;
    let top_line = line_str(&buf, strip_top);
    assert!(top_line.starts_with('│'));
    assert!(top_line.ends_with('┘'));
}

#[test]
fn horizontal_tab_bar_end_sqr_bottom_inactive_first() {
    let area = Rect::new(0, 0, 40, 5);
    let mut buf = Buffer::empty(area);
    draw(
        TabNav::new(&["One", "Two"], 1)
            .horizontal_position(HorizontalPosition::Bottom)
            .tab_bar_end(TabBarEnd::Sqr),
        area,
        &mut buf,
    );
    let strip_top = area.bottom() - 3;
    let top_line = line_str(&buf, strip_top);
    assert!(top_line.starts_with('├'));
    assert!(top_line.ends_with('┘'));
}

#[test]
fn horizontal_tab_bar_end_rnd_bottom_selected() {
    let area = Rect::new(0, 0, 20, 5);
    let mut buf = Buffer::empty(area);
    draw(
        TabNav::new(&["Tab"], 0)
            .horizontal_position(HorizontalPosition::Bottom)
            .tab_bar_end(TabBarEnd::Rnd),
        area,
        &mut buf,
    );
    let strip_top = area.bottom() - 3;
    let top_line = line_str(&buf, strip_top);
    assert!(top_line.starts_with('│'));
    assert!(top_line.ends_with('╯'));
}

#[test]
fn horizontal_tab_bar_end_center_align() {
    let area = Rect::new(0, 0, 60, 3);
    let mut buf = Buffer::empty(area);
    draw(
        TabNav::new(&["A", "B"], 0)
            .tab_bar_end(TabBarEnd::Sqr)
            .tab_bar_align(TabBarAlign::Center),
        area,
        &mut buf,
    );
    let bot_line = line_str(&buf, 2);
    assert_eq!(bot_line.chars().next(), Some('┌'));
    assert!(bot_line.ends_with('┐'));
}

#[test]
fn horizontal_tab_bar_end_center_align_bottom() {
    let area = Rect::new(0, 0, 60, 5);
    let mut buf = Buffer::empty(area);
    draw(
        TabNav::new(&["A", "B"], 0)
            .horizontal_position(HorizontalPosition::Bottom)
            .tab_bar_end(TabBarEnd::Sqr)
            .tab_bar_align(TabBarAlign::Center),
        area,
        &mut buf,
    );
    let strip_top = area.bottom() - 3;
    let top_line = line_str(&buf, strip_top);
    assert_eq!(top_line.chars().next(), Some('└'));
    assert!(top_line.ends_with('┘'));
}

#[test]
fn horizontal_tab_bar_end_center_align_rnd_top() {
    let area = Rect::new(0, 0, 60, 3);
    let mut buf = Buffer::empty(area);
    draw(
        TabNav::new(&["A", "B"], 0)
            .tab_bar_end(TabBarEnd::Rnd)
            .tab_bar_align(TabBarAlign::Center),
        area,
        &mut buf,
    );
    let bot_line = line_str(&buf, 2);
    assert_eq!(bot_line.chars().next(), Some('╭'));
    assert!(bot_line.ends_with('╮'));
}

#[test]
fn horizontal_tab_bar_end_end_align() {
    let area = Rect::new(0, 0, 60, 3);
    let mut buf = Buffer::empty(area);
    draw(
        TabNav::new(&["A", "B"], 1)
            .tab_bar_end(TabBarEnd::Sqr)
            .tab_bar_align(TabBarAlign::End),
        area,
        &mut buf,
    );
    let bot_line = line_str(&buf, 2);
    assert_eq!(bot_line.chars().next(), Some('┌'));
    assert!(bot_line.ends_with('│'));
}

#[test]
fn horizontal_tab_bar_end_end_align_last_not_selected() {
    let area = Rect::new(0, 0, 60, 3);
    let mut buf = Buffer::empty(area);
    draw(
        TabNav::new(&["A", "B"], 0)
            .tab_bar_end(TabBarEnd::Sqr)
            .tab_bar_align(TabBarAlign::End),
        area,
        &mut buf,
    );
    let bot_line = line_str(&buf, 2);
    assert_eq!(bot_line.chars().next(), Some('┌'));
    assert!(bot_line.ends_with('┤'));
}

#[test]
fn vertical_tab_bar_end_center_align() {
    let first = vertical_label("One");
    let second = vertical_label("Two");
    let tabs = [first.as_str(), second.as_str()];
    let nav = TabNav::new(&tabs, 0)
        .orientation(TabOrientation::Vertical)
        .tab_bar_end(TabBarEnd::Sqr)
        .tab_bar_align(TabBarAlign::Center);
    let width = nav.vertical_rail_width();
    let area = Rect::new(0, 0, width, 30);
    let mut buf = Buffer::empty(area);
    draw(nav, area, &mut buf);
    let right_col = col_str(&buf, width - 1);
    assert_eq!(right_col.chars().next(), Some('┌'));
    assert!(right_col.ends_with('└'));
}

#[test]
fn vertical_tab_bar_end_center_align_rnd_right_position() {
    let first = vertical_label("One");
    let second = vertical_label("Two");
    let tabs = [first.as_str(), second.as_str()];
    let nav = TabNav::new(&tabs, 0)
        .orientation(TabOrientation::Vertical)
        .vertical_position(VerticalPosition::Right)
        .tab_bar_end(TabBarEnd::Rnd)
        .tab_bar_align(TabBarAlign::Center);
    let width = nav.vertical_rail_width();
    let area = Rect::new(0, 0, width + 4, 30);
    let mut buf = Buffer::empty(area);
    draw(nav, area, &mut buf);
    let baseline_x = area.right() - width;
    let baseline_col: String = (0..area.height)
        .map(|y| buf[(baseline_x, y)].symbol().to_string())
        .collect();
    assert_eq!(baseline_col.chars().next(), Some('╮'));
    assert!(baseline_col.ends_with('╯'));
}

#[test]
fn horizontal_tab_bar_end_rnd_inactive_first() {
    let area = Rect::new(0, 0, 40, 3);
    let mut buf = Buffer::empty(area);
    draw(
        TabNav::new(&["One", "Two"], 1).tab_bar_end(TabBarEnd::Rnd),
        area,
        &mut buf,
    );
    let bot_line = line_str(&buf, 2);
    assert!(bot_line.starts_with('├'));
    assert!(bot_line.ends_with('╮'));
}

#[test]
fn all_caps_renders_uppercase_labels() {
    let area = Rect::new(0, 0, 30, 3);
    let mut buf = Buffer::empty(area);
    draw(TabNav::new(&["Example"], 0).all_caps(true), area, &mut buf);
    let mid_line = line_str(&buf, 1);
    assert!(mid_line.contains("EXAMPLE"));
    assert!(!mid_line.contains("Example"));
}

#[test]
fn vertical_tab_bar_end_rnd() {
    let label = vertical_label("Tab");
    let tabs = [label.as_str()];
    let nav = TabNav::new(&tabs, 0)
        .orientation(TabOrientation::Vertical)
        .tab_bar_end(TabBarEnd::Rnd);
    let width = nav.vertical_rail_width();
    let height = auto_vertical_tab_height(tabs[0], TabPadding::vertical_default());
    let area = Rect::new(0, 0, width, height);
    let mut buf = Buffer::empty(area);
    draw(nav, area, &mut buf);
    let right_col = col_str(&buf, width - 1);
    assert!(right_col.starts_with('─'));
    assert!(right_col.ends_with('─'));
}

#[test]
fn vertical_tab_bar_end_rnd_right_position() {
    let first = vertical_label("One");
    let second = vertical_label("Two");
    let tabs = [first.as_str(), second.as_str()];
    let nav = TabNav::new(&tabs, 1)
        .orientation(TabOrientation::Vertical)
        .vertical_position(VerticalPosition::Right)
        .tab_bar_end(TabBarEnd::Rnd);
    let width = nav.vertical_rail_width();
    let height = auto_vertical_tab_height(tabs[0], TabPadding::vertical_default())
        + auto_vertical_tab_height(tabs[1], TabPadding::vertical_default());
    let area = Rect::new(0, 0, width + 4, height);
    let mut buf = Buffer::empty(area);
    draw(nav, area, &mut buf);
    let baseline_x = area.right() - width;
    let baseline_col: String = (0..height)
        .map(|y| buf[(baseline_x, y)].symbol().to_string())
        .collect();
    assert!(baseline_col.starts_with('┬'));
    assert!(baseline_col.ends_with('─'));
}

#[test]
fn vertical_tab_bar_end_sqr_right_selected_first() {
    let label = vertical_label("Tab");
    let tabs = [label.as_str()];
    let nav = TabNav::new(&tabs, 0)
        .orientation(TabOrientation::Vertical)
        .vertical_position(VerticalPosition::Right)
        .tab_bar_end(TabBarEnd::Sqr);
    let width = nav.vertical_rail_width();
    let height = auto_vertical_tab_height(tabs[0], TabPadding::vertical_default());
    let area = Rect::new(0, 0, width + 4, height);
    let mut buf = Buffer::empty(area);
    draw(nav, area, &mut buf);
    let baseline_x = area.right() - width;
    let baseline_col: String = (0..height)
        .map(|y| buf[(baseline_x, y)].symbol().to_string())
        .collect();
    assert!(baseline_col.starts_with('─'));
    assert!(baseline_col.ends_with('─'));
}

#[test]
fn vertical_exact_fit_start_last_tab_inactive_junction() {
    let first = vertical_label("One");
    let second = vertical_label("Two");
    let tabs = [first.as_str(), second.as_str()];
    let nav = TabNav::new(&tabs, 0)
        .orientation(TabOrientation::Vertical)
        .tab_bar_end(TabBarEnd::Sqr)
        .tab_bar_align(TabBarAlign::Start)
        .overflow(OverflowPolicy::Truncate);
    let width = nav.vertical_rail_width();
    let height = nav.auto_tab_height(0).unwrap() + nav.auto_tab_height(1).unwrap();
    let area = Rect::new(0, 0, width, height);
    let mut buf = Buffer::empty(area);
    draw(nav, area, &mut buf);
    let baseline_x = width - 1;
    let last_y = height - 1;
    assert_eq!(buf[(baseline_x, last_y)].symbol(), "┴");
}

#[test]
fn vertical_exact_fit_start_right_position_last_tab_inactive_junction() {
    let first = vertical_label("One");
    let second = vertical_label("Two");
    let tabs = [first.as_str(), second.as_str()];
    let nav = TabNav::new(&tabs, 0)
        .orientation(TabOrientation::Vertical)
        .vertical_position(VerticalPosition::Right)
        .tab_bar_end(TabBarEnd::Sqr)
        .tab_bar_align(TabBarAlign::Start)
        .overflow(OverflowPolicy::Truncate);
    let width = nav.vertical_rail_width();
    let height = nav.auto_tab_height(0).unwrap() + nav.auto_tab_height(1).unwrap();
    let area = Rect::new(0, 0, width + 4, height);
    let mut buf = Buffer::empty(area);
    draw(nav, area, &mut buf);
    let baseline_x = area.right() - width;
    let last_y = height - 1;
    assert_eq!(
        buf[(baseline_x, last_y)].symbol(),
        "┴",
        "last tab baseline junction"
    );
}

#[test]
fn vertical_exact_fit_start_right_position_last_tab_selected_junction() {
    let first = vertical_label("One");
    let second = vertical_label("Two");
    let tabs = [first.as_str(), second.as_str()];
    let nav = TabNav::new(&tabs, 1)
        .orientation(TabOrientation::Vertical)
        .vertical_position(VerticalPosition::Right)
        .tab_bar_end(TabBarEnd::Rnd)
        .tab_bar_align(TabBarAlign::Start)
        .overflow(OverflowPolicy::Truncate);
    let width = nav.vertical_rail_width();
    let height = nav.auto_tab_height(0).unwrap() + nav.auto_tab_height(1).unwrap();
    let area = Rect::new(0, 0, width + 4, height);
    let mut buf = Buffer::empty(area);
    draw(nav, area, &mut buf);
    let baseline_x = area.right() - width;
    let last_y = height - 1;
    assert_eq!(buf[(baseline_x, last_y)].symbol(), "─");
}

#[test]
fn vertical_exact_fit_end_first_tab_inactive_junction() {
    let first = vertical_label("One");
    let second = vertical_label("Two");
    let tabs = [first.as_str(), second.as_str()];
    let nav = TabNav::new(&tabs, 1)
        .orientation(TabOrientation::Vertical)
        .tab_bar_end(TabBarEnd::Rnd)
        .tab_bar_align(TabBarAlign::End)
        .overflow(OverflowPolicy::Truncate);
    let width = nav.vertical_rail_width();
    let height = nav.auto_tab_height(0).unwrap() + nav.auto_tab_height(1).unwrap();
    let area = Rect::new(0, 0, width, height);
    let mut buf = Buffer::empty(area);
    draw(nav, area, &mut buf);
    let baseline_x = width - 1;
    assert_eq!(buf[(baseline_x, 0)].symbol(), "┬");
    assert_eq!(buf[(baseline_x, height - 1)].symbol(), "─");
}

#[test]
fn vertical_exact_fit_end_right_position_first_tab_inactive_junction() {
    let first = vertical_label("One");
    let second = vertical_label("Two");
    let tabs = [first.as_str(), second.as_str()];
    let nav = TabNav::new(&tabs, 1)
        .orientation(TabOrientation::Vertical)
        .vertical_position(VerticalPosition::Right)
        .tab_bar_end(TabBarEnd::Sqr)
        .tab_bar_align(TabBarAlign::End)
        .overflow(OverflowPolicy::Truncate);
    let width = nav.vertical_rail_width();
    let height = nav.auto_tab_height(0).unwrap() + nav.auto_tab_height(1).unwrap();
    let area = Rect::new(0, 0, width + 4, height);
    let mut buf = Buffer::empty(area);
    draw(nav, area, &mut buf);
    let baseline_x = area.right() - width;
    assert_eq!(buf[(baseline_x, 0)].symbol(), "┬");
    assert_eq!(buf[(baseline_x, height - 1)].symbol(), "─");
}

#[test]
fn demo_vertical_exact_height_start_right_ui_tab_inactive() {
    let labels = demo_vertical_labels();
    let tab_refs: Vec<&str> = labels.iter().map(String::as_str).collect();
    let nav = TabNav::new(&tab_refs, 4)
        .orientation(TabOrientation::Vertical)
        .vertical_position(VerticalPosition::Right)
        .tab_bar_end(TabBarEnd::Sqr)
        .tab_bar_align(TabBarAlign::Start)
        .overflow(OverflowPolicy::Truncate);
    let height = demo_vertical_total_height(&nav);
    let width = nav.vertical_rail_width();
    let area = Rect::new(0, 0, width + 4, height);
    let mut buf = Buffer::empty(area);
    draw(nav, area, &mut buf);
    let baseline_x = area.right() - width;
    assert_eq!(buf[(baseline_x, height - 1)].symbol(), "┴");
}

#[test]
fn demo_vertical_exact_height_end_right_logs_tab_selected() {
    let labels = demo_vertical_labels();
    let tab_refs: Vec<&str> = labels.iter().map(String::as_str).collect();
    let nav = TabNav::new(&tab_refs, 6)
        .orientation(TabOrientation::Vertical)
        .vertical_position(VerticalPosition::Right)
        .tab_bar_end(TabBarEnd::Rnd)
        .tab_bar_align(TabBarAlign::End)
        .overflow(OverflowPolicy::Truncate);
    let height = demo_vertical_total_height(&nav);
    let width = nav.vertical_rail_width();
    let area = Rect::new(0, 0, width + 4, height);
    let mut buf = Buffer::empty(area);
    draw(nav, area, &mut buf);
    let baseline_x = area.right() - width;
    assert_eq!(buf[(baseline_x, 0)].symbol(), "┬");
    assert_eq!(buf[(baseline_x, height - 1)].symbol(), "─");
}

#[test]
fn vertical_default_indicator_disabled() {
    let label = vertical_label("Tab");
    let tabs = [label.as_str()];
    let nav = TabNav::new(&tabs, 0).orientation(TabOrientation::Vertical);
    let width = nav.vertical_rail_width();
    let height = auto_vertical_tab_height(tabs[0], TabPadding::vertical_default());
    let buf = render_vertical(&tabs, 0, width, height);
    let label_col = col_str(&buf, 2);
    assert!(!label_col.contains('▸'));
}

#[test]
fn vertical_single_tab_renders_stacked_label() {
    let label = vertical_label("Log");
    let label = label.as_str();
    let pad = TabPadding::vertical_default();
    let width = auto_horizontal_tab_width(label, pad, false);
    let height = auto_vertical_tab_height(label, pad);
    let area = Rect::new(0, 0, width, height);
    let mut buf = Buffer::empty(area);
    draw(
        TabNav::new(&[label], 0)
            .orientation(TabOrientation::Vertical)
            .indicator(Some(DEFAULT_INDICATOR)),
        area,
        &mut buf,
    );
    let label_col = col_str(&buf, 2);

    assert!(label_col.contains("L"));
    assert!(label_col.contains("o"));
    assert!(label_col.contains("g"));
    assert_eq!(buf[(2, 1)].symbol(), "▸");
    assert_eq!(buf[(2, 2)].symbol(), "L");
}

#[test]
fn vertical_active_tab_top_border_uses_top_right_corner() {
    let label = vertical_label("Tab");
    let tabs = [label.as_str()];
    let nav = TabNav::new(&tabs, 0).orientation(TabOrientation::Vertical);
    let width = nav.vertical_rail_width();
    let height = auto_vertical_tab_height(tabs[0], TabPadding::vertical_default());
    let buf = render_vertical(&tabs, 0, width, height);
    let top_line = line_str(&buf, 0);

    assert!(top_line.starts_with('╭'));
    assert!(top_line.ends_with('╯'));
}

#[test]
fn vertical_active_tab_opens_right() {
    let label = vertical_label("Tab");
    let tabs = [label.as_str()];
    let nav = TabNav::new(&tabs, 0).orientation(TabOrientation::Vertical);
    let width = nav.vertical_rail_width();
    let height = auto_vertical_tab_height(tabs[0], TabPadding::vertical_default());
    let buf = render_vertical(&tabs, 0, width, height);
    let active_col: String = (0..height)
        .map(|y| buf[(width - 1, y)].symbol().to_string())
        .collect();
    let glyphs: Vec<char> = active_col.chars().collect();

    assert_eq!(glyphs.first(), Some(&'╯'));
    assert!(glyphs[1..glyphs.len() - 1].iter().all(|&ch| ch == ' '));
    assert_eq!(glyphs.last(), Some(&'╮'));
}

#[test]
fn vertical_inactive_tab_has_right_junction() {
    let first = vertical_label("One");
    let second = vertical_label("Two");
    let first = first.as_str();
    let second = second.as_str();
    let pad = TabPadding::vertical_default();
    let width = auto_horizontal_tab_width(first, pad, false);
    let height = auto_vertical_tab_height(first, pad) + auto_vertical_tab_height(second, pad);
    let buf = render_vertical(&[first, second], 1, width, height);
    let right_col = col_str(&buf, width - 1);

    assert!(right_col.contains('┤'));
}

#[test]
fn vertical_overflow_tabs_are_omitted() {
    let tall = vertical_label("ABCDEFGHIJ");
    let tall = tall.as_str();
    let also = vertical_label("X");
    let also = also.as_str();
    let pad = TabPadding::vertical_default();
    let width = auto_horizontal_tab_width(tall, pad, false);
    let height = auto_vertical_tab_height(tall, pad);
    let area = Rect::new(0, 0, width, height);
    let mut buf = Buffer::empty(area);
    draw(
        TabNav::new(&[tall, also], 0)
            .orientation(TabOrientation::Vertical)
            .overflow(OverflowPolicy::Truncate),
        area,
        &mut buf,
    );
    let col = col_str(&buf, 2);

    assert!(col.contains('A'));
    assert!(!col.contains('X'));
}

#[test]
fn horizontal_center_aligns_tabs_in_wide_area() {
    let tabs = ["A", "B"];
    let nav = TabNav::new(&tabs, 0).tab_bar_align(TabBarAlign::Center);
    let area = Rect::new(0, 0, 80, 3);
    let rects = nav.tab_rects(area);

    assert_eq!(rects.len(), 2);
    let group_width = rects[1].x + rects[1].width - rects[0].x;
    let expected_start = (area.width - group_width) / 2;
    assert_eq!(rects[0].x, expected_start);
}

#[test]
fn horizontal_end_aligns_tabs_in_wide_area() {
    let tabs = ["A", "B"];
    let nav = TabNav::new(&tabs, 0).tab_bar_align(TabBarAlign::End);
    let area = Rect::new(0, 0, 80, 3);
    let rects = nav.tab_rects(area);

    assert_eq!(rects.len(), 2);
    assert_eq!(rects[1].x + rects[1].width, area.right());
}

#[test]
fn horizontal_end_truncate_tabs_do_not_overlap() {
    let tabs = ["AAA", "BBB", "CCC", "DDD"];
    let nav = TabNav::new(&tabs, 0).tab_bar_align(TabBarAlign::End);
    let area = Rect::new(0, 0, 24, 3);
    let rects = nav.tab_rects(area);

    assert!(rects.len() >= 2);
    for (i, left) in rects.iter().enumerate() {
        for right in &rects[i + 1..] {
            assert!(
                left.x + left.width <= right.x || right.x + right.width <= left.x,
                "tabs overlap: {left:?} {right:?}"
            );
        }
    }
    assert_eq!(
        rects.last().unwrap().x + rects.last().unwrap().width,
        area.right()
    );
}

#[test]
fn horizontal_end_scroll_renders_without_buffer_panic() {
    let tabs = ["One", "Two", "Three", "Four", "Five"];
    let area = Rect::new(0, 0, 28, 3);
    let nav = TabNav::new(&tabs, 4)
        .tab_bar_align(TabBarAlign::End)
        .overflow(OverflowPolicy::Scroll);
    let mut state = TabNavState::new(4);
    state.ensure_selected_visible(&nav, area);

    let rects = nav.tab_rects_with_scroll(area, state.scroll_offset);
    for rect in &rects {
        assert!(rect.x + rect.width <= area.right());
        assert!(rect.right() <= area.right());
    }

    let mut buf = Buffer::empty(area);
    draw(nav, area, &mut buf);
}

#[test]
fn vertical_end_scroll_renders_without_buffer_panic() {
    let labels: Vec<String> = ["One", "Two", "Three", "Four", "Five"]
        .into_iter()
        .map(vertical_label)
        .collect();
    let tabs: Vec<&str> = labels.iter().map(String::as_str).collect();
    let nav = TabNav::new(&tabs, 4)
        .orientation(TabOrientation::Vertical)
        .tab_bar_align(TabBarAlign::End)
        .overflow(OverflowPolicy::Scroll);
    let width = nav.vertical_rail_width();
    let area = Rect::new(0, 0, width, 12);
    let mut state = TabNavState::new(4);
    state.ensure_selected_visible(&nav, area);

    let rects = nav.tab_rects_with_scroll(area, state.scroll_offset);
    for rect in &rects {
        assert!(rect.y + rect.height <= area.bottom());
        assert!(rect.bottom() <= area.bottom());
    }

    let mut buf = Buffer::empty(area);
    draw(nav, area, &mut buf);
}

#[test]
fn vertical_end_truncate_tabs_do_not_overlap() {
    let labels: Vec<String> = ["A", "B", "C", "D"]
        .into_iter()
        .map(vertical_label)
        .collect();
    let tabs: Vec<&str> = labels.iter().map(String::as_str).collect();
    let nav = TabNav::new(&tabs, 0)
        .orientation(TabOrientation::Vertical)
        .tab_bar_align(TabBarAlign::End);
    let width = nav.vertical_rail_width();
    let tab_height = nav.auto_tab_height(0).unwrap();
    let area = Rect::new(0, 0, width, tab_height * 2 + 1);
    let rects = nav.tab_rects(area);

    assert!(rects.len() >= 2);
    for (i, upper) in rects.iter().enumerate() {
        for lower in &rects[i + 1..] {
            assert!(
                upper.y + upper.height <= lower.y || lower.y + lower.height <= upper.y,
                "tabs overlap: {upper:?} {lower:?}"
            );
        }
    }
    assert_eq!(
        rects.last().unwrap().y + rects.last().unwrap().height,
        area.bottom()
    );
}

#[test]
fn vertical_end_aligns_tabs_in_tall_area() {
    let first = vertical_label("One");
    let second = vertical_label("Two");
    let first = first.as_str();
    let second = second.as_str();
    let tabs = [first, second];
    let nav = TabNav::new(&tabs, 0)
        .orientation(TabOrientation::Vertical)
        .tab_bar_align(TabBarAlign::End);
    let width = nav.vertical_rail_width();
    let area = Rect::new(0, 0, width, 30);
    let rects = nav.tab_rects(area);

    assert_eq!(rects.len(), 2);
    assert_eq!(rects[1].y + rects[1].height, area.bottom());
}

#[test]
fn tab_rects_match_auto_horizontal_widths() {
    let tabs = ["Files", "Search"];
    let nav = TabNav::new(&tabs, 0);
    let area = Rect::new(5, 2, 40, 3);
    let rects = nav.tab_rects(area);

    assert_eq!(rects.len(), 2);
    assert_eq!(rects[0].x, area.x);
    assert_eq!(rects[0].width, nav.auto_tab_width(0).unwrap());
    assert_eq!(rects[0].height, nav.horizontal_strip_height());
    assert_eq!(rects[1].x, rects[0].x + rects[0].width);
    assert_eq!(rects[1].width, nav.auto_tab_width(1).unwrap());
}

#[test]
fn tab_widths_override_auto_layout() {
    let tabs = ["A", "B"];
    let widths = [20, 12];
    let nav = TabNav::new(&tabs, 0).tab_widths(&widths);
    let rects = nav.tab_rects(Rect::new(0, 0, 40, 3));

    assert_eq!(rects.len(), 2);
    assert_eq!(rects[0].width, 20);
    assert_eq!(rects[1].width, 12);
    assert_ne!(rects[0].width, nav.auto_tab_width(0).unwrap());
}

#[test]
fn tab_rects_respect_margin_and_overflow() {
    let nav = TabNav::new(&["Long", "Overflow"], 0)
        .margin(TabMargin::horizontal(2, 0))
        .overflow(OverflowPolicy::Truncate);
    let area = Rect::new(0, 0, 20, 3);
    let rects = nav.tab_rects(area);

    assert_eq!(rects.len(), 1);
    assert_eq!(rects[0].x, 2);
}

#[test]
fn horizontal_bottom_strip_anchors_to_area_bottom() {
    let tabs = ["A", "B"];
    let nav = TabNav::new(&tabs, 0).horizontal_position(HorizontalPosition::Bottom);
    let area = Rect::new(0, 4, 40, 7);
    let rects = nav.tab_rects(area);

    assert_eq!(rects.len(), 2);
    assert_eq!(rects[0].y, area.bottom() - nav.horizontal_strip_height());
    assert_eq!(rects[0].height, nav.horizontal_strip_height());
}

#[test]
fn vertical_right_rail_anchors_to_area_right() {
    let label = vertical_label("Tab");
    let tabs = [label.as_str()];
    let nav = TabNav::new(&tabs, 0)
        .orientation(TabOrientation::Vertical)
        .vertical_position(VerticalPosition::Right);
    let width = nav.vertical_rail_width();
    let area = Rect::new(0, 0, width + 8, nav.auto_tab_height(0).unwrap() + 4);
    let rects = nav.tab_rects(area);

    assert_eq!(rects.len(), 1);
    assert_eq!(rects[0].x, area.right() - width);
    assert_eq!(rects[0].width, width);
}

#[test]
fn vertical_right_active_tab_opens_left() {
    let label = vertical_label("Tab");
    let tabs = [label.as_str()];
    let nav = TabNav::new(&tabs, 0)
        .orientation(TabOrientation::Vertical)
        .vertical_position(VerticalPosition::Right);
    let width = nav.vertical_rail_width();
    let height = auto_vertical_tab_height(tabs[0], TabPadding::vertical_default());
    let area = Rect::new(0, 0, width + 4, height);
    let mut buf = Buffer::empty(area);
    draw(nav, area, &mut buf);
    let active_col: String = (0..height)
        .map(|y| buf[(area.right() - width, y)].symbol().to_string())
        .collect();
    let glyphs: Vec<char> = active_col.chars().collect();

    assert_eq!(glyphs.first(), Some(&'╰'));
    assert!(glyphs[1..glyphs.len() - 1].iter().all(|&ch| ch == ' '));
    assert_eq!(glyphs.last(), Some(&'╭'));
}

#[test]
fn horizontal_bottom_active_tab_opens_up() {
    let area = Rect::new(0, 0, 30, 5);
    let mut buf = Buffer::empty(area);
    draw(
        TabNav::new(&["Hi"], 0).horizontal_position(HorizontalPosition::Bottom),
        area,
        &mut buf,
    );
    let strip_top = area.bottom() - 3;
    let strip_bottom = area.bottom() - 1;
    let top_line = line_str(&buf, strip_top);
    let label_line = line_str(&buf, strip_top + 1);
    let bottom_line = line_str(&buf, strip_bottom);

    assert!(top_line.starts_with('╮'));
    assert!(label_line.contains("Hi"));
    assert!(bottom_line.contains('╰'));
    assert!(bottom_line.contains('╯'));
}

#[test]
fn tab_rects_vertical_match_auto_heights() {
    let label = vertical_label("Tab");
    let tabs = [label.as_str()];
    let nav = TabNav::new(&tabs, 0).orientation(TabOrientation::Vertical);
    let width = nav.vertical_rail_width();
    let height = nav.auto_tab_height(0).unwrap();
    let area = Rect::new(0, 0, width, height + 4);
    let rects = nav.tab_rects(area);

    assert_eq!(rects.len(), 1);
    assert_eq!(rects[0].y, area.y);
    assert_eq!(rects[0].width, width);
    assert_eq!(rects[0].height, height);
}

#[test]
fn unicode_label_uses_display_width() {
    let pad = TabPadding::horizontal_default();
    assert_eq!(auto_horizontal_tab_width("日本", pad, false), 12);
    assert_eq!(
        auto_horizontal_tab_width("日本", pad, false),
        auto_horizontal_tab_width("abcd", pad, false)
    );
}

#[test]
fn scroll_mode_shows_later_tabs() {
    let tabs = ["One", "Two", "Three", "Four"];
    let area = Rect::new(0, 0, 28, 3);
    let mut buf = Buffer::empty(area);
    draw(
        TabNav::new(&tabs, 3)
            .overflow(OverflowPolicy::Scroll)
            .scroll_offset(2),
        area,
        &mut buf,
    );
    let mid_line = line_str(&buf, 1);
    assert!(!mid_line.contains("One"));
    assert!(mid_line.contains("Three") || mid_line.contains("Four"));
}

const DEMO_TABS: [&str; 7] = [
    "Overview", "Nodes", "Network", "Content", "UI", "Config", "Logs",
];

fn demo_nav(selected: usize, end: TabBarEnd, align: TabBarAlign) -> TabNav<'static> {
    TabNav::new(&DEMO_TABS, selected)
        .tab_bar_end(end)
        .tab_bar_align(align)
}

fn demo_total_width(nav: &TabNav<'_>) -> u16 {
    (0..nav.tabs.len())
        .map(|index| nav.auto_tab_width(index).unwrap())
        .sum()
}

fn demo_vertical_total_height(nav: &TabNav<'_>) -> u16 {
    (0..nav.tabs.len())
        .map(|index| nav.auto_tab_height(index).unwrap())
        .sum()
}

fn demo_vertical_labels() -> Vec<String> {
    DEMO_TABS
        .iter()
        .map(|label| vertical_label(label))
        .collect()
}

#[test]
fn demo_exact_width_95_start_sqr_end_caps() {
    let nav = demo_nav(0, TabBarEnd::Sqr, TabBarAlign::Start);
    let width = demo_total_width(&nav);
    assert_eq!(width, 95);
    let area = Rect::new(0, 0, width, 3);
    let mut buf = Buffer::empty(area);
    draw(nav, area, &mut buf);
    let baseline_y = 2;
    assert_eq!(
        buf[(0, baseline_y)].symbol(),
        "│",
        "leading cap at width {width}"
    );
    assert_eq!(
        buf[(width - 1, baseline_y)].symbol(),
        "┤",
        "trailing junction at width {width}"
    );
}

#[test]
fn demo_exact_width_95_start_sqr_last_tab_selected() {
    let nav = demo_nav(6, TabBarEnd::Sqr, TabBarAlign::Start);
    let width = demo_total_width(&nav);
    let area = Rect::new(0, 0, width, 3);
    let mut buf = Buffer::empty(area);
    draw(nav, area, &mut buf);
    assert_eq!(buf[(width - 1, 2)].symbol(), "│");
}

#[test]
fn demo_exact_width_96_start_sqr_end_caps() {
    let nav = demo_nav(0, TabBarEnd::Sqr, TabBarAlign::Start);
    let width = demo_total_width(&nav) + 1;
    assert_eq!(width, 96);
    let area = Rect::new(0, 0, width, 3);
    let mut buf = Buffer::empty(area);
    draw(nav, area, &mut buf);
    let baseline_y = 2;
    assert_eq!(buf[(0, baseline_y)].symbol(), "│");
    assert_eq!(buf[(width - 1, baseline_y)].symbol(), "┐");
}

#[test]
fn demo_exact_width_96_start_sqr_trailing_junction() {
    let nav = demo_nav(0, TabBarEnd::Sqr, TabBarAlign::Start);
    let width = demo_total_width(&nav) + 1;
    let area = Rect::new(0, 0, width, 3);
    let mut buf = Buffer::empty(area);
    draw(nav, area, &mut buf);
    let baseline_y = 2;
    assert_eq!(buf[(width - 2, baseline_y)].symbol(), "┴");
    assert_eq!(buf[(width - 1, baseline_y)].symbol(), "┐");
}

#[test]
fn demo_exact_width_95_end_align_sqr_trailing_cap() {
    let nav = demo_nav(1, TabBarEnd::Sqr, TabBarAlign::End);
    let width = demo_total_width(&nav);
    let area = Rect::new(0, 0, width, 3);
    let mut buf = Buffer::empty(area);
    draw(nav, area, &mut buf);
    assert_eq!(buf[(0, 2)].symbol(), "├");
    assert_eq!(buf[(width - 1, 2)].symbol(), "┤");
}

#[test]
fn demo_exact_width_96_end_align_sqr_trailing_cap() {
    let nav = demo_nav(1, TabBarEnd::Sqr, TabBarAlign::End);
    let width = demo_total_width(&nav) + 1;
    let area = Rect::new(0, 0, width, 3);
    let mut buf = Buffer::empty(area);
    draw(nav, area, &mut buf);
    assert_eq!(buf[(0, 2)].symbol(), "┌");
    assert_eq!(buf[(width - 1, 2)].symbol(), "┤");
}

#[test]
fn demo_exact_width_95_center_sqr_trailing_cap() {
    let nav = demo_nav(0, TabBarEnd::Sqr, TabBarAlign::Center);
    let width = demo_total_width(&nav);
    let area = Rect::new(0, 0, width, 3);
    let mut buf = Buffer::empty(area);
    draw(nav, area, &mut buf);
    assert_eq!(buf[(0, 2)].symbol(), "│");
    assert_eq!(buf[(width - 1, 2)].symbol(), "┤");
}

#[test]
fn demo_exact_width_96_center_sqr_trailing_cap() {
    let nav = demo_nav(0, TabBarEnd::Sqr, TabBarAlign::Center);
    let width = demo_total_width(&nav) + 1;
    let area = Rect::new(0, 0, width, 3);
    let mut buf = Buffer::empty(area);
    draw(nav, area, &mut buf);
    let baseline_y = 2;
    assert_eq!(
        buf[(0, baseline_y)].symbol(),
        "│",
        "first tab selected: leading junction not margin cap"
    );
    assert_eq!(buf[(width - 2, baseline_y)].symbol(), "┴");
    assert_eq!(buf[(width - 1, baseline_y)].symbol(), "┐");
}

#[test]
fn demo_exact_width_96_center_sqr_first_tab_inactive() {
    let nav = demo_nav(1, TabBarEnd::Sqr, TabBarAlign::Center);
    let width = demo_total_width(&nav) + 1;
    let area = Rect::new(0, 0, width, 3);
    let mut buf = Buffer::empty(area);
    draw(nav, area, &mut buf);
    assert_eq!(buf[(0, 2)].symbol(), "├");
}

#[test]
fn demo_exact_width_96_center_rnd_first_tab_active() {
    let nav = demo_nav(0, TabBarEnd::Rnd, TabBarAlign::Center);
    let width = demo_total_width(&nav) + 1;
    let area = Rect::new(0, 0, width, 3);
    let mut buf = Buffer::empty(area);
    draw(nav, area, &mut buf);
    assert_eq!(buf[(0, 2)].symbol(), "│");
}

#[test]
fn demo_exact_width_96_center_bottom_first_tab_inactive() {
    let nav = demo_nav(1, TabBarEnd::Sqr, TabBarAlign::Center)
        .horizontal_position(HorizontalPosition::Bottom);
    let width = demo_total_width(&nav) + 1;
    let area = Rect::new(0, 0, width, 5);
    let mut buf = Buffer::empty(area);
    draw(nav, area, &mut buf);
    let baseline_y = area.bottom() - 3;
    assert_eq!(buf[(0, baseline_y)].symbol(), "├");
}

#[test]
fn demo_vertical_center_uneven_slack_first_tab_junctions() {
    let labels = demo_vertical_labels();
    let tab_refs: Vec<&str> = labels.iter().map(String::as_str).collect();
    let nav = TabNav::new(&tab_refs, 0)
        .orientation(TabOrientation::Vertical)
        .tab_bar_end(TabBarEnd::Sqr)
        .tab_bar_align(TabBarAlign::Center);
    let width = nav.vertical_rail_width();
    let edge_height = demo_vertical_total_height(&nav) + 1;
    for (height, selected, expected) in [(edge_height, 0, "─"), (edge_height, 1, "┬")] {
        let nav = TabNav::new(&tab_refs, selected)
            .orientation(TabOrientation::Vertical)
            .tab_bar_end(TabBarEnd::Sqr)
            .tab_bar_align(TabBarAlign::Center);
        let area = Rect::new(0, 0, width, height);
        let viewport = compute_viewport(&nav, area, 0);
        let first_y = viewport
            .entries
            .first()
            .map(|entry| entry.offset)
            .unwrap_or(0);
        assert_eq!(first_y, area.y, "edge case tabs flush to content top");
        let mut buf = Buffer::empty(area);
        draw(nav, area, &mut buf);
        let baseline_x = width - 1;
        assert_eq!(
            buf[(baseline_x, first_y)].symbol(),
            expected,
            "height {height}, selected {selected}"
        );
    }
}

#[test]
fn demo_vertical_center_uneven_slack_right_position() {
    let labels = demo_vertical_labels();
    let tab_refs: Vec<&str> = labels.iter().map(String::as_str).collect();
    let nav = TabNav::new(&tab_refs, 0)
        .orientation(TabOrientation::Vertical)
        .vertical_position(VerticalPosition::Right)
        .tab_bar_end(TabBarEnd::Rnd)
        .tab_bar_align(TabBarAlign::Center);
    let width = nav.vertical_rail_width();
    let height = demo_vertical_total_height(&nav) + 1;
    let area = Rect::new(0, 0, width + 4, height);
    let viewport = compute_viewport(&nav, area, 0);
    let first_y = viewport
        .entries
        .first()
        .map(|entry| entry.offset)
        .unwrap_or(0);
    assert_eq!(first_y, area.y);
    let mut buf = Buffer::empty(area);
    draw(nav, area, &mut buf);
    let baseline_x = area.right() - width;
    assert_eq!(buf[(baseline_x, first_y)].symbol(), "─");
}

#[test]
fn demo_exact_width_97_center_sqr_bilateral_slack() {
    let nav = demo_nav(1, TabBarEnd::Sqr, TabBarAlign::Center);
    let width = demo_total_width(&nav) + 2;
    let area = Rect::new(0, 0, width, 3);
    let viewport = compute_viewport(&nav, area, 0);
    let first_x = viewport
        .entries
        .first()
        .map(|entry| entry.offset)
        .unwrap_or(0);
    let mut buf = Buffer::empty(area);
    draw(nav, area, &mut buf);
    assert_eq!(buf[(0, 2)].symbol(), "┌", "margin cap in leading slack");
    assert_eq!(
        buf[(first_x, 2)].symbol(),
        "┴",
        "inactive first tab keeps tee junction when centered with bilateral slack"
    );
    assert_eq!(buf[(width - 1, 2)].symbol(), "┐");
}

#[test]
fn horizontal_center_bilateral_slack_inactive_first_keeps_tee() {
    let area = Rect::new(0, 0, 60, 3);
    let nav = TabNav::new(&["A", "B"], 1)
        .tab_bar_end(TabBarEnd::Sqr)
        .tab_bar_align(TabBarAlign::Center);
    let first_x = compute_viewport(&nav, area, 0)
        .entries
        .first()
        .map(|entry| entry.offset)
        .unwrap_or(0);
    let mut buf = Buffer::empty(area);
    draw(nav, area, &mut buf);
    assert_eq!(buf[(0, 2)].symbol(), "┌");
    assert_eq!(buf[(first_x, 2)].symbol(), "┴");
    assert_eq!(buf[(59, 2)].symbol(), "┐");
}

#[test]
fn horizontal_center_bilateral_slack_active_first_keeps_tab_corner() {
    let area = Rect::new(0, 0, 120, 3);
    let nav = TabNav::new(&["A", "B"], 0)
        .tab_bar_end(TabBarEnd::Rnd)
        .tab_bar_align(TabBarAlign::Center);
    let first_x = compute_viewport(&nav, area, 0)
        .entries
        .first()
        .map(|entry| entry.offset)
        .unwrap_or(0);
    let mut buf = Buffer::empty(area);
    draw(nav, area, &mut buf);
    assert_eq!(buf[(0, 2)].symbol(), "╭");
    assert_eq!(
        buf[(first_x, 2)].symbol(),
        "╯",
        "active first tab keeps open-corner junction, not TabBarEnd leading cap"
    );
}

#[test]
fn vertical_center_bilateral_slack_first_tab_keeps_tab_junctions() {
    let labels = demo_vertical_labels();
    let tab_refs: Vec<&str> = labels.iter().map(String::as_str).collect();
    let area = Rect::new(0, 0, 8, 40);
    let baseline_x = TabNav::new(&tab_refs, 0)
        .orientation(TabOrientation::Vertical)
        .vertical_rail_width()
        - 1;
    let first_y = compute_viewport(
        &TabNav::new(&tab_refs, 0)
            .orientation(TabOrientation::Vertical)
            .tab_bar_align(TabBarAlign::Center),
        area,
        0,
    )
    .entries
    .first()
    .map(|entry| entry.offset)
    .unwrap_or(0);

    let mut active = Buffer::empty(area);
    draw(
        TabNav::new(&tab_refs, 0)
            .orientation(TabOrientation::Vertical)
            .tab_bar_end(TabBarEnd::Rnd)
            .tab_bar_align(TabBarAlign::Center),
        area,
        &mut active,
    );
    assert_eq!(active[(baseline_x, 0)].symbol(), "╭");
    assert_eq!(
        active[(baseline_x, first_y)].symbol(),
        "╯",
        "active first tab keeps open-corner junction"
    );

    let mut inactive = Buffer::empty(area);
    draw(
        TabNav::new(&tab_refs, 1)
            .orientation(TabOrientation::Vertical)
            .tab_bar_end(TabBarEnd::Rnd)
            .tab_bar_align(TabBarAlign::Center),
        area,
        &mut inactive,
    );
    assert_eq!(
        inactive[(baseline_x, first_y)].symbol(),
        "┤",
        "inactive first tab keeps rail junction, not TabBarEnd tee"
    );
}

#[test]
fn demo_exact_width_95_start_rnd_end_caps() {
    let nav = demo_nav(0, TabBarEnd::Rnd, TabBarAlign::Start);
    let width = demo_total_width(&nav);
    let area = Rect::new(0, 0, width, 3);
    let mut buf = Buffer::empty(area);
    draw(nav, area, &mut buf);
    let baseline_y = 2;
    assert_eq!(buf[(0, baseline_y)].symbol(), "│");
    assert_eq!(buf[(width - 1, baseline_y)].symbol(), "┤");
}

#[test]
fn truncate_shows_overflow_affordance() {
    let area = Rect::new(0, 0, 20, 3);
    let mut buf = Buffer::empty(area);
    draw(
        TabNav::new(&["Long", "Overflow"], 0).overflow(OverflowPolicy::Truncate),
        area,
        &mut buf,
    );
    let bot_line = line_str(&buf, 2);
    assert!(bot_line.contains('…'));
}

#[test]
fn scroll_start_align_keeps_first_visible_tab_at_flow_start() {
    let tabs = ["One", "Two", "Three", "Four", "Five"];
    let nav = TabNav::new(&tabs, 2)
        .tab_bar_align(TabBarAlign::Start)
        .overflow(OverflowPolicy::Scroll);
    let narrow = Rect::new(0, 0, 28, 3);
    let mut state = TabNavState::new(2);
    state.ensure_selected_visible(&nav, narrow);
    let viewport = compute_viewport(&nav, narrow, state.scroll_offset);
    let flow_start = narrow.x;
    assert_eq!(viewport.entries.first().unwrap().offset, flow_start);
}

#[test]
fn scroll_expands_restores_earlier_tabs() {
    let tabs = ["A", "B", "C", "D", "E"];
    let nav = TabNav::new(&tabs, 4).overflow(OverflowPolicy::Scroll);
    let narrow = Rect::new(0, 0, 24, 3);
    let wide = Rect::new(0, 0, 80, 3);
    let mut state = TabNavState::new(4);
    state.ensure_selected_visible(&nav, narrow);
    assert!(state.scroll_offset > 0);
    state.ensure_selected_visible(&nav, wide);
    assert_eq!(state.scroll_offset, 0);
    let viewport = compute_viewport(&nav, wide, state.scroll_offset);
    assert!(viewport.entries.iter().any(|entry| entry.index == 0));
}

#[test]
fn scroll_shows_in_tab_overflow_markers_not_on_baseline() {
    let tabs = ["One", "Two", "Three", "Four"];
    let nav = TabNav::new(&tabs, 3)
        .overflow(OverflowPolicy::Scroll)
        .scroll_offset(1);
    let area = Rect::new(0, 0, 28, 3);
    let mut buf = Buffer::empty(area);
    draw(nav, area, &mut buf);
    let label_y = 1;
    let baseline_y = 2;
    let bot_line = line_str(&buf, baseline_y);
    assert!(!bot_line.contains('‹'));
    assert!(!bot_line.contains('›'));
    assert!(!bot_line.contains('…'));
    assert!(line_str(&buf, label_y).contains('⯇'));
    assert!(line_str(&buf, label_y).contains('⯈'));
}

#[test]
fn stateful_widget_uses_state_selection() {
    let area = Rect::new(0, 0, 30, 3);
    let mut buf = Buffer::empty(area);
    let mut state = TabNavState::new(1);
    ratatui_core::widgets::StatefulWidget::render(
        TabNav::new(&["A", "B"], 0),
        area,
        &mut buf,
        &mut state,
    );
    let mid_line = line_str(&buf, 1);
    assert!(mid_line.contains('B'));
}

#[test]
fn ensure_selected_visible_scrolls_window() {
    let tabs = ["A", "B", "C", "D", "E"];
    let nav = TabNav::new(&tabs, 0).overflow(OverflowPolicy::Scroll);
    let area = Rect::new(0, 0, 24, 3);
    let mut state = TabNavState::new(4);
    state.ensure_selected_visible(&nav, area);
    assert!(state.scroll_offset > 0);
    let viewport = compute_viewport(&nav, area, state.scroll_offset);
    assert!(viewport.entries.iter().any(|entry| entry.index == 4));
}

fn line_str(buf: &Buffer, y: u16) -> String {
    let area = buf.area();
    (area.x..area.right())
        .map(|x| buf[(x, y)].symbol().to_string())
        .collect()
}

fn col_str(buf: &Buffer, x: u16) -> String {
    let area = buf.area();
    (area.y..area.bottom())
        .map(|y| buf[(x, y)].symbol().to_string())
        .collect()
}

#[test]
fn mouse_wheel_switches_tab_when_hovering_strip() {
    let nav = TabNav::new(&["A", "B", "C"], 0);
    let area = Rect::new(0, 0, 30, 3);
    let mut state = TabNavState::new(0);
    assert!(state.handle_mouse_wheel(&nav, area, 5, 1, TabWheelDirection::Down,));
    assert_eq!(state.selected, 1);
}

#[test]
fn mouse_wheel_ignored_when_disabled() {
    let nav = TabNav::new(&["A", "B"], 0).mouse_wheel(false);
    let area = Rect::new(0, 0, 20, 3);
    let mut state = TabNavState::new(0);
    assert!(!state.handle_mouse_wheel(&nav, area, 5, 1, TabWheelDirection::Down,));
    assert_eq!(state.selected, 0);
}

#[test]
fn mouse_wheel_ignored_outside_strip() {
    let nav = TabNav::new(&["A", "B"], 0);
    let area = Rect::new(0, 0, 20, 3);
    let mut state = TabNavState::new(0);
    assert!(!state.handle_mouse_wheel(&nav, area, 25, 1, TabWheelDirection::Down,));
    assert_eq!(state.selected, 0);
}

#[test]
fn wheel_direction_from_axes_prefers_horizontal_on_horizontal_strip() {
    use crate::config::TabOrientation;
    assert_eq!(
        TabWheelDirection::from_axes(
            Some(TabWheelDirection::Down),
            Some(TabWheelDirection::Up),
            TabOrientation::Horizontal,
        ),
        Some(TabWheelDirection::Up),
    );
    assert_eq!(
        TabWheelDirection::from_axes(
            Some(TabWheelDirection::Down),
            None,
            TabOrientation::Horizontal,
        ),
        Some(TabWheelDirection::Down),
    );
}

#[test]
fn wheel_direction_from_axes_prefers_vertical_on_vertical_strip() {
    use crate::config::TabOrientation;
    assert_eq!(
        TabWheelDirection::from_axes(
            Some(TabWheelDirection::Up),
            Some(TabWheelDirection::Down),
            TabOrientation::Vertical,
        ),
        Some(TabWheelDirection::Up),
    );
}

#[test]
fn wheel_hover_accepts_pointer_over_visible_tab() {
    let nav = TabNav::new(&["Alpha", "Beta", "Gamma"], 0);
    let strip = Rect::new(0, 0, 12, 3);
    // Narrow render area; pointer over second tab rect still counts.
    assert!(nav.wheel_hover(strip, 0, 7, 1));
    assert!(!nav.wheel_hover(strip, 0, 50, 1));
}

#[test]
fn mouse_click_selects_tab_under_pointer() {
    let nav = TabNav::new(&["A", "B", "C"], 0);
    let area = Rect::new(0, 0, 30, 3);
    let rects = nav.tab_rects(area);
    let col = rects[1].x + 1;
    let row = rects[1].y + 1;
    let mut state = TabNavState::new(0);
    assert!(state.handle_mouse_click(&nav, area, col, row));
    assert_eq!(state.selected, 1);
}

#[test]
fn mouse_click_ignored_when_disabled() {
    let nav = TabNav::new(&["A", "B"], 0).mouse_click(false);
    let area = Rect::new(0, 0, 20, 3);
    let rects = nav.tab_rects(area);
    let mut state = TabNavState::new(0);
    assert!(!state.handle_mouse_click(&nav, area, rects[0].x + 1, 1));
    assert_eq!(state.selected, 0);
}

#[test]
fn vertical_tab_index_at_matches_each_visible_tab() {
    let labels: Vec<String> = ["One", "Two", "Three", "Four"]
        .iter()
        .map(|name| vertical_label(name))
        .collect();
    let tabs: Vec<&str> = labels.iter().map(String::as_str).collect();
    let nav = TabNav::new(&tabs, 0)
        .orientation(TabOrientation::Vertical)
        .reorder_policy(TabReorderPolicy::NonePinned)
        .mouse_reorder(true);
    let width = nav.vertical_rail_width();
    let total_height: u16 = (0..tabs.len())
        .map(|i| nav.auto_tab_height(i).unwrap())
        .sum();
    let area = Rect::new(5, 10, width, total_height);
    let rects = nav.tab_rects(area);
    assert_eq!(rects.len(), tabs.len());

    for (index, rect) in rects.iter().enumerate() {
        let row = rect.y + rect.height / 2;
        let col = rect.x + 1;
        assert_eq!(
            nav.tab_index_at(area, 0, col, row),
            Some(index),
            "tab {index} at ({col}, {row})"
        );
    }
}

#[test]
fn vertical_tab_index_at_respects_scroll_offset() {
    let labels: Vec<String> = (0..6).map(|i| vertical_label(&format!("T{i}"))).collect();
    let tabs: Vec<&str> = labels.iter().map(String::as_str).collect();
    let nav = TabNav::new(&tabs, 0)
        .orientation(TabOrientation::Vertical)
        .overflow(OverflowPolicy::Scroll);
    let width = nav.vertical_rail_width();
    let area = Rect::new(0, 0, width, 12);
    let scroll_offset = 2usize;
    let rects = nav.tab_rects_with_scroll(area, scroll_offset);
    assert!(!rects.is_empty());
    let first = rects[0];
    let row = first.y + first.height / 2;
    assert_eq!(
        nav.tab_index_at(area, scroll_offset, first.x + 1, row),
        Some(scroll_offset)
    );
}

#[test]
fn tab_index_at_returns_none_outside_tabs() {
    let nav = TabNav::new(&["A", "B"], 0);
    let area = Rect::new(0, 0, 20, 3);
    assert!(nav.tab_index_at(area, 0, 50, 1).is_none());
}

#[test]
fn mouse_reorder_moves_unpinned_tab() {
    let mut labels = vec!["A", "B", "C", "D"];
    let pinned = [true, false, true, false];
    let nav = TabNav::new(&labels, 0)
        .reorder_policy(TabReorderPolicy::SomePinned)
        .tab_pinned(&pinned)
        .mouse_reorder(true);
    let area = Rect::new(0, 0, 48, 3);
    let rects = nav.tab_rects(area);
    let mut state = TabNavState::new(0);
    let from_col = rects[1].x + 1;
    let to_col = rects[3].x + 1;
    assert!(state.handle_mouse_reorder_press(&nav, area, from_col, 1));
    state.handle_mouse_reorder_drag(&nav, area, to_col, 1);
    let reorder = state
        .handle_mouse_reorder_release(&nav)
        .expect("expected reorder");
    assert_eq!(reorder.from, 1);
    assert_eq!(reorder.to, 3);
    assert!(try_reorder(
        &mut labels,
        reorder.from,
        reorder.to,
        TabReorderPolicy::SomePinned,
        Some(&pinned),
    ));
    assert_eq!(labels, ["A", "D", "C", "B"]);
}

#[test]
fn reorder_drag_highlights_source_tab_with_indexed_46() {
    let nav = TabNav::new(&["A", "B", "C"], 0)
        .reorder_policy(TabReorderPolicy::NonePinned)
        .mouse_reorder(true);
    let area = Rect::new(0, 0, 30, 3);
    let rects = nav.tab_rects(area);
    let mut buf = Buffer::empty(area);
    let mut state = TabNavState::new(0);
    state.reorder_drag = Some(TabReorderDrag {
        source: 1,
        hover: 1,
        armed: true,
    });
    StatefulWidget::render(nav, area, &mut buf, &mut state);
    let tab = rects[1];
    let mut found = false;
    for y in tab.y..tab.bottom() {
        for x in tab.x..tab.right() {
            if buf[(x, y)].symbol() == "B" {
                assert_eq!(buf[(x, y)].fg, Color::Indexed(46));
                found = true;
                break;
            }
        }
        if found {
            break;
        }
    }
    assert!(found, "expected label B in dragged tab rect");
}

#[test]
fn reorder_press_without_armed_does_not_highlight() {
    let nav = TabNav::new(&["A", "B", "C"], 0)
        .reorder_policy(TabReorderPolicy::NonePinned)
        .mouse_reorder(true)
        .border_style(Style::new().fg(Color::White));
    let area = Rect::new(0, 0, 30, 3);
    let rects = nav.tab_rects(area);
    let mut buf = Buffer::empty(area);
    let mut state = TabNavState::new(0);
    state.reorder_drag = Some(TabReorderDrag {
        source: 1,
        hover: 1,
        armed: false,
    });
    StatefulWidget::render(nav, area, &mut buf, &mut state);
    let tab = rects[1];
    let corner_fg = buf[(tab.x, tab.y)].fg;
    assert_eq!(
        corner_fg,
        Color::White,
        "unarmed drag must not apply indexed-46 border"
    );
    assert_ne!(corner_fg, Color::Indexed(46));
}

#[test]
fn selection_flash_highlights_border_not_label() {
    let nav = TabNav::new(&["A", "B"], 0)
        .border_style(Style::new().fg(Color::White))
        .style(Style::new().fg(Color::White));
    let area = Rect::new(0, 0, 20, 3);
    let rects = nav.tab_rects(area);
    let mut buf = Buffer::empty(area);
    let mut state = TabNavState::new(0);
    state.flash_selection(1);
    StatefulWidget::render(nav, area, &mut buf, &mut state);
    let tab = rects[1];
    let mut border_46 = false;
    let mut label_46 = false;
    for y in tab.y..tab.bottom() {
        for x in tab.x..tab.right() {
            let cell = &buf[(x, y)];
            if cell.fg != Color::Indexed(46) {
                continue;
            }
            if cell.symbol() == "B" {
                label_46 = true;
            } else {
                border_46 = true;
            }
        }
    }
    assert!(border_46, "expected border fg 46 during flash");
    assert!(!label_46, "label must not use flash color");
}

#[test]
fn vertical_tab_index_at_matches_stacked_label_rects() {
    let overview = vertical_label("Overview");
    let network = vertical_label("Network");
    let tabs = [overview.as_str(), network.as_str()];
    let nav = TabNav::new(&tabs, 0).orientation(TabOrientation::Vertical);
    let area = Rect::new(0, 0, 8, 40);
    let rects = nav.tab_rects(area);
    assert_eq!(rects.len(), 2);
    for (expected_index, rect) in rects.into_iter().enumerate() {
        let row = rect.y + rect.height / 2;
        let col = rect.x + rect.width / 2;
        assert_eq!(
            nav.tab_index_at(area, 0, col, row),
            Some(expected_index),
            "pointer in tab {expected_index} rect should hit that tab"
        );
    }
}

#[test]
fn vertical_tab_index_at_differs_when_label_geometry_differs() {
    let stacked = vertical_label("Network");
    let flat = "Network";
    let stacked_tabs = [stacked.as_str()];
    let flat_tabs = [flat];
    let stacked_nav = TabNav::new(&stacked_tabs, 0).orientation(TabOrientation::Vertical);
    let flat_nav = TabNav::new(&flat_tabs, 0).orientation(TabOrientation::Vertical);
    let area = Rect::new(0, 0, 8, 40);
    let stacked_rect = stacked_nav.tab_rects(area)[0];
    let row = stacked_rect.y + stacked_rect.height.saturating_sub(1);
    let col = stacked_rect.x + 1;
    assert_eq!(stacked_nav.tab_index_at(area, 0, col, row), Some(0));
    assert_eq!(
        flat_nav.tab_index_at(area, 0, col, row),
        None,
        "same pointer must not hit a shorter flat label tab"
    );
}
