//! # agent-counterpoint
//!
//! Multi-agent coordination modeled on species counterpoint.
//!
//! In counterpoint, two or more independent melodies play simultaneously.
//! Each melody is self-sufficient — it makes sense on its own. But together,
//! they create harmonies that neither could produce alone.
//!
//! This is EXACTLY how good multi-agent systems work. Each agent pursues its
//! own goal independently. The system's intelligence emerges from the
//! INTERACTION of those independent lines, not from central control.
//!
//! The rules of counterpoint (no parallel fifths, contrary motion preferred,
//! resolve dissonance) translate directly to agent coordination rules:
//! - No two agents should pursue the same approach (parallel motion)
//! - Agents should complement, not mirror (contrary motion)
//! - Conflicts should resolve quickly (dissonance resolution)

/// Interval between two agents' "pitches" — their current approach vectors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Interval {
    Unison = 0,     // Same approach — both doing the same thing
    MinorSecond,    // Nearly conflicting — adjacent approaches
    MajorSecond,
    MinorThird,     // Slightly different — complementary close
    MajorThird,
    PerfectFourth,  // Supportive — one stabilizes the other
    Tritone,        // Dissonant — working at cross-purposes
    PerfectFifth,   // Harmonious — strong complementary relationship
    MinorSixth,
    MajorSixth,
    MinorSeventh,
    MajorSeventh,
    Octave,         // Same approach at different intensity levels
}

impl Interval {
    /// Classify an interval as consonant or dissonant.
    pub fn consonance(&self) -> Consonance {
        match self {
            Interval::Unison | Interval::PerfectFifth | Interval::Octave => Consonance::Perfect,
            Interval::MinorThird | Interval::MajorThird | Interval::MinorSixth | Interval::MajorSixth => Consonance::Imperfect,
            Interval::PerfectFourth => Consonance::Conditional,
            Interval::MinorSecond | Interval::MajorSecond | Interval::Tritone | Interval::MinorSeventh | Interval::MajorSeventh => Consonance::Dissonant,
        }
    }
}

/// Consonance classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Consonance {
    Dissonant = 0,   // Conflict — agents working against each other
    Conditional = 1, // OK in context — depends on what surrounds it
    Imperfect = 2,   // Good — different but complementary
    Perfect = 3,     // Optimal — strong alignment without duplication
}

/// Motion between two agents across time steps.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Motion {
    /// Both agents move in the same direction (parallel).
    Parallel,
    /// Both agents move, but in opposite directions (contrary).
    Contrary,
    /// One agent moves, the other holds (oblique).
    Oblique,
    /// Neither agent moves (static).
    Static,
}

/// A voice (agent) in the counterpoint.
#[derive(Debug, Clone)]
pub struct Voice {
    pub name: String,
    /// Current "pitch" — an abstract representation of the agent's approach.
    /// Higher = more aggressive/active, lower = more conservative/passive.
    pub pitch: i32,
    /// History of pitches for analysis.
    pub pitch_history: Vec<i32>,
}

impl Voice {
    pub fn new(name: &str, initial_pitch: i32) -> Self {
        Self { name: name.to_string(), pitch: initial_pitch, pitch_history: vec![initial_pitch] }
    }

    /// Move the voice to a new pitch.
    pub fn move_to(&mut self, new_pitch: i32) {
        self.pitch = new_pitch;
        self.pitch_history.push(new_pitch);
    }

    /// Direction of last movement: -1, 0, +1.
    pub fn direction(&self) -> i32 {
        if self.pitch_history.len() < 2 { return 0; }
        let last = self.pitch_history.len() - 1;
        (self.pitch_history[last] - self.pitch_history[last - 1]).signum()
    }
}

/// Classify the interval between two pitches.
pub fn classify_interval(a: i32, b: i32) -> Interval {
    let semitones = (a - b).abs() % 12;
    match semitones {
        0 => Interval::Unison,
        1 => Interval::MinorSecond,
        2 => Interval::MajorSecond,
        3 => Interval::MinorThird,
        4 => Interval::MajorThird,
        5 => Interval::PerfectFourth,
        6 => Interval::Tritone,
        7 => Interval::PerfectFifth,
        8 => Interval::MinorSixth,
        9 => Interval::MajorSixth,
        10 => Interval::MinorSeventh,
        11 => Interval::MajorSeventh,
        _ => Interval::Octave,
    }
}

/// Classify the motion between two voices across a time step.
pub fn classify_motion(a_before: i32, a_after: i32, b_before: i32, b_after: i32) -> Motion {
    let da = a_after - a_before;
    let db = b_after - b_before;
    if da == 0 && db == 0 { Motion::Static }
    else if da == 0 || db == 0 { Motion::Oblique }
    else if (da > 0 && db > 0) || (da < 0 && db < 0) { Motion::Parallel }
    else { Motion::Contrary }
}

/// A counterpoint session tracking multiple voices.
#[derive(Debug, Clone)]
pub struct CounterpointSession {
    pub voices: Vec<Voice>,
    pub motions: Vec<Vec<Motion>>,  // motions[step][voice_pair]
    pub intervals: Vec<Vec<Interval>>,
}

impl CounterpointSession {
    pub fn new(voice_names: Vec<&str>, initial_pitches: Vec<i32>) -> Self {
        let voices = voice_names.iter().zip(initial_pitches.iter())
            .map(|(name, &pitch)| Voice::new(name, pitch))
            .collect();
        Self { voices, motions: Vec::new(), intervals: Vec::new() }
    }

