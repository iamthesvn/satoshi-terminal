#[cfg(feature = "audio")]
use rodio::{OutputStream, OutputStreamHandle, Sink, buffer::SamplesBuffer};

// ─── constants ────────────────────────────────────────────────────────────────
const SAMPLE_RATE: u32 = 44_100;
const MASTER_VOL: f32 = 0.18;

pub struct MusicPlayer {
    #[cfg(feature = "audio")]
    _stream: Option<OutputStream>,
    #[cfg(feature = "audio")]
    handle: Option<OutputStreamHandle>,
    #[cfg(feature = "audio")]
    sink: Option<Sink>,

    muted: bool,
    current_track: usize,
}

impl MusicPlayer {
    pub fn new() -> Self {
        #[cfg(feature = "audio")]
        {
            match OutputStream::try_default() {
                Ok((stream, handle)) => Self {
                    _stream: Some(stream),
                    handle: Some(handle),
                    sink: None,
                    muted: false,
                    current_track: 0,
                },
                Err(_) => Self {
                    _stream: None,
                    handle: None,
                    sink: None,
                    muted: false,
                    current_track: 0,
                },
            }
        }
        #[cfg(not(feature = "audio"))]
        Self {
            muted: false,
            current_track: 0,
        }
    }

    pub fn tick(&mut self) {
        #[cfg(feature = "audio")]
        {
            if self.muted {
                return;
            }
            let needs_next = match &self.sink {
                None => true,
                Some(s) => s.empty(),
            };
            if needs_next {
                self.play_next();
            }
        }
    }

    pub fn toggle_mute(&mut self) {
        self.muted = !self.muted;
        #[cfg(feature = "audio")]
        {
            if self.muted {
                if let Some(s) = &self.sink {
                    s.pause();
                }
            } else {
                match &self.sink {
                    Some(s) if !s.empty() => s.play(),
                    _ => self.play_next(),
                }
            }
        }
    }

    #[cfg(feature = "audio")]
    fn play_next(&mut self) {
        let handle = match &self.handle {
            Some(h) => h,
            None => return,
        };
        let sink = match Sink::try_new(handle) {
            Ok(s) => s,
            Err(_) => return,
        };

        let samples = render_track(self.current_track);
        let buf = SamplesBuffer::new(1, SAMPLE_RATE, samples);
        sink.append(buf);
        self.sink = Some(sink);
        self.current_track = (self.current_track + 1) % TRACK_COUNT;
    }
}

// ─── Track catalogue ──────────────────────────────────────────────────────────

const TRACK_COUNT: usize = 3;

fn render_track(idx: usize) -> Vec<f32> {
    match idx {
        0 => render_genesis_block(), // C minor — dark, propulsive
        1 => render_mempool_wait(),  // A minor pentatonic — bright, bouncy
        2 => render_hash_wars(),     // D minor — urgent, driving
        _ => render_genesis_block(),
    }
}

// ─── Oscillator primitives (all sine-based = alias-free) ─────────────────────

fn sin_osc(freq: f32, t: f32) -> f32 {
    (2.0 * std::f32::consts::PI * freq * t).sin()
}

/// Rich bass: fundamental + octave + slight 5th harmonic for punch
fn bass_rich(freq: f32, t: f32, amp: f32) -> f32 {
    let f = sin_osc(freq, t) * 0.60 + sin_osc(freq * 2.0, t) * 0.30 + sin_osc(freq * 3.0, t) * 0.10;
    f * amp
}

/// Kick drum: sine sweep from high to low for punch without aliasing
fn kick(t: f32, amp: f32) -> f32 {
    if !(0.0..=0.10).contains(&t) {
        return 0.0;
    }
    let env = 1.0 - (t / 0.10);
    let sweep = 160.0 * (1.0 - t * 8.0).max(0.2); // 160Hz → ~32Hz
    sin_osc(sweep, t) * env * amp
}

/// Snare / clap: two sines with fast decay
fn snare(t: f32, amp: f32) -> f32 {
    if !(0.0..=0.08).contains(&t) {
        return 0.0;
    }
    let env = (-t * 40.0).exp();
    (sin_osc(220.0, t) * 0.6 + sin_osc(330.0, t) * 0.4) * env * amp
}

/// Hi-hat click: very short high sine burst
fn hihat(t: f32, amp: f32) -> f32 {
    if !(0.0..=0.03).contains(&t) {
        return 0.0;
    }
    let env = (-t * 120.0).exp();
    sin_osc(8000.0, t) * env * amp
}

/// Electric pluck: sine + 2nd harmonic with fast attack/decay
fn pluck(freq: f32, t: f32, note_dur: f32, amp: f32) -> f32 {
    let progress = (t / note_dur).min(1.0);
    let env = (-progress * 6.0).exp();
    let tone = sin_osc(freq, t) * 0.75 + sin_osc(freq * 2.01, t) * 0.25;
    tone * env * amp
}

