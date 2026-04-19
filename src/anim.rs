#![allow(clippy::too_many_arguments)]

// anim.rs — vyfor/animate integration for Satoshi's Terminal.
//
// These are *structural* animations: they run independently of the 100ms
// game tick and make the UI feel alive.

use animate::animate;
use ratatui::style::Color;

/// Animation state managed by the `animate` crate.
/// Call `animate::tick(delta_ms)` in the event loop, then `anim.animate()`
/// every frame to advance the values.
#[animate]
pub struct AnimState {
    /// Terminal cursor opacity: blinks between 0.3 and 1.0.
    #[alternate(duration = 530, easing = quad_in_out)]
    pub cursor_blink: f64,

    /// Hint panel slide: 0.0 = closed, 1.0 = fully open.
    /// Triggered by pressing `?` in-game.
    #[once(duration = 200, easing = quad_out)]
    pub hint_openness: f64,

    /// XP counter rise on chapter-complete screen.
    /// Set target to the earned XP when the screen appears.
    #[once(duration = 800, easing = cubic_out)]
    pub xp_rise: u32,

    /// Transition tile shimmer: brightens/darkens the Bitcoin-orange flood tiles.
    #[alternate(duration = 900, easing = quad_in_out)]
    pub transition_shimmer: u8,

    /// Git graph growth during transitions (0.0 = nothing, 1.0 = full graph).
    /// Visualises the player's branching knowledge expanding.
    #[once(duration = 1200, easing = quad_out)]
    pub graph_growth: f64,

    /// Warm border colour that breathes between accent orange and bright gold.
    /// Used on the menu box, volume-complete and game-complete borders.
    #[alternate(duration = 2500, easing = quad_in_out)]
    pub border_breathe: Color,

    /// Success border colour that breathes between green and gold.
    /// Used exclusively on the chapter-complete celebration screen.
    #[alternate(duration = 1200, easing = quad_in_out)]
    pub success_breathe: Color,

    /// Chapter-intro typewriter: reveals NPC dialogue character-by-character.
    /// Target is set to the full dialogue when entering the intro screen.
    #[once(duration = 2000, easing = quad_out)]
    pub intro_typewriter: String,
}

impl AnimState {
    /// Construct with sensible initial values and prime the continuous
    /// animations (alternate / cycle) by calling `set()` once.
    pub fn init() -> Self {
        let mut s = Self::new(
            0.3,
            0.0,
            0,
            120,
            0.0,
            Color::Rgb(255, 120, 40),
            Color::Rgb(60, 220, 100),
            String::new(),
        );
        // Start the continuous ones
        s.cursor_blink.set(1.0);
        s.transition_shimmer.set(180);
        s.border_breathe.set(Color::Rgb(255, 200, 100));
        s.success_breathe.set(Color::Rgb(255, 215, 0));
        s
    }

    /// Trigger the hint panel to open or close.
    pub fn set_hint_open(&mut self, open: bool) {
        self.hint_openness.set(if open { 1.0 } else { 0.0 });
    }

    /// Trigger the XP counter animation from 0 up to `earned`.
    pub fn start_xp_rise(&mut self, earned: u32) {
        self.xp_rise.set(earned);
    }

    /// Reset the one-shot animations that are tied to level/chapter transitions.
    pub fn reset_level_anims(&mut self) {
        self.hint_openness.set(0.0);
        self.xp_rise.set(0);
        self.graph_growth.set(0.0);
        self.intro_typewriter.set(String::new());
    }
}
