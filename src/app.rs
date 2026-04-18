// app.rs — Satoshi's Terminal state machine, event loop, and save system.
//
// State flow:
//   Menu → VolumeSelect → ChapterIntro → Playing → ChapterComplete
//       → (next chapter) or VolumeComplete → GameComplete

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{Terminal, backend::Backend};
use std::{
    io,
    time::{Duration, Instant},
};

use crate::ui::chapter::ChapterState;
use crate::{
    anim::AnimState,
    audio::{MusicPlayer, Sound, SoundManager},
    ui,
    volumes::{Chapter, Volume, all_volumes, rank_title},
};
use animate::Animate;

const TICK_RATE: Duration = Duration::from_millis(100);

// ── Save data ─────────────────────────────────────────────────────────────────

pub struct SaveData {
    pub vol_idx: usize, // 0-based index into volumes vec
    pub ch_idx: usize,  // 0-based index into current volume's chapters
    pub total_xp: u32,
    pub xp_per_chapter: Vec<Vec<u32>>, // [vol][ch]
}

fn save_path() -> Option<std::path::PathBuf> {
    dirs::home_dir().map(|h| h.join(".satoshi-terminal").join("save.json"))
}

impl SaveData {
    pub fn load() -> Self {
        if let Some(path) = save_path()
            && let Ok(data) = std::fs::read_to_string(&path)
                && let Ok(json) = serde_json::from_str::<serde_json::Value>(&data) {
                    let vol_idx = json["vol_idx"].as_u64().unwrap_or(0) as usize;
                    let ch_idx = json["ch_idx"].as_u64().unwrap_or(0) as usize;
                    let total_xp = json["total_xp"].as_u64().unwrap_or(0) as u32;
                    let xp_per_chapter = json["xp_per_chapter"]
                        .as_array()
                        .map(|vols| {
                            vols.iter()
                                .map(|v| {
                                    v.as_array()
                                        .map(|chs| {
                                            chs.iter()
                                                .map(|x| x.as_u64().unwrap_or(0) as u32)
                                                .collect()
                                        })
                                        .unwrap_or_default()
                                })
                                .collect()
                        })
                        .unwrap_or_default();
                    return Self {
                        vol_idx,
                        ch_idx,
                        total_xp,
                        xp_per_chapter,
                    };
                }
        Self {
            vol_idx: 0,
            ch_idx: 0,
            total_xp: 0,
            xp_per_chapter: vec![],
        }
    }

    pub fn save(&self) {
        if let Some(path) = save_path() {
            if let Some(dir) = path.parent() {
                let _ = std::fs::create_dir_all(dir);
            }
            let json = serde_json::json!({
                "vol_idx": self.vol_idx,
                "ch_idx": self.ch_idx,
                "total_xp": self.total_xp,
                "xp_per_chapter": self.xp_per_chapter,
            });
            let _ = std::fs::write(&path, json.to_string());
        }
    }

    pub fn reset(&mut self) {
        self.vol_idx = 0;
        self.ch_idx = 0;
        self.total_xp = 0;
        self.xp_per_chapter = vec![];
    }

    pub fn record_chapter(&mut self, vol_idx: usize, ch_idx: usize, xp: u32) {
        // Grow the jagged vec if needed
        while self.xp_per_chapter.len() <= vol_idx {
            self.xp_per_chapter.push(vec![]);
        }
        while self.xp_per_chapter[vol_idx].len() <= ch_idx {
            self.xp_per_chapter[vol_idx].push(0);
        }
        self.xp_per_chapter[vol_idx][ch_idx] = xp;
        self.total_xp = self
            .xp_per_chapter
            .iter()
            .flat_map(|v| v.iter().copied())
            .sum();
        // Advance progress pointer
        self.vol_idx = vol_idx;
        self.ch_idx = ch_idx + 1;
    }
}

// ── App state ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    /// Main menu (New Game / Continue / Quit)
    Menu {
        selected: usize,
    },
    /// Volume selection screen
    VolumeSelect {
        selected: usize,
    },
    /// Full-screen chapter intro — shows volume + chapter title, NPC first line, press Enter
    ChapterIntro {
        vol_idx: usize,
        ch_idx: usize,
    },
    /// Active gameplay
    Playing {
        vol_idx: usize,
        ch_idx: usize,
    },
    /// Chapter complete — brief celebration then auto-advance
    ChapterComplete {
        vol_idx: usize,
        ch_idx: usize,
        earned_xp: u32,
        anim_tick: usize,
    },
    /// Transition flood animation
    Transition {
        next_vol: usize,
        next_ch: usize,
        frame: usize,
    },
    /// All chapters in a volume done
    VolumeComplete {
        vol_idx: usize,
    },
    /// All volumes done
    GameComplete,
    Quit,
}