/// Energetic pad: 3 detuned sines + slow amplitude modulation
fn pad_rich(freq: f32, t: f32, amp: f32) -> f32 {
    let detune = freq * 0.010;
    let s = sin_osc(freq, t) * 0.55
        + sin_osc(freq + detune, t) * 0.30
        + sin_osc(freq - detune * 0.6, t) * 0.15;
    s * amp
}

// ─── Track 1: Genesis Block (C minor) ────────────────────────────────────────
// 92 BPM — driving bass, fast pluck arpeggio, four-on-the-floor
fn render_genesis_block() -> Vec<f32> {
    let bpm = 92.0f32;
    let beat = 60.0 / bpm;
    let bars = 8;
    let total_secs = beat * 4.0 * bars as f32;
    let n_samples = (SAMPLE_RATE as f32 * total_secs) as usize;
    let mut out = vec![0.0f32; n_samples];

    let arp: &[(f32, f32)] = &[
        (261.63, 0.0),
        (311.13, 0.5),
        (392.00, 1.0),
        (466.16, 1.5),
        (523.25, 2.0),
        (392.00, 2.5),
        (311.13, 3.0),
        (261.63, 3.5),
        (196.00, 4.0),
        (261.63, 4.5),
        (311.13, 5.0),
        (523.25, 5.5),
        (392.00, 6.0),
        (311.13, 6.5),
        (261.63, 7.0),
        (196.00, 7.5),
        (261.63, 8.0),
        (311.13, 8.5),
        (392.00, 9.0),
        (466.16, 9.5),
        (523.25, 10.0),
        (466.16, 10.5),
        (392.00, 11.0),
        (311.13, 11.5),
        (261.63, 12.0),
        (523.25, 12.5),
        (392.00, 13.0),
        (311.13, 13.5),
        (261.63, 14.0),
        (196.00, 14.5),
        (130.81, 15.0),
        (261.63, 15.5),
        (311.13, 16.0),
        (392.00, 16.5),
        (466.16, 17.0),
        (523.25, 17.5),
        (392.00, 18.0),
        (311.13, 18.5),
        (261.63, 19.0),
        (196.00, 19.5),
        (261.63, 20.0),
        (311.13, 20.5),
        (392.00, 21.0),
        (466.16, 21.5),
        (523.25, 22.0),
        (392.00, 22.5),
        (311.13, 23.0),
        (261.63, 23.5),
        (196.00, 24.0),
        (261.63, 24.5),
        (311.13, 25.0),
        (392.00, 25.5),
        (523.25, 26.0),
        (392.00, 26.5),
        (311.13, 27.0),
        (261.63, 27.5),
        (196.00, 28.0),
        (261.63, 28.5),
        (311.13, 29.0),
        (392.00, 29.5),
        (523.25, 30.0),
        (466.16, 30.5),
        (392.00, 31.0),
        (311.13, 31.5),
    ];

    for (i, slot) in out.iter_mut().enumerate() {
        let t_abs = i as f32 / SAMPLE_RATE as f32;
        let beat_abs = t_abs / beat;
        let mut s = 0.0f32;

        s += bass_rich(65.41, t_abs, 0.26);

        let bar_phase = beat_abs % 4.0;
        let pad_env = if bar_phase < 3.4 {
            1.0
        } else {
            (4.0 - bar_phase) / 0.6
        };
        s += pad_rich(130.81, t_abs, 0.14 * pad_env);
        s += pad_rich(155.56, t_abs, 0.10 * pad_env);
        s += pad_rich(196.00, t_abs, 0.10 * pad_env);

        for &(freq, start_beat) in arp {
            let note_t = beat_abs - start_beat;
            if (0.0..0.5).contains(&note_t) {
                s += pluck(freq, note_t * beat, 0.5 * beat, 0.30);
            }
        }

        let beat_t = (beat_abs % 1.0) * beat;
        s += kick(beat_t, 0.45);
        let half_beat = beat_abs % 1.0;
        if (half_beat - 0.5).abs() < 0.08 {
            s += snare((half_beat - 0.5).abs() * beat * 12.0, 0.28);
        }

        *slot = (s * MASTER_VOL).clamp(-1.0, 1.0);
    }
    out
}

