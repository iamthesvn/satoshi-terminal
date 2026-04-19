//! chapter.rs — Satoshi's Terminal in-game chapter screen renderer
//!
//! Renders the main gameplay screen for a given chapter. The layout is:
//!
//!   ┌──────────────────────────────────────────────────────────────────┐
//!   │ HUD: Vol N · Ch N · title        Sats: NNN  [?] Hint [^C] Quit │
//!   ├────────────────────┬─────────────────────────────────────────────┤
//!   │  ASCII ART (left)  │  NPC DIALOGUE (right top)                  │
//!   │  8 lines tall      ├─────────────────────────────────────────────┤
//!   │                    │  TASK PROMPT (bright yellow box)            │
//!   ├────────────────────┴─────────────────────────────────────────────┤
//!   │  TERMINAL INPUT: $ _                                             │
//!   ├──────────────────────────────────────────────────────────────────┤
//!   │  HINT PANEL (collapsible — press ? to toggle)                   │
//!   └──────────────────────────────────────────────────────────────────┘
//!
//! Minimum terminal size: 80 × 24. Zero-size areas are handled gracefully.

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::anim::AnimState;
use crate::volumes::Chapter;

// ── Colour palette ────────────────────────────────────────────────────────────

/// Dark space background used throughout the screen.
const BG: Color = Color::Rgb(10, 10, 18);

/// Bitcoin Orange — borders, accents.
const ACCENT: Color = Color::Rgb(247, 147, 26);

/// Success / correct-answer green.
const GREEN: Color = Color::Rgb(60, 220, 100);

/// Error / wrong-answer red.
const RED: Color = Color::Rgb(220, 60, 60);

/// Mentor name highlight colour (bold orange-yellow).
const MENTOR_NAME_COLOR: Color = Color::Rgb(255, 200, 80);

/// Terminal-pane background — slightly darker than the global BG.
const TERM_BG: Color = Color::Rgb(8, 8, 14);

/// Task-prompt background.
const TASK_BG: Color = Color::Rgb(20, 20, 40);

/// Hint-panel background.
const HINT_BG: Color = Color::Rgb(15, 15, 30);

// ── ChapterState ─────────────────────────────────────────────────────────────

/// All mutable runtime state belonging to a single chapter play-through.
///
/// Caller is responsible for updating the fields on each tick / key event:
/// * Decrement `flash_wrong` / `flash_correct` by 1 each game tick.
/// * Append / remove from `input` on character / backspace events.
/// * Toggle `show_hint` when `?` is pressed.
/// * Advance `hint_level` (up to 3) when `H` is pressed.
#[derive(Clone, Default)]
pub struct ChapterState {
    /// The text the player has typed so far.
    pub input: String,
    /// Countdown (in ticks) for the red "wrong answer" flash on the terminal border.
    /// Set to e.g. 5 when the player submits an incorrect command.
    pub flash_wrong: u8,
    /// Countdown (in ticks) for the green "correct answer" flash.
    pub flash_correct: u8,
    /// Whether the hint panel is currently expanded.
    pub show_hint: bool,
    /// How many hints have been revealed: 0 = none, 1 = first only, ..., 3 = all.
    pub hint_level: usize,
    /// Whether the chapter has been completed successfully.
    pub completed: bool,
    /// Running count of submission attempts.
    pub attempts: u32,
    /// Timer remaining in game ticks (100 ms each). 0 = expired.
    pub time_remaining_ticks: u32,
}

impl ChapterState {
    /// Create a new state with the start timer set to right now.
    pub fn new() -> Self {
        Self::default()
    }

    /// Reset timer from a chapter's time limit (seconds → 100 ms ticks).
    pub fn set_timer(&mut self, secs: u32) {
        self.time_remaining_ticks = secs.saturating_mul(10);
    }
}

// ── Public draw entry-point ───────────────────────────────────────────────────