    /// Record a step: all voices move to new pitches.
    pub fn step(&mut self, new_pitches: Vec<i32>) {
        let old_pitches: Vec<i32> = self.voices.iter().map(|v| v.pitch).collect();
        for (voice, &new_pitch) in self.voices.iter_mut().zip(new_pitches.iter()) {
            voice.move_to(new_pitch);
        }
        // Record pairwise motions and intervals
        let n = self.voices.len();
        let mut step_motions = Vec::new();
        let mut step_intervals = Vec::new();
        for i in 0..n {
            for j in (i+1)..n {
                step_motions.push(classify_motion(
                    old_pitches[i], new_pitches[i],
                    old_pitches[j], new_pitches[j],
                ));
                step_intervals.push(classify_interval(new_pitches[i], new_pitches[j]));
            }
        }
        self.motions.push(step_motions);
        self.intervals.push(step_intervals);
    }

    /// Fraction of contrary motion across all steps (higher = more independent).
    pub fn contrary_fraction(&self) -> f64 {
        let total: usize = self.motions.iter().map(|m| m.len()).sum();
        if total == 0 { return 1.0; }
        let contrary: usize = self.motions.iter().flat_map(|m| m.iter())
            .filter(|m| **m == Motion::Contrary).count();
        contrary as f64 / total as f64
    }

    /// Fraction of consonant intervals across all steps.
    pub fn consonance_fraction(&self) -> f64 {
        let total: usize = self.intervals.iter().map(|i| i.len()).sum();
        if total == 0 { return 1.0; }
        let consonant: usize = self.intervals.iter().flat_map(|i| i.iter())
            .filter(|i| i.consonance() >= Consonance::Imperfect).count();
        consonant as f64 / total as f64
    }

    /// Check for forbidden parallel motion at perfect intervals.
    pub fn parallel_fifths_count(&self) -> usize {
        let mut count = 0;
        for step in 0..self.motions.len() {
            for (mi, interval) in self.motions[step].iter().zip(self.intervals[step].iter()) {
                if *mi == Motion::Parallel && matches!(interval, Interval::PerfectFifth | Interval::Octave) {
                    count += 1;
                }
            }
        }
        count
    }

    /// Overall counterpoint quality score (0.0 to 1.0).
    pub fn quality_score(&self) -> f64 {
        let contrary = self.contrary_fraction();
        let consonance = self.consonance_fraction();
        let parallel_penalty = (self.parallel_fifths_count() as f64 * 0.1).min(1.0);
        (contrary * 0.4 + consonance * 0.4 + (1.0 - parallel_penalty) * 0.2)
    }

    /// Number of steps recorded.
    pub fn steps(&self) -> usize { self.motions.len() }

    /// Number of voices.
    pub fn num_voices(&self) -> usize { self.voices.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_interval_unison() {
        assert_eq!(classify_interval(60, 60), Interval::Unison);
    }

    #[test]
    fn test_classify_interval_fifth() {
        assert_eq!(classify_interval(60, 67), Interval::PerfectFifth);
    }

    #[test]
    fn test_classify_interval_tritone() {
        assert_eq!(classify_interval(60, 66), Interval::Tritone);
    }

    #[test]
    fn test_consonance() {
        assert!(Interval::PerfectFifth.consonance() >= Consonance::Perfect);
        assert!(Interval::Tritone.consonance() == Consonance::Dissonant);
    }

    #[test]
    fn test_contrary_motion() {
        assert_eq!(classify_motion(60, 62, 64, 62), Motion::Contrary);
    }

    #[test]
    fn test_parallel_motion() {
        assert_eq!(classify_motion(60, 62, 64, 66), Motion::Parallel);
    }

    #[test]
    fn test_oblique_motion() {
        assert_eq!(classify_motion(60, 62, 64, 64), Motion::Oblique);
    }

    #[test]
    fn test_session_quality_good_counterpoint() {
        // Contrary motion, consonant intervals
        let mut session = CounterpointSession::new(
            vec!["agent-a", "agent-b"],
            vec![60, 64],  // major third — consonant
        );
        // A goes up, B goes down → contrary + consonant
        session.step(vec![62, 62]);
        // A goes up, B goes up → parallel but third = OK
        session.step(vec![64, 64]);
        assert!(session.contrary_fraction() > 0.0);
        assert!(session.consonance_fraction() > 0.5);
        assert!(session.quality_score() > 0.5);
    }

    #[test]
    fn test_session_parallel_fifths_detected() {
        let mut session = CounterpointSession::new(
            vec!["a", "b"],
            vec![60, 67],  // fifth
        );
        // Both move up by same amount = parallel fifths
        session.step(vec![62, 69]); // still a fifth, parallel motion
        assert!(session.parallel_fifths_count() > 0);
    }

    #[test]
    fn test_voice_direction() {
        let mut v = Voice::new("test", 60);
        assert_eq!(v.direction(), 0); // no movement yet
        v.move_to(62);
        assert_eq!(v.direction(), 1);
        v.move_to(60);
        assert_eq!(v.direction(), -1);
    }

    #[test]
    fn test_three_voice_session() {
        let mut session = CounterpointSession::new(
            vec!["bass", "tenor", "soprano"],
            vec![48, 60, 72],
        );
        session.step(vec![50, 59, 71]); // contrary motion
        session.step(vec![52, 57, 69]);
        assert_eq!(session.num_voices(), 3);
        assert_eq!(session.steps(), 2);
    }
}