// ─── Track 2: Mempool Wait (A minor pentatonic) ──────────────────────────────
// 100 BPM — bouncy, bright, syncopated
fn render_mempool_wait() -> Vec<f32> {
    let bpm = 100.0f32;
    let beat = 60.0 / bpm;
    let bars = 8;
    let total_secs = beat * 4.0 * bars as f32;
    let n_samples = (SAMPLE_RATE as f32 * total_secs) as usize;
    let mut out = vec![0.0f32; n_samples];

    let melody: &[(f32, f32)] = &[
        (440.00, 0.0),
        (523.25, 0.5),
        (587.33, 1.0),
        (659.25, 1.5),
        (784.00, 2.0),
        (880.00, 2.5),
        (784.00, 3.0),
        (659.25, 3.5),
        (587.33, 4.0),
        (523.25, 4.5),
        (440.00, 5.0),
        (392.00, 5.5),
        (440.00, 6.0),
        (587.33, 6.5),
        (659.25, 7.0),
        (784.00, 7.5),
        (880.00, 8.0),
        (1046.50, 8.5),
        (880.00, 9.0),
        (784.00, 9.5),
        (659.25, 10.0),
        (587.33, 10.5),
        (523.25, 11.0),
        (440.00, 11.5),
        (329.63, 12.0),
        (392.00, 12.5),
        (440.00, 13.0),
        (523.25, 13.5),
        (587.33, 14.0),
        (659.25, 14.5),
        (784.00, 15.0),
        (880.00, 15.5),
        (440.00, 16.0),
        (523.25, 16.5),
        (587.33, 17.0),
        (659.25, 17.5),
        (784.00, 18.0),
        (880.00, 18.5),
        (1046.50, 19.0),
        (880.00, 19.5),
        (784.00, 20.0),
        (659.25, 20.5),
        (587.33, 21.0),
        (523.25, 21.5),
        (440.00, 22.0),
        (392.00, 22.5),
        (329.63, 23.0),
        (392.00, 23.5),
        (440.00, 24.0),
        (523.25, 24.5),
        (587.33, 25.0),
        (659.25, 25.5),
        (784.00, 26.0),
        (880.00, 26.5),
        (1046.50, 27.0),
        (1174.66, 27.5),
        (1318.51, 28.0),
        (1174.66, 28.5),
        (1046.50, 29.0),
        (880.00, 29.5),
        (784.00, 30.0),
        (659.25, 30.5),
        (587.33, 31.0),
        (523.25, 31.5),
    ];

    for (i, slot) in out.iter_mut().enumerate() {
        let t_abs = i as f32 / SAMPLE_RATE as f32;
        let beat_abs = t_abs / beat;
        let mut s = 0.0f32;

        s += bass_rich(110.0, t_abs, 0.24);

        let bar_phase = beat_abs % 4.0;
        let pad_env = if bar_phase < 3.5 {
            1.0
        } else {
            (4.0 - bar_phase) / 0.5
        };
        s += pad_rich(110.0, t_abs, 0.13 * pad_env);
        s += pad_rich(130.81, t_abs, 0.10 * pad_env);
        s += pad_rich(164.81, t_abs, 0.10 * pad_env);

        for &(freq, start_beat) in melody {
            let note_t = beat_abs - start_beat;
            if (0.0..0.5).contains(&note_t) {
                s += pluck(freq, note_t * beat, 0.5 * beat, 0.28);
            }
        }

        let beat_t = (beat_abs % 1.0) * beat;
        s += kick(beat_t, 0.42);
        let sixteenth = beat_abs % 1.0;
        if (sixteenth - 0.5).abs() < 0.08 {
            s += snare((sixteenth - 0.5).abs() * beat * 12.0, 0.25);
        }
        if (sixteenth - 0.25).abs() < 0.04 || (sixteenth - 0.75).abs() < 0.04 {
            s += hihat((sixteenth - 0.25).abs() * beat * 25.0, 0.10);
        }

        *slot = (s * MASTER_VOL).clamp(-1.0, 1.0);
    }
    out
}