/// Render the full chapter screen into `frame`.
///
/// # Parameters
/// * `frame`   - The ratatui frame to draw into.
/// * `vol_num` - 1-indexed volume number shown in the HUD.
/// * `ch_num`  - 1-indexed chapter number shown in the HUD.
/// * `chapter` - Chapter data (mentor, ASCII art, dialogue, objective, hints, Sats).
/// * `state`   - Current mutable-but-read-only-for-rendering player state.
#[allow(clippy::too_many_arguments)]
pub fn draw_chapter(
    frame: &mut Frame,
    vol_num: usize,
    ch_num: usize,
    chapter: &Chapter,
    state: &ChapterState,
    anim: &AnimState,
    total_xp: u32,
    difficulty: crate::app::Difficulty,
) {
    let area = frame.area();

    // Guard: refuse to render into a comically tiny area that would panic.
    if area.width < 20 || area.height < 8 {
        return;
    }

    // ── Top-level vertical split ───────────────────────────────────────────
    // Rows (from top to bottom):
    //   [0] HUD bar           — always 1 row tall (inside a border = 3 total)
    //   [1] Art + NPC + Task  — fills remaining space
    //   [2] Terminal input    — 3 rows
    //   [3] Hint panel        — 0 (hidden) or dynamic height
    //
    // The hint panel height is computed first so Layout constraints are exact.
    // Animate hint panel open/close via anim.hint_openness (0.0 → 1.0)
    let openness = *anim.hint_openness;
    let show_hint_anim = openness > 0.001;

    let full_hint_height = {
        let revealed = state.hint_level.min(chapter.hints.len());
        (revealed.max(1) + 3) as u16
    };

    // Clamp hint height so it doesn't crowd out the core layout.
    let max_hint = area.height.saturating_sub(14);
    let hint_height = ((full_hint_height as f64 * openness) as u16).min(max_hint);

    let timer_height: u16 = 1; // thin timer bar above HUD
    let terminal_height: u16 = 3; // border + 1 content row + border
    let hud_height: u16 = 3; // border + 1 content row + border

    // Remaining height for the art/dialogue/task area.
    let mid_height = area
        .height
        .saturating_sub(timer_height + hud_height + terminal_height + hint_height);

    let vertical_constraints: Vec<Constraint> = if show_hint_anim {
        vec![
            Constraint::Length(timer_height),
            Constraint::Length(hud_height),
            Constraint::Length(mid_height),
            Constraint::Length(terminal_height),
            Constraint::Length(hint_height),
        ]
    } else {
        vec![
            Constraint::Length(timer_height),
            Constraint::Length(hud_height),
            Constraint::Min(mid_height),
            Constraint::Length(terminal_height),
        ]
    };

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vertical_constraints)
        .split(area);

    // Draw each zone. Index safety: rows will always have at least 4 entries
    // because we provided at least 4 constraints above.
    draw_timer_bar(frame, chapter, state, rows[0], difficulty);
    draw_hud(
        frame, vol_num, ch_num, chapter, state, rows[1], total_xp, difficulty,
    );
    draw_mid(frame, chapter, state, rows[2]);
    draw_terminal(frame, chapter, state, rows[3], anim);

    if show_hint_anim && let Some(hint_area) = rows.get(4) {
        draw_hints(frame, chapter, state, *hint_area, anim);
    }
}

// ── Timer bar ─────────────────────────────────────────────────────────────────

fn draw_timer_bar(
    frame: &mut Frame,
    chapter: &Chapter,
    state: &ChapterState,
    area: Rect,
    difficulty: crate::app::Difficulty,
) {
    if area.width == 0 || area.height == 0 {
        return;
    }
    let total_ticks = chapter.time_limit_secs.saturating_mul(10);
    if total_ticks == 0 {
        return;
    }

    let remaining = state.time_remaining_ticks.min(total_ticks);
    let ratio = remaining as f32 / total_ticks as f32;

    let bar_width = area.width as usize;
    let filled = (ratio * bar_width as f32).ceil() as usize;
    let empty = bar_width.saturating_sub(filled);

    let (fg, bg) = if difficulty == crate::app::Difficulty::Hard && ratio < 0.25 {
        (Color::Rgb(200, 40, 40), Color::Rgb(40, 10, 10))
    } else if ratio < 0.25 {
        (Color::Rgb(247, 147, 26), Color::Rgb(40, 20, 5))
    } else if ratio < 0.5 {
        (Color::Rgb(240, 200, 40), Color::Rgb(30, 25, 5))
    } else {
        (Color::Rgb(60, 210, 80), Color::Rgb(10, 30, 10))
    };

    let filled_ch = "▓";
    let empty_ch = "░";
    let bar_text = format!("{}{}", filled_ch.repeat(filled), empty_ch.repeat(empty));

    let line = Line::from(Span::styled(bar_text, Style::default().fg(fg).bg(bg)));
    frame.render_widget(Paragraph::new(line).style(Style::default().bg(bg)), area);
}