// ── App ───────────────────────────────────────────────────────────────────────

pub struct App {
    pub state: AppState,
    pub save: SaveData,
    pub sound: SoundManager,
    pub music: MusicPlayer,
    pub volumes: Vec<Volume>,
    pub chapter_state: ChapterState,
    pub anim: AnimState,
    pub anim_tick: usize,
    music_tick_counter: u8,
}

impl App {
    pub fn new() -> Self {
        let save = SaveData::load();
        Self {
            state: AppState::Menu { selected: 0 },
            save,
            sound: SoundManager::new(),
            music: MusicPlayer::new(),
            volumes: all_volumes(),
            chapter_state: ChapterState::new(),
            anim: AnimState::init(),
            anim_tick: 0,
            music_tick_counter: 0,
        }
    }

    // ── Accessors ─────────────────────────────────────────────────────────────

    pub fn current_volume(&self, vol_idx: usize) -> Option<&Volume> {
        self.volumes.get(vol_idx)
    }

    pub fn current_chapter(&self, vol_idx: usize, ch_idx: usize) -> Option<&Chapter> {
        self.volumes.get(vol_idx)?.chapters.get(ch_idx)
    }

    pub fn total_xp(&self) -> u32 {
        self.save.total_xp
    }
    pub fn rank(&self) -> &'static str {
        rank_title(self.save.total_xp)
    }

    pub fn toggle_mute(&mut self) {
        self.music.toggle_mute();
    }

    // ── Tick (called every 100 ms) ────────────────────────────────────────────

    pub fn tick(&mut self) {
        self.anim.animate();
        self.anim_tick = self.anim_tick.wrapping_add(1);

        // Decay flash timers
        if self.chapter_state.flash_wrong > 0 {
            self.chapter_state.flash_wrong -= 1;
        }
        if self.chapter_state.flash_correct > 0 {
            self.chapter_state.flash_correct -= 1;
        }

        // ChapterComplete — animate but do not auto-advance
        if let AppState::ChapterComplete {
            vol_idx,
            ch_idx,
            earned_xp: _,
            anim_tick,
        } = &self.state.clone()
        {
            let new_tick = anim_tick + 1;
            self.state = AppState::ChapterComplete {
                vol_idx: *vol_idx,
                ch_idx: *ch_idx,
                earned_xp: match &self.state {
                    AppState::ChapterComplete { earned_xp, .. } => *earned_xp,
                    _ => 0,
                },
                anim_tick: new_tick,
            };
        }

        // Transition animation frames
        if let AppState::Transition {
            next_vol,
            next_ch,
            frame,
        } = &self.state.clone()
        {
            let new_frame = frame + 1;
            if new_frame >= 30 {
                self.state = AppState::ChapterIntro {
                    vol_idx: *next_vol,
                    ch_idx: *next_ch,
                };
                self.chapter_state = ChapterState::new();
            } else {
                self.state = AppState::Transition {
                    next_vol: *next_vol,
                    next_ch: *next_ch,
                    frame: new_frame,
                };
            }
        }

        // Music tick every 5 game ticks (~500 ms)
        self.music_tick_counter = self.music_tick_counter.wrapping_add(1);
        if self.music_tick_counter.is_multiple_of(5) {
            self.music.tick();
        }
    }

    fn advance_after_complete(&mut self, vol_idx: usize, ch_idx: usize) {
        let vol = match self.volumes.get(vol_idx) {
            Some(v) => v,
            None => return,
        };
        let next_ch = ch_idx + 1;
        if next_ch < vol.chapters.len() {
            // Next chapter in same volume
            self.sound.play(Sound::Transition);
            self.anim.reset_level_anims();
            self.anim.graph_growth.set(1.0);
            self.state = AppState::Transition {
                next_vol: vol_idx,
                next_ch,
                frame: 0,
            };
        } else {
            // Volume done
            let next_vol = vol_idx + 1;
            if next_vol < self.volumes.len() {
                self.state = AppState::VolumeComplete { vol_idx };
            } else {
                self.sound.play(Sound::GameComplete);
                self.state = AppState::GameComplete;
            }
        }
    }

    // ── Key handlers ──────────────────────────────────────────────────────────

    pub fn handle_key(&mut self, key: KeyEvent) {
        // Global: Ctrl+C quits
        if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
            self.state = AppState::Quit;
            return;
        }
        // Global: M toggles music (except when typing in Playing)
        let is_playing = matches!(&self.state, AppState::Playing { .. });
        if !is_playing && (key.code == KeyCode::Char('m') || key.code == KeyCode::Char('M')) {
            self.toggle_mute();
            return;
        }

        match self.state.clone() {
            AppState::Menu { selected } => self.handle_menu(key, selected),
            AppState::VolumeSelect { selected } => self.handle_volume_select(key, selected),
            AppState::ChapterIntro { vol_idx, ch_idx } => self.handle_intro(key, vol_idx, ch_idx),
            AppState::Playing { vol_idx, ch_idx } => self.handle_playing(key, vol_idx, ch_idx),
            AppState::ChapterComplete {
                vol_idx, ch_idx, ..
            } => self.handle_chapter_complete(key, vol_idx, ch_idx),
            AppState::VolumeComplete { vol_idx } => self.handle_volume_complete(key, vol_idx),
            AppState::GameComplete => self.handle_game_complete(key),
            AppState::Transition { .. } => {} // no keys during transition
            AppState::Quit => {}
        }
    }

    fn handle_menu(&mut self, key: KeyEvent, selected: usize) {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                let s = selected.saturating_sub(1);
                self.state = AppState::Menu { selected: s };
            }
            KeyCode::Down | KeyCode::Char('j') => {
                let s = (selected + 1).min(2);
                self.state = AppState::Menu { selected: s };
            }
            KeyCode::Enter | KeyCode::Char(' ') => {
                self.sound.play(Sound::Correct);
                match selected {
                    0 => {
                        // New Game
                        self.save.reset();
                        self.chapter_state = ChapterState::new();
                        self.state = AppState::ChapterIntro {
                            vol_idx: 0,
                            ch_idx: 0,
                        };
                    }
                    1 => {
                        // Continue
                        let vi = self.save.vol_idx.min(self.volumes.len().saturating_sub(1));
                        let ci = self.save.ch_idx.min(
                            self.volumes
                                .get(vi)
                                .map(|v| v.chapters.len().saturating_sub(1))
                                .unwrap_or(0),
                        );
                        self.chapter_state = ChapterState::new();
                        self.state = AppState::ChapterIntro {
                            vol_idx: vi,
                            ch_idx: ci,
                        };
                    }
                    _ => self.state = AppState::Quit,
                }
            }
            _ => {}
        }
    }

    fn handle_volume_select(&mut self, key: KeyEvent, selected: usize) {
        let max = self.volumes.len().saturating_sub(1);
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.state = AppState::VolumeSelect {
                    selected: selected.saturating_sub(1),
                };
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.state = AppState::VolumeSelect {
                    selected: (selected + 1).min(max),
                };
            }
            KeyCode::Enter | KeyCode::Char(' ') => {
                self.chapter_state = ChapterState::new();
                self.state = AppState::ChapterIntro {
                    vol_idx: selected,
                    ch_idx: 0,
                };
            }
            KeyCode::Esc | KeyCode::Char('q') => {
                self.state = AppState::Menu { selected: 0 };
            }
            _ => {}
        }
    }

    fn handle_intro(&mut self, key: KeyEvent, vol_idx: usize, ch_idx: usize) {
        if key.code == KeyCode::Enter || key.code == KeyCode::Char(' ') {
            self.sound.play(Sound::KeyPress);
            self.chapter_state = ChapterState::new();
            self.state = AppState::Playing { vol_idx, ch_idx };
        }
        if key.code == KeyCode::Esc || key.code == KeyCode::Char('q') {
            self.state = AppState::Menu { selected: 0 };
        }
    }

    fn handle_playing(&mut self, key: KeyEvent, vol_idx: usize, ch_idx: usize) {
        let chapter = match self.current_chapter(vol_idx, ch_idx) {
            Some(c) => c.clone(),
            None => return,
        };

        // [?] toggles hint panel open/closed — never conflicts with typing
        if key.code == KeyCode::Char('?') {
            self.chapter_state.show_hint = !self.chapter_state.show_hint;
            self.anim.set_hint_open(self.chapter_state.show_hint);
            self.sound.play(Sound::KeyPress);
            return;
        }

        // [Tab] reveals next hint tier when the hint panel is already open.
        if key.code == KeyCode::Tab && self.chapter_state.show_hint {
            if self.chapter_state.hint_level < chapter.hints.len() {
                self.chapter_state.hint_level += 1;
                self.sound.play(Sound::KeyPress);
            }
            return;
        }

        self.sound.play(Sound::KeyPress);

        match key.code {
            KeyCode::Backspace => {
                self.chapter_state.input.pop();
            }
            KeyCode::Enter => {
                let input = self.chapter_state.input.trim().to_string();
                self.chapter_state.attempts += 1;

                let correct = chapter.accepted_answers.iter().any(|a| {
                    // Flexible matching: lowercase, collapse spaces
                    let norm_input: String = input
                        .split_whitespace()
                        .collect::<Vec<_>>()
                        .join(" ")
                        .to_lowercase();
                    let norm_ans: String = a
                        .split_whitespace()
                        .collect::<Vec<_>>()
                        .join(" ")
                        .to_lowercase();
                    norm_input == norm_ans
                });

                if correct {
                    // Score: base xp, -1 per extra attempt, -1 per hint revealed
                    let xp = chapter
                        .xp
                        .saturating_sub((self.chapter_state.attempts.saturating_sub(1)) * 2)
                        .saturating_sub(self.chapter_state.hint_level as u32 * 3)
                        .max(chapter.xp / 4); // floor at 25%
                    self.save.record_chapter(vol_idx, ch_idx, xp);
                    self.save.save();
                    self.sound.play(Sound::LevelComplete);
                    self.chapter_state.flash_correct = 8;
                    self.chapter_state.completed = true;
                    self.anim.start_xp_rise(xp);
                    self.state = AppState::ChapterComplete {
                        vol_idx,
                        ch_idx,
                        earned_xp: xp,
                        anim_tick: 0,
                    };
                } else {
                    self.sound.play(Sound::Error);
                    self.chapter_state.flash_wrong = 6;
                    self.chapter_state.input.clear();
                }
            }
            KeyCode::Char(c) => {
                self.chapter_state.input.push(c);
            }
            _ => {}
        }
    }

    fn handle_chapter_complete(&mut self, _key: KeyEvent, vol_idx: usize, ch_idx: usize) {
        self.advance_after_complete(vol_idx, ch_idx);
    }

    fn handle_volume_complete(&mut self, key: KeyEvent, vol_idx: usize) {
        if key.code == KeyCode::Enter || key.code == KeyCode::Char(' ') {
            let next = vol_idx + 1;
            if next < self.volumes.len() {
                self.chapter_state = ChapterState::new();
                self.state = AppState::ChapterIntro {
                    vol_idx: next,
                    ch_idx: 0,
                };
            } else {
                self.state = AppState::GameComplete;
            }
        }
        if key.code == KeyCode::Esc || key.code == KeyCode::Char('q') {
            self.state = AppState::Menu { selected: 0 };
        }
    }

    fn handle_game_complete(&mut self, key: KeyEvent) {
        if key.code == KeyCode::Enter || key.code == KeyCode::Char('q') {
            self.state = AppState::Menu { selected: 0 };
        }
    }
}

// ── Event loop ────────────────────────────────────────────────────────────────

pub fn run<B: Backend>(terminal: &mut Terminal<B>) -> Result<(), Box<dyn std::error::Error>>
where
    io::Error: From<B::Error>,
    B::Error: 'static,
{
    let mut app = App::new();
    let mut last_tick = Instant::now();

    loop {
        // Render
        let size = terminal.size()?;
        if size.width < 80 || size.height < 24 {
            terminal.draw(ui::draw_resize_warning)?;
        } else {
            terminal.draw(|f| ui::draw(f, &app))?;
        }

        // Poll events
        let timeout = TICK_RATE
            .checked_sub(last_tick.elapsed())
            .unwrap_or(Duration::ZERO);
        if event::poll(timeout)?
            && let Event::Key(key) = event::read()? {
                app.handle_key(key);
            }

        // Tick
        if last_tick.elapsed() >= TICK_RATE {
            animate::tick(TICK_RATE.as_millis() as usize);
            app.tick();
            last_tick = Instant::now();
        }

        if app.state == AppState::Quit {
            break;
        }
    }

    Ok(())
}
