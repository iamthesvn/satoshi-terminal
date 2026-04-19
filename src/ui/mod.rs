// ui/mod.rs — Satoshi's Terminal render dispatcher.

pub mod chapter;
pub mod menu;
pub mod transition;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::{App, AppState};

const ACCENT: Color = Color::Rgb(255, 120, 40);
const BG: Color = Color::Rgb(10, 10, 18);

pub fn draw(frame: &mut Frame, app: &App) {
    match &app.state {
        AppState::Menu { selected } => {
            menu::draw_menu(frame, frame.area(), *selected, *app.anim.border_breathe);
        }
        AppState::DifficultySelect { selected } => {
            menu::draw_difficulty_select(frame, frame.area(), *selected, app.save.difficulty);
        }
        AppState::VolumeSelect { selected } => {
            draw_volume_select(frame, app, *selected);
        }
        AppState::ChapterIntro { vol_idx, ch_idx } => {
            draw_chapter_intro(frame, app, *vol_idx, *ch_idx, app.anim.intro_typewriter.as_str());
        }
        AppState::Playing { vol_idx, ch_idx } => {
            if let Some(ch) = app.current_chapter(*vol_idx, *ch_idx) {
                chapter::draw_chapter(
                    frame,
                    vol_idx + 1,
                    ch_idx + 1,
                    ch,
                    &app.chapter_state,
                    &app.anim,
                    app.total_xp(),
                    app.save.difficulty,
                );
            }
        }
        AppState::ChapterComplete {
            vol_idx,
            ch_idx,
            earned_xp,
            anim_tick,
        } => {
            draw_chapter_complete(
                frame, app, *vol_idx, *ch_idx, *earned_xp, *anim_tick, *app.anim.success_breathe,
            );
        }
        AppState::Transition {
            next_vol,
            next_ch,
            frame: anim_frame,
        } => {
            transition::draw_transition(
                frame,
                *next_vol,
                *anim_frame,
                *app.anim.transition_shimmer,
            );
            let _ = next_ch;
        }
        AppState::VolumeComplete { vol_idx } => {
            draw_volume_complete(frame, app, *vol_idx, *app.anim.border_breathe);
        }
        AppState::GameComplete => {
            draw_game_complete(frame, app, *app.anim.border_breathe);
        }
        AppState::Quit => {}
    }
}

pub fn draw_resize_warning(frame: &mut Frame) {
    let area = frame.area();
    let p = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled(
            "  ⚠  Terminal too small!",
            Style::default().fg(Color::Yellow),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "  Satoshi's Terminal needs at least 80 × 24.",
            Style::default().fg(Color::Rgb(180, 180, 180)),
        )),
        Line::from(Span::styled(
            "  Please resize your terminal window.",
            Style::default().fg(Color::DarkGray),
        )),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow)),
    );
    frame.render_widget(p, area);
}

// ── Volume select ─────────────────────────────────────────────────────────────

fn draw_volume_select(frame: &mut Frame, app: &App, selected: usize) {
    let area = frame.area();
    let mut lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "  Select a Volume",
            Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
    ];
    for (i, vol) in app.volumes.iter().enumerate() {
        let cursor = if i == selected { "▶ " } else { "  " };
        let style = if i == selected {
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Rgb(140, 140, 140))
        };
        lines.push(Line::from(Span::styled(
            format!(
                "  {}Vol {} — {}   {}",
                cursor, vol.id, vol.title, vol.tagline
            ),
            style,
        )));
        lines.push(Line::from(""));
    }
    lines.push(Line::from(Span::styled(
        "  [↑↓] Navigate  [Enter] Select  [Esc] Back",
        Style::default().fg(Color::DarkGray),
    )));

    let p = Paragraph::new(lines).style(Style::default().bg(BG)).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(ACCENT))
            .title(Span::styled(
                " Satoshi's Terminal — Choose Your Volume ",
                Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
            )),
    );
    frame.render_widget(p, area);
}

// ── Chapter intro ─────────────────────────────────────────────────────────────