// ── HUD bar ───────────────────────────────────────────────────────────────────

/// Draw the top HUD bar:  `Vol N · Ch N · title   Sats: NNN  [?] Hint  [^C] Quit`
#[allow(clippy::too_many_arguments)]
fn draw_hud(
    frame: &mut Frame,
    vol_num: usize,
    ch_num: usize,
    chapter: &Chapter,
    _state: &ChapterState,
    area: Rect,
    total_xp: u32,
    difficulty: crate::app::Difficulty,
) {
    if area.width == 0 || area.height == 0 {
        return;
    }

    // Left portion: breadcrumb + difficulty badge
    let diff_badge = format!("[{}] ", difficulty.short());

    // Right portion: Sats + keybind hints
    let sats_str = format!(
        "Sats: {:>4} (+{})  [?] Hint  [Ctrl+C] Quit ",
        total_xp, chapter.xp
    );

    // Split HUD horizontally: left fills, right is fixed
    let right_len = sats_str.chars().count() as u16;
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(10), Constraint::Length(right_len)])
        .split(area);

    // Left: breadcrumb in accent colour
    let diff_color = match difficulty {
        crate::app::Difficulty::Easy => Color::Green,
        crate::app::Difficulty::Normal => Color::Yellow,
        crate::app::Difficulty::Hard => Color::Red,
    };
    let left_line = Line::from(vec![
        Span::styled(
            diff_badge,
            Style::default().fg(diff_color).add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("Vol {} . Ch {} . {}", vol_num, ch_num, chapter.title),
            Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
        ),
    ]);
    let left_para = Paragraph::new(left_line).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(ACCENT))
            .style(Style::default().bg(BG)),
    );
    frame.render_widget(left_para, cols[0]);

    // Right: Sats + keys in dark-gray
    let right_line = Line::from(vec![
        Span::styled("Sats: ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("{:>3}", chapter.xp),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            "  [?] Hint  [Ctrl+C] Quit ",
            Style::default().fg(Color::DarkGray),
        ),
    ]);
    let right_para = Paragraph::new(right_line).block(
        Block::default()
            .borders(Borders::TOP | Borders::RIGHT | Borders::BOTTOM)
            .border_style(Style::default().fg(ACCENT))
            .style(Style::default().bg(BG)),
    );
    frame.render_widget(right_para, cols[1]);
}

// ── Middle area: ASCII art (left) + NPC dialogue + Task (right) ───────────────

/// Draw the two-panel middle section.
fn draw_mid(frame: &mut Frame, chapter: &Chapter, state: &ChapterState, area: Rect) {
    if area.width < 4 || area.height < 2 {
        return;
    }

    // Horizontal split: art on the left (~28 cols), NPC+task on the right.
    let art_width: u16 = 22; // inner art content width
    let left_total = art_width + 2; // +2 for borders

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(left_total.min(area.width / 2)),
            Constraint::Min(30),
        ])
        .split(area);

    draw_ascii_art(frame, chapter, cols[0]);
    draw_mentor_and_objective(frame, chapter, state, cols[1]);
}

