use ratatui::{
    Frame,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

// Bitcoin logo tile — same 7×7 footprint as the original GitQuest diamond,
// but without the box-drawing border.  Just a single ₿ centred in empty space.

const LOGO_ROWS: &[&str] = &[
    "       ",
    "       ",
    "       ",
    "   ₿   ",
    "       ",
    "       ",
    "       ",
];

const TILE_H: usize = 7;

fn logo_row_chars() -> Vec<Vec<char>> {
    LOGO_ROWS.iter().map(|s| s.chars().collect()).collect()
}

pub fn draw_transition(frame: &mut Frame, _next_vol: usize, anim_frame: usize, shimmer: u8) {
    let area = frame.area();
    let width = area.width as usize;
    let height = area.height as usize;

    if width == 0 || height == 0 {
        return;
    }

    let logo_chars: Vec<Vec<char>> = logo_row_chars();
    let tile_w = logo_chars.iter().map(|r| r.len()).max().unwrap_or(7);

    let cols = width.div_ceil(tile_w);
    let rows = height.div_ceil(TILE_H);
    let total_tiles = cols * rows;

    if total_tiles == 0 {
        return;
    }

    // Deterministic pseudo-random flood order
    let mut order: Vec<(usize, usize)> = (0..rows)
        .flat_map(|r| (0..cols).map(move |c| (r, c)))
        .collect();
    let len = order.len();
    for i in 0..len {
        let j = (i.wrapping_mul(6271).wrapping_add(1337)) % len;
        order.swap(i, j);
    }

    // Phase: 30 ticks total
    //  0-12  → flood in
    //  13-19 → hold
    //  20-29 → drain out
    let visible_count = if anim_frame <= 12 {
        if anim_frame == 0 {
            0
        } else {
            (anim_frame * total_tiles) / 12
        }
    } else if anim_frame <= 19 {
        total_tiles
    } else {
        let drain = anim_frame - 20;
        total_tiles.saturating_sub((drain + 1) * total_tiles / 10)
    }
    .min(total_tiles);

    let visible: std::collections::HashSet<(usize, usize)> =
        order.iter().take(visible_count).copied().collect();

    let s = shimmer;
    let bitcoin_color = Color::Rgb(
        (247u8).saturating_add(s / 4),
        (147u8).saturating_add(s / 3),
        (26u8).saturating_add(s / 2),
    );

    let mut lines: Vec<Line> = Vec::with_capacity(height);

    for screen_row in 0..height {
        let tile_row = screen_row / TILE_H;
        let local_row = screen_row % TILE_H;
        let row_chars: &[char] = &logo_chars[local_row.min(logo_chars.len() - 1)];

        let mut spans: Vec<Span> = Vec::new();
        let mut screen_col = 0usize;

        while screen_col < width {
            let tile_col = screen_col / tile_w;
            let local_col = screen_col % tile_w;
            let take = (tile_w - local_col).min(width - screen_col);

            let chunk: String = (local_col..local_col + take)
                .map(|ci| {
                    if ci < row_chars.len() {
                        row_chars[ci]
                    } else {
                        ' '
                    }
                })
                .collect();

            let style = if visible.contains(&(tile_row, tile_col)) {
                Style::default()
                    .fg(bitcoin_color)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Rgb(12, 12, 18))
            };

            spans.push(Span::styled(chunk, style));
            screen_col += take;
        }

        lines.push(Line::from(spans));
    }

    let p = Paragraph::new(lines).style(Style::default().bg(Color::Rgb(8, 8, 14)));
    frame.render_widget(p, area);
}