fn draw_chapter_intro(
    frame: &mut Frame,
    app: &App,
    vol_idx: usize,
    ch_idx: usize,
    intro_text: &str,
) {
    let area = frame.area();
    let vol = match app.current_volume(vol_idx) {
        Some(v) => v,
        None => return,
    };
    let ch = match app.current_chapter(vol_idx, ch_idx) {
        Some(c) => c,
        None => return,
    };

    let panels = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(34), Constraint::Min(40)])
        .split(area);

    // Left: ASCII art
    let art_lines: Vec<Line> = ch
        .scene_art
        .iter()
        .map(|l| {
            Line::from(Span::styled(
                *l,
                Style::default().fg(Color::Rgb(80, 180, 100)),
            ))
        })
        .collect();
    let art_widget = Paragraph::new(art_lines)
        .style(Style::default().bg(BG))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(40, 80, 40)))
                .title(Span::styled(
                    " Scene ",
                    Style::default().fg(Color::Rgb(80, 180, 100)),
                )),
        );
    frame.render_widget(art_widget, panels[0]);

    // Right: volume header + chapter title + NPC dialogue (typewriter)
    let mut lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            format!("  Vol {} — {}", vol.id, vol.title),
            Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            format!("  {}", vol.tagline),
            Style::default()
                .fg(Color::Rgb(140, 140, 140))
                .add_modifier(Modifier::ITALIC),
        )),
        Line::from(""),
        Line::from(Span::styled(
            format!("  Chapter {} — {}", ch_idx + 1, ch.title),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            format!("  {} says:", ch.npc_name),
            Style::default()
                .fg(Color::Rgb(255, 200, 80))
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
    ];

    // Typewriter-revealed dialogue
    let revealed_lines: Vec<&str> = intro_text.split('\n').collect();
    for line in revealed_lines {
        lines.push(Line::from(vec![
            Span::styled("  ╎ ", Style::default().fg(Color::Rgb(80, 80, 100))),
            Span::styled(
                line,
                Style::default()
                    .fg(Color::Rgb(200, 200, 200))
                    .add_modifier(Modifier::ITALIC),
            ),
        ]));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled("  ❯ Press ", Style::default().fg(Color::Rgb(160, 160, 160))),
        Span::styled(
            "[Enter]",
            Style::default()
                .fg(Color::Rgb(255, 255, 100))
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            " to begin   ",
            Style::default().fg(Color::Rgb(160, 160, 160)),
        ),
        Span::styled("[Esc]", Style::default().fg(Color::DarkGray)),
        Span::styled(" Menu", Style::default().fg(Color::DarkGray)),
    ]));

    let right = Paragraph::new(lines).style(Style::default().bg(BG)).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(50, 50, 80)))
            .title(Span::styled(
                " Briefing ",
                Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
            )),
    );
    frame.render_widget(right, panels[1]);
}

// ── Chapter complete ──────────────────────────────────────────────────────────

fn draw_chapter_complete(
    frame: &mut Frame,
    app: &App,
    vol_idx: usize,
    ch_idx: usize,
    _earned_xp: u32,
    anim_tick: usize,
    border_breathe: Color,
) {
    let area = frame.area();
    let ch = match app.current_chapter(vol_idx, ch_idx) {
        Some(c) => c,
        None => return,
    };

    let border_color = border_breathe;

    // Reveal success message character by character
    let msg_chars: usize = (anim_tick * 4).min(ch.success_message.len());
    let revealed_msg: String = ch.success_message.chars().take(msg_chars).collect();

    let animated_xp = *app.anim.xp_rise;
    let xp_bar_filled = ((animated_xp as usize * 20) / (ch.xp as usize).max(1)).min(20);
    let xp_bar = "▓".repeat(xp_bar_filled) + &"░".repeat(20 - xp_bar_filled);

    let mut lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "  ✓  Block Mined!",
            Style::default()
                .fg(Color::Rgb(60, 220, 100))
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            format!("  {}", revealed_msg),
            Style::default()
                .fg(Color::Rgb(200, 220, 200))
                .add_modifier(Modifier::ITALIC),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Sats earned  ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("+{animated_xp}"),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(Span::styled(
            format!("  [{xp_bar}]"),
            Style::default().fg(Color::Rgb(60, 180, 80)),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Total Sats ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{}", app.total_xp()),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("  Title      ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                app.rank(),
                Style::default()
                    .fg(Color::Rgb(255, 215, 0))
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "  [Enter] Next Block",
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::ITALIC),
        )),
    ];

    // Attempts penalty note
    if app.chapter_state.attempts > 1 {
        lines.push(Line::from(Span::styled(
            format!("  ({} attempts — Sats adjusted)", app.chapter_state.attempts),
            Style::default().fg(Color::Rgb(180, 140, 60)),
        )));
    }
    if app.chapter_state.hint_level > 0 {
        lines.push(Line::from(Span::styled(
            format!(
                "  ({} hints used — Sats adjusted)",
                app.chapter_state.hint_level
            ),
            Style::default().fg(Color::Rgb(100, 160, 200)),
        )));
    }

    let p = Paragraph::new(lines).style(Style::default().bg(BG)).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color))
            .title(Span::styled(
                " Chapter Complete ",
                Style::default()
                    .fg(border_color)
                    .add_modifier(Modifier::BOLD),
            )),
    );
    frame.render_widget(p, area);
}

// ── Volume complete ───────────────────────────────────────────────────────────