/// Render the ASCII art panel on the left.
fn draw_ascii_art(frame: &mut Frame, chapter: &Chapter, area: Rect) {
    if area.width == 0 || area.height == 0 {
        return;
    }

    // Build art lines, clipping to available inner height (area - 2 border rows).
    let inner_h = area.height.saturating_sub(2) as usize;
    let art_lines: Vec<Line> = chapter
        .scene_art
        .iter()
        .take(inner_h)
        .map(|row| {
            Line::from(Span::styled(
                *row,
                Style::default().fg(Color::Rgb(100, 200, 130)),
            ))
        })
        .collect();

    let art_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(40, 80, 50)))
        .title(Span::styled(
            " Block View ",
            Style::default().fg(Color::Rgb(100, 200, 130)),
        ))
        .style(Style::default().bg(BG));

    let art_para = Paragraph::new(art_lines).block(art_block);
    frame.render_widget(art_para, area);
}

/// Render the NPC dialogue (top-right) and task prompt (bottom-right) stacked
/// inside the right column of the middle area.
fn draw_mentor_and_objective(frame: &mut Frame, chapter: &Chapter, _state: &ChapterState, area: Rect) {
    if area.width < 4 || area.height < 4 {
        return;
    }

    // Decide how to split: task box needs at least 4 rows.
    // NPC dialogue gets remaining space.
    let task_height: u16 = 4.max(area.height / 3).min(area.height.saturating_sub(4));
    let npc_height = area.height.saturating_sub(task_height);

    let right_rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(npc_height),
            Constraint::Length(task_height),
        ])
        .split(area);

    draw_mentor_dialogue(frame, chapter, right_rows[0]);
    draw_objective(frame, chapter, right_rows[1]);
}

/// Render the NPC name and speech bubble dialogue lines.
fn draw_mentor_dialogue(frame: &mut Frame, chapter: &Chapter, area: Rect) {
    if area.width == 0 || area.height == 0 {
        return;
    }

    let inner_h = area.height.saturating_sub(2) as usize; // subtract border rows

    let mut lines: Vec<Line> = Vec::new();

    // NPC name header
    lines.push(Line::from(vec![
        Span::styled(" [[ ", Style::default().fg(Color::Rgb(80, 80, 100))),
        Span::styled(
            chapter.npc_name,
            Style::default()
                .fg(MENTOR_NAME_COLOR)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" ]]", Style::default().fg(Color::Rgb(80, 80, 100))),
    ]));

    // Speech bubble lines — italic light-gray
    let max_dialogue_lines = inner_h.saturating_sub(1); // reserve the name row
    for dl in chapter.npc_dialogue.iter().take(max_dialogue_lines) {
        lines.push(Line::from(vec![
            Span::styled(" | ", Style::default().fg(Color::Rgb(80, 80, 100))),
            Span::styled(
                *dl,
                Style::default()
                    .fg(Color::Rgb(200, 200, 210))
                    .add_modifier(Modifier::ITALIC),
            ),
        ]));
    }

    let mentor_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(60, 60, 90)))
        .title(Span::styled(
            " Mentor ",
            Style::default().fg(Color::Rgb(80, 80, 120)),
        ))
        .style(Style::default().bg(BG));

    let mentor_para = Paragraph::new(lines).block(mentor_block);
    frame.render_widget(mentor_para, area);
}

/// Render the bright-yellow-bordered task prompt box.
fn draw_objective(frame: &mut Frame, chapter: &Chapter, area: Rect) {
    if area.width == 0 || area.height == 0 {
        return;
    }

    let task_line = Line::from(vec![
        Span::styled(
            "> Objective: ",
            Style::default()
                .fg(Color::Rgb(255, 240, 60))
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            chapter.task_prompt,
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
    ]);

    let task_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(ACCENT))
        .title(Span::styled(
            " Objective ",
            Style::default()
                .fg(Color::Rgb(255, 240, 60))
                .add_modifier(Modifier::BOLD),
        ))
        .style(Style::default().bg(TASK_BG));

    let task_para = Paragraph::new(task_line).block(task_block);
    frame.render_widget(task_para, area);
}

// ── Terminal input pane ───────────────────────────────────────────────────────

