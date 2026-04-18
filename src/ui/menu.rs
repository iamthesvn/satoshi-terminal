use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::Difficulty;

// "SATOSHI'S TERMINAL" in block letters — bitcoin orange
const LOGO_ART: &[&str] = &[
    r" ██████╗  █████╗  ████████╗  ██████╗   ██████╗ ██╗  ██╗  ██╗  ██╗  ██████╗",
    r"██╔════╝ ██╔══██╗ ╚══██╔══╝ ██╔═══██╗ ██╔════╝ ██║  ██║  ██║  ██║ ██╔════╝",
    r"╚█████╗  ███████║    ██║    ██║   ██║ ╚█████╗  ███████║  ██║  ╚═╝ ╚█████╗ ",
    r" ╚═══██║ ██╔══██║    ██║    ██║   ██║  ╚═══██║ ██╔══██║  ██║       ╚═══██║",
    r"██████╔╝ ██║  ██║    ██║    ╚██████╔╝ ██████╔╝ ██║  ██║  ██║      ██████╔╝",
    r"╚═════╝  ╚═╝  ╚═╝    ╚═╝     ╚═════╝  ╚═════╝  ╚═╝  ╚═╝  ╚═╝      ╚═════╝ ",
    r"████████╗ ████████╗ ██████╗   ███╗   ███╗ ██╗ ██╗   ██╗  █████╗  ██╗      ",
    r"╚══██╔══╝ ██╔═════╝ ██╔══██╗  ████╗ ████║ ██║ ███╗  ██║ ██╔══██╗ ██║      ",
    r"   ██║    █████╗    ██████╔╝  ██╔████╔██║ ██║ ██╔██╗██║ ███████║ ██║      ",
    r"   ██║    ██╔══╝    ██╔══██╗  ██║╚██╔╝██║ ██║ ██║╚████║ ██╔══██║ ██║      ",
    r"   ██║    ███████╗  ██║  ██║  ██║ ╚═╝ ██║ ██║ ██║ ╚███║ ██║  ██║ ███████╗ ",
    r"   ╚═╝    ╚══════╝  ╚═╝  ╚═╝  ╚═╝     ╚═╝ ╚═╝ ╚═╝  ╚══╝ ╚═╝  ╚═╝ ╚══════╝ ",
];

const MENU_ITEMS: &[&str] = &["     New Game", "     Continue", "     Quit"];

pub fn draw_menu(frame: &mut Frame, area: Rect, selected: usize, menu_glow: u8) {
    // Wide enough for QUEST logo (44 chars) + padding
    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(76),
            Constraint::Fill(1),
        ])
        .split(area);

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(28),
            Constraint::Fill(1),
        ])
        .split(horizontal[1]);

    let menu_area = vertical[1];

    let mut lines: Vec<Line> = vec![Line::from("")];

    // Terminal boot screen logo
    for logo_line in LOGO_ART {
        lines.push(Line::from(Span::styled(
            *logo_line,
            Style::default()
                .fg(Color::Rgb(247, 147, 26))
                .add_modifier(Modifier::BOLD),
        )));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(""));

    // Menu items
    for (i, item) in MENU_ITEMS.iter().enumerate() {
        let is_selected = i == selected;
        let display = if is_selected {
            format!("  ▶  {}", item.trim())
        } else {
            item.to_string()
        };
        let style = if is_selected {
            Style::default()
                .fg(Color::Rgb(247, 147, 26))
                .add_modifier(Modifier::BOLD)
                .bg(Color::Rgb(30, 18, 5))
        } else {
            Style::default().fg(Color::Rgb(180, 180, 180))
        };
        lines.push(Line::from(Span::styled(display, style)));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(""));

    lines.push(Line::from(Span::styled(
        "   Learn Bitcoin. One block at a time.",
        Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::ITALIC),
    )));

    lines.push(Line::from(""));

    lines.push(Line::from(Span::styled(
        "   [↑↓] Navigate  [Enter] Select  [Q] Quit",
        Style::default().fg(Color::Rgb(80, 80, 80)),
    )));

    let glow = menu_glow.max(180);
    let border_color = Color::Rgb(glow, glow.saturating_sub(40), glow.saturating_sub(60));
    let title_color = Color::Rgb(glow, glow.saturating_sub(30), glow.saturating_sub(50));

    let menu = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(border_color))
                .title(Span::styled(
                    " Satoshi's Terminal v1.0 ",
                    Style::default()
                        .fg(title_color)
                        .add_modifier(Modifier::BOLD),
                ))
                .title_alignment(Alignment::Center),
        )
        .alignment(Alignment::Left);

    let bg = Paragraph::new("").style(Style::default().bg(Color::Rgb(10, 10, 15)));
    frame.render_widget(bg, area);
    frame.render_widget(menu, menu_area);
}

// ── Difficulty selector screen ────────────────────────────────────────────────

const DIFFICULTY_ITEMS: &[(Difficulty, &str, &str)] = &[
    (
        Difficulty::Easy,
        "Easy",
        "Hints are free. No penalty for wrong attempts. Floor at 50% XP.",
    ),
    (
        Difficulty::Normal,
        "Normal",
        "-3 XP per hint. -2 XP per retry. Floor at 25% XP.",
    ),
    (
        Difficulty::Hard,
        "Hard",
        "No hints. -5 XP per retry. Floor at 10% XP.",
    ),
];

pub fn draw_difficulty_select(frame: &mut Frame, area: Rect, selected: usize, current: Difficulty) {
    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(76),
            Constraint::Fill(1),
        ])
        .split(area);

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(20),
            Constraint::Fill(1),
        ])
        .split(horizontal[1]);

    let menu_area = vertical[1];

    let mut lines: Vec<Line> = vec![Line::from("")];

    let title = Span::styled(
        "  SELECT DIFFICULTY",
        Style::default()
            .fg(Color::Rgb(247, 147, 26))
            .add_modifier(Modifier::BOLD),
    );
    lines.push(Line::from(title));
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "  Bitcoin difficulty retargets every 2016 blocks. Choose your target.",
        Style::default().fg(Color::DarkGray),
    )));
    lines.push(Line::from(""));

    for (i, (diff, label, desc)) in DIFFICULTY_ITEMS.iter().enumerate() {
        let is_selected = i == selected;
        let marker = if is_selected { "▶ " } else { "  " };
        let diff_line = format!("{}{}", marker, label);
        let style = if is_selected {
            Style::default()
                .fg(Color::Rgb(247, 147, 26))
                .add_modifier(Modifier::BOLD)
                .bg(Color::Rgb(30, 18, 5))
        } else if *diff == current {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::Rgb(180, 180, 180))
        };
        lines.push(Line::from(Span::styled(diff_line, style)));
        lines.push(Line::from(Span::styled(
            format!("     {}", desc),
            Style::default().fg(Color::Rgb(100, 100, 100)),
        )));
        lines.push(Line::from(""));
    }

    lines.push(Line::from(Span::styled(
        "   [↑↓] Navigate  [Enter] Select  [Esc] Back",
        Style::default().fg(Color::Rgb(80, 80, 80)),
    )));

    let menu = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(247, 147, 26)))
                .title(Span::styled(
                    " Difficulty Adjustment ",
                    Style::default()
                        .fg(Color::Rgb(247, 147, 26))
                        .add_modifier(Modifier::BOLD),
                ))
                .title_alignment(Alignment::Center),
        )
        .alignment(Alignment::Left);

    let bg = Paragraph::new("").style(Style::default().bg(Color::Rgb(10, 10, 15)));
    frame.render_widget(bg, area);
    frame.render_widget(menu, menu_area);
}