fn draw_volume_complete(frame: &mut Frame, app: &App, vol_idx: usize, border_breathe: Color) {
    let area = frame.area();
    let vol = match app.current_volume(vol_idx) {
        Some(v) => v,
        None => return,
    };

    let vol_xp: u32 = app
        .save
        .xp_per_chapter
        .get(vol_idx)
        .map(|v| v.iter().sum::<u32>())
        .unwrap_or(0);

    let next_vol = app.volumes.get(vol_idx + 1);

    let stars = "★★★★★";

    let mut lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            format!("  ✦  Volume {} Complete!", vol_idx + 1),
            Style::default()
                .fg(Color::Rgb(255, 215, 0))
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            format!("  \"{}\"", vol.title),
            Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            format!("  {}", vol.tagline),
            Style::default()
                .fg(Color::Rgb(140, 140, 140))
                .add_modifier(Modifier::ITALIC),
        )),
        Line::from(""),
        Line::from(Span::styled(
            format!("  {stars}"),
            Style::default().fg(Color::Rgb(255, 215, 0)),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Vol Sats   ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{vol_xp}"),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("  Total Sats ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{}", app.total_xp()),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("  Title      ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                app.rank(),
                Style::default()
                    .fg(Color::Rgb(255, 215, 0))
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
    ];

    if let Some(next) = next_vol {
        lines.push(Line::from(Span::styled(
            format!("  Up next: Vol {} — \"{}\"", next.id, next.title),
            Style::default().fg(Color::Rgb(180, 220, 180)),
        )));
        lines.push(Line::from(Span::styled(
            format!("  {}", next.tagline),
            Style::default()
                .fg(Color::Rgb(120, 140, 120))
                .add_modifier(Modifier::ITALIC),
        )));
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "  [Enter] Begin next volume   [Esc] Main menu",
            Style::default().fg(Color::DarkGray),
        )));
    } else {
        lines.push(Line::from(Span::styled(
            "  [Enter] Final chapter →",
            Style::default().fg(Color::DarkGray),
        )));
    }

    let p = Paragraph::new(lines).style(Style::default().bg(BG)).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_breathe))
            .title(Span::styled(
                " Volume Complete ",
                Style::default()
                    .fg(Color::Rgb(255, 215, 0))
                    .add_modifier(Modifier::BOLD),
            ))
            .title_alignment(Alignment::Center),
    );
    frame.render_widget(p, area);
}

// ── Game complete ─────────────────────────────────────────────────────────────

const TROPHY: &[&str] = &[
    r"      ___________      ",
    r"     '._==_==_=_.'     ",
    r"     .-\:      /-.     ",
    r"    | (|:.     |) |    ",
    r"     '-|:.     |-'     ",
    r"       \::.    /       ",
    r"        '::. .'        ",
    r"          ) (          ",
    r"        _.' '._        ",
    r"       '-------'       ",
];

fn draw_game_complete(frame: &mut Frame, app: &App, border_breathe: Color) {
    let area = frame.area();

    let centered = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(62),
            Constraint::Fill(1),
        ])
        .split(area);
    let centered = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(32),
            Constraint::Fill(1),
        ])
        .split(centered[1]);
    let game_area = centered[1];

    let mut lines = vec![Line::from("")];
    for t in TROPHY {
        lines.push(Line::from(Span::styled(
            *t,
            Style::default()
                .fg(Color::Rgb(255, 215, 0))
                .add_modifier(Modifier::BOLD),
        )));
    }
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "     You survived NovaTech. Git mastered.",
        Style::default()
            .fg(Color::Rgb(60, 220, 100))
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(""));

    // Per-volume breakdown
    for (vi, vol) in app.volumes.iter().enumerate() {
        let vol_xp: u32 = app
            .save
            .xp_per_chapter
            .get(vi)
            .map(|v| v.iter().sum::<u32>())
            .unwrap_or(0);
        let filled = ((vol_xp as usize * 16) / 150).min(16);
        let bar = "▓".repeat(filled) + &"░".repeat(16 - filled);
        lines.push(Line::from(vec![
            Span::styled(format!("  Vol {} ", vi + 1), Style::default().fg(ACCENT)),
            Span::styled(
                format!("{:<28}", vol.title),
                Style::default().fg(Color::Rgb(140, 140, 140)),
            ),
            Span::styled(
                format!("[{bar}]"),
                Style::default().fg(Color::Rgb(80, 180, 80)),
            ),
            Span::styled(
                format!(" {vol_xp:>4} Sats"),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled("  Total Sats ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("{}", app.total_xp()),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
    ]));
    lines.push(Line::from(vec![
        Span::styled("  Final Title ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            app.rank(),
            Style::default()
                .fg(Color::Rgb(255, 215, 0))
                .add_modifier(Modifier::BOLD),
        ),
    ]));
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "  Stay humble. Stack sats. Verify everything.",
        Style::default()
            .fg(Color::Rgb(140, 180, 140))
            .add_modifier(Modifier::ITALIC),
    )));
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "  [Enter] Back to menu   [Q] Quit",
        Style::default().fg(Color::DarkGray),
    )));

    let bg = Paragraph::new("").style(Style::default().bg(BG));
    frame.render_widget(bg, area);

    let p = Paragraph::new(lines).style(Style::default().bg(BG)).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_breathe))
            .title(Span::styled(
                " Satoshi's Terminal Complete — Saylor, Bitcoin Principal ",
                Style::default()
                    .fg(Color::Rgb(255, 215, 0))
                    .add_modifier(Modifier::BOLD),
            ))
            .title_alignment(Alignment::Center),
    );
    frame.render_widget(p, game_area);
}