// ─── Track 3: Hash Wars (D minor) ────────────────────────────────────────────
// 112 BPM — urgent, rapid-fire, heavy pulse
fn render_hash_wars() -> Vec<f32> {
    let bpm = 112.0f32;
    let beat = 60.0 / bpm;
    let bars = 8;
    let total_secs = beat * 4.0 * bars as f32;
    let n_samples = (SAMPLE_RATE as f32 * total_secs) as usize;
    let mut out = vec![0.0f32; n_samples];

    let melody: &[(f32, f32)] = &[
        (293.66, 0.0),
        (349.23, 0.25),
        (440.00, 0.5),
        (523.25, 0.75),
        (587.33, 1.0),
        (523.25, 1.25),
        (440.00, 1.5),
        (349.23, 1.75),
        (293.66, 2.0),
        (220.00, 2.25),
        (261.63, 2.5),
        (293.66, 2.75),
        (349.23, 3.0),
        (440.00, 3.25),
        (523.25, 3.5),
        (587.33, 3.75),
        (698.46, 4.0),
        (587.33, 4.25),
        (523.25, 4.5),
        (440.00, 4.75),
        (349.23, 5.0),
        (293.66, 5.25),
        (220.00, 5.5),
        (146.83, 5.75),
        (293.66, 6.0),
        (349.23, 6.25),
        (440.00, 6.5),
        (523.25, 6.75),
        (587.33, 7.0),
        (698.46, 7.25),
        (880.00, 7.5),
        (1046.50, 7.75),
        (1174.66, 8.0),
        (1046.50, 8.25),
        (880.00, 8.5),
        (698.46, 8.75),
        (587.33, 9.0),
        (523.25, 9.25),
        (440.00, 9.5),
        (349.23, 9.75),
        (293.66, 10.0),
        (349.23, 10.25),
        (440.00, 10.5),
        (523.25, 10.75),
        (587.33, 11.0),
        (440.00, 11.25),
        (349.23, 11.5),
        (293.66, 11.75),
        (220.00, 12.0),
        (293.66, 12.25),
        (349.23, 12.5),
        (440.00, 12.75),
        (587.33, 13.0),
        (698.46, 13.25),
        (880.00, 13.5),
        (1046.50, 13.75),
        (1174.66, 14.0),
        (880.00, 14.25),
        (698.46, 14.5),
        (587.33, 14.75),
        (440.00, 15.0),
        (349.23, 15.25),
        (293.66, 15.5),
        (220.00, 15.75),
        (293.66, 16.0),
        (349.23, 16.25),
        (440.00, 16.5),
        (523.25, 16.75),
        (587.33, 17.0),
        (698.46, 17.25),
        (880.00, 17.5),
        (1046.50, 17.75),
        (1174.66, 18.0),
        (1046.50, 18.25),
        (880.00, 18.5),
        (698.46, 18.75),
        (587.33, 19.0),
        (523.25, 19.25),
        (440.00, 19.5),
        (349.23, 19.75),
        (293.66, 20.0),
        (349.23, 20.25),
        (440.00, 20.5),
        (523.25, 20.75),
        (587.33, 21.0),
        (440.00, 21.25),
        (349.23, 21.5),
        (293.66, 21.75),
        (220.00, 22.0),
        (146.83, 22.25),
        (293.66, 22.5),
        (349.23, 22.75),
        (440.00, 23.0),
        (587.33, 23.25),
        (698.46, 23.5),
        (880.00, 23.75),
        (1046.50, 24.0),
        (880.00, 24.25),
        (698.46, 24.5),
        (587.33, 24.75),
        (440.00, 25.0),
        (349.23, 25.25),
        (293.66, 25.5),
        (220.00, 25.75),
        (146.83, 26.0),
        (220.00, 26.25),
        (293.66, 26.5),
        (349.23, 26.75),
        (440.00, 27.0),
        (523.25, 27.25),
        (587.33, 27.5),
        (698.46, 27.75),
        (880.00, 28.0),
        (698.46, 28.25),
        (587.33, 28.5),
        (523.25, 28.75),
        (440.00, 29.0),
        (349.23, 29.25),
        (293.66, 29.5),
        (220.00, 29.75),
        (293.66, 30.0),
        (349.23, 30.25),
        (440.00, 30.5),
        (523.25, 30.75),
        (587.33, 31.0),
        (440.00, 31.25),
        (349.23, 31.5),
        (293.66, 31.75),
    ];

    for (i, slot) in out.iter_mut().enumerate() {
        let t_abs = i as f32 / SAMPLE_RATE as f32;
        let beat_abs = t_abs / beat;
        let mut s = 0.0f32;

        s += bass_rich(73.42, t_abs, 0.28);

        let bar_phase = beat_abs % 4.0;
        let pad_env = if bar_phase < 3.6 {
            1.0
        } else {
            (4.0 - bar_phase) / 0.4
        };
        s += pad_rich(146.83, t_abs, 0.14 * pad_env);
        s += pad_rich(174.61, t_abs, 0.11 * pad_env);
        s += pad_rich(220.00, t_abs, 0.11 * pad_env);

        for &(freq, start_beat) in melody {
            let note_t = beat_abs - start_beat;
            if (0.0..0.25).contains(&note_t) {
                s += pluck(freq, note_t * beat, 0.25 * beat, 0.30);
            }
        }

        let beat_t = (beat_abs % 1.0) * beat;
        s += kick(beat_t, 0.48);
        let bar_beat = beat_abs % 4.0;
        if (bar_beat - 1.0).abs() < 0.08 || (bar_beat - 3.0).abs() < 0.08 {
            s += snare((bar_beat - 1.0).abs() * beat * 10.0, 0.28);
        }
        let sixteenth = beat_abs % 1.0;
        if (sixteenth - 0.25).abs() < 0.04 || (sixteenth - 0.75).abs() < 0.04 {
            s += hihat((sixteenth - 0.25).abs() * beat * 35.0, 0.09);
        }

        *slot = (s * MASTER_VOL).clamp(-1.0, 1.0);
    }
    out
}