/// Render the command-input terminal bar.
///
/// * Normal:        dark bg, ACCENT border.
/// * `flash_wrong`  > 0: red bg + red border.
/// * `flash_correct`> 0: green border.
fn draw_terminal(
    frame: &mut Frame,
    _chapter: &Chapter,
    state: &ChapterState,
    area: Rect,
    anim: &AnimState,
) {
    if area.width == 0 || area.height == 0 {
        return;
    }

    // Determine visual state
    let (border_color, bg_color) = if state.flash_wrong > 0 {
        (RED, Color::Rgb(40, 8, 8))
    } else if state.flash_correct > 0 {
        (GREEN, TERM_BG)
    } else {
        (ACCENT, TERM_BG)
    };

    // Build the input line: prompt + typed text + blinking cursor
    let prompt = Span::styled(
        "  ₿> ",
        Style::default().fg(GREEN).add_modifier(Modifier::BOLD),
    );

    let typed = Span::styled(state.input.as_str(), Style::default().fg(Color::White));

    // Blinking cursor: always rendered (the terminal owns the blink illusion
    // via state.flash_wrong / flash_correct toggling; the cursor itself is
    // always visible here so the player knows where they are).
    let blink = *anim.cursor_blink;
    let intensity = (blink * 255.0).clamp(0.0, 255.0) as u8;
    let cursor = Span::styled(
        "|",
        Style::default()
            .fg(Color::Rgb(intensity, intensity, 255))
            .add_modifier(Modifier::BOLD),
    );

    let input_line = Line::from(vec![prompt, typed, cursor]);

    let term_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color))
        .title(Span::styled(
            " Node Console ",
            Style::default().fg(border_color),
        ))
        .style(Style::default().bg(bg_color));

    let term_para = Paragraph::new(input_line).block(term_block);
    frame.render_widget(term_para, area);
}

// ── Hint panel ────────────────────────────────────────────────────────────────

/// Render the collapsible hint panel at the bottom of the screen.
///
/// Only called when `state.show_hint == true`.
/// Shows `hints[0..state.hint_level]` as bullet points, then a footer line
/// prompting the player to reveal the next hint or telling them all are shown.
fn draw_hints(
    frame: &mut Frame,
    chapter: &Chapter,
    state: &ChapterState,
    area: Rect,
    _anim: &AnimState,
) {
    if area.width == 0 || area.height == 0 {
        return;
    }

    let revealed = state.hint_level.min(chapter.hints.len());
    let all_revealed = revealed >= chapter.hints.len();

    let mut lines: Vec<Line> = Vec::new();

    // One line per revealed hint
    for (i, hint) in chapter.hints.iter().take(revealed).enumerate() {
        lines.push(Line::from(vec![
            Span::styled(
                format!("  {} ", bullet_char(i)),
                Style::default().fg(Color::Cyan),
            ),
            Span::styled(*hint, Style::default().fg(Color::Rgb(200, 220, 255))),
        ]));
    }

    // If no hints are revealed yet, show a placeholder
    if revealed == 0 {
        lines.push(Line::from(Span::styled(
            "  Press [Tab] to reveal the first hint.",
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::ITALIC),
        )));
    }

    // Footer: reveal-next or all-revealed indicator
    let footer = if all_revealed {
        Line::from(Span::styled(
            "  All hints revealed.",
            Style::default()
                .fg(Color::Rgb(100, 200, 100))
                .add_modifier(Modifier::ITALIC),
        ))
    } else {
        Line::from(vec![
            Span::styled(
                "  [Tab] ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "Reveal next hint",
                Style::default().fg(Color::Rgb(160, 200, 220)),
            ),
        ])
    };
    lines.push(Line::from("")); // breathing room before footer
    lines.push(footer);

    let hint_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .title(Span::styled(
            " Hints ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ))
        .style(Style::default().bg(HINT_BG));

    let hint_para = Paragraph::new(lines).block(hint_block);
    frame.render_widget(hint_para, area);
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Return a visually distinct bullet character for the nth hint (0-indexed).
#[inline]
fn bullet_char(index: usize) -> &'static str {
    match index {
        0 => ">",
        1 => ">>",
        _ => "-",
    }
}
