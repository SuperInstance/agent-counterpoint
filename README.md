# agent-counterpoint

**Species counterpoint as fleet coordination.**

In 1725, Johann Joseph Fux wrote *Gradus ad Parnassum*, a textbook that taught Mozart, Beethoven, and Haydn how to write music for multiple independent voices. The rules are simple:

1. **Prefer contrary motion** — if one voice goes up, the other should go down
2. **Avoid parallel fifths and octaves** — two voices moving in the same direction to the same interval is wasteful
3. **Resolve dissonance quickly** — clashes are fine as tension, unforgivable as permanent state

These rules produce music where each voice is independent yet harmonious. The whole is greater than the sum. And — here's the key — these exact same rules predict good multi-agent coordination.

`agent-counterpoint` applies species counterpoint to fleet management. Parallel fifths become redundant agents. Contrary motion becomes productive differentiation. Dissonance resolution becomes conflict management. The rules that governed 16th-century polyphony govern 21st-century agent fleets.

## Why This Exists

Most multi-agent coordination approaches are either centralized (one brain controls everything) or emergent (agents figure it out). Counterpoint offers a third path: **rule-governed independence.** Each agent pursues its own goal, but a set of constraints ensures the voices work together.

The rules aren't arbitrary. Parallel fifths sound bad because they waste the potential of two independent voices — if they're moving in parallel, one is redundant. In agent terms: if two agents are doing the same thing, one of them is wasted compute. Contrary motion sounds good because it maximizes the information content of the combined output. In agent terms: agents approaching a problem from different angles cover more ground.

## Core Idea

Every agent is a voice with a "pitch" — an abstract representation of its approach. Higher pitch = more aggressive/active, lower = more conservative/passive. The relationships between voices determine coordination quality.

**Interval classification** (12 intervals, from unison to octave):

| Consonance | Intervals | Agent Interpretation |
|------------|-----------|---------------------|
| Perfect | Unison, P5, Octave | Strong alignment — but risk of redundancy |
| Imperfect | m3, M3, m6, M6 | Complementary — different but harmonious |
| Conditional | P4 | OK in context — depends on surroundings |
| Dissonant | m2, M2, Tritone, m7, M7 | Conflict — working at cross-purposes |

**Motion classification** (how two agents change between steps):

| Motion | Pattern | Quality |
|--------|---------|---------|
| Contrary | A↑ B↓ | Best — maximum independence |
| Oblique | A↑ B→ | Good — one leads, one holds |
| Parallel | A↑ B↑ | Risky — potential redundancy |
| Static | A→ B→ | Neutral — no movement |

## Architecture

```
Voice (agent with pitch history)
  └─ move_to(pitch) → updates pitch, records history

Interval (12 types from Unison to Octave)
  └─ consonance() → Perfect / Imperfect / Conditional / Dissonant

classify_interval(a, b) → Interval
classify_motion(a_before, a_after, b_before, b_after) → Motion

CounterpointSession
  ├─ voices: Vec<Voice>
  ├─ step(new_pitches) → records pairwise motions + intervals
  ├─ contrary_fraction() → f64 (higher = more independent)
  ├─ consonance_fraction() → f64 (higher = more harmonious)
  ├─ parallel_fifths_count() → usize (forbidden motion detection)
  └─ quality_score() → f64 (0.0 to 1.0)
```

## Usage

### Create a Counterpoint Session

```rust
use agent_counterpoint::*;

// Two agents starting at different "pitches"
let mut session = CounterpointSession::new(
    vec!["search-agent", "rank-agent"],
    vec![60, 64],  // major third — consonant start
);
```

### Record Steps and Analyze

```rust
// Agents move to new approaches
session.step(vec![62, 62]);  // search goes up, rank comes down → contrary motion
session.step(vec![64, 60]);  // search up, rank down → more contrary

// How's the counterpoint?
println!("Contrary motion: {:.0}%", session.contrary_fraction() * 100.0);
println!("Consonance: {:.0}%", session.consonance_fraction() * 100.0);
println!("Parallel fifths: {}", session.parallel_fifths_count());
println!("Quality: {:.2}", session.quality_score());
```

### Detecting Redundancy (Parallel Fifths)

```rust
let mut session = CounterpointSession::new(vec!["a", "b"], vec![60, 67]); // fifth
session.step(vec![62, 69]); // both move up by 2 → parallel fifths!

assert!(session.parallel_fifths_count() > 0);
// These agents are doing the same thing at different scales — one is redundant
```

This is the most practically useful check. If two agents consistently show parallel motion at perfect intervals, they're not adding independent value. One should be reassigned, retrained, or given a different objective.

### Three-Voice Coordination

```rust
let mut session = CounterpointSession::new(
    vec!["bass", "tenor", "soprano"],
    vec![48, 60, 72],
);

// Bass up, tenor down, soprano down → rich contrary motion
session.step(vec![50, 59, 71]);
session.step(vec![52, 57, 69]);

assert_eq!(session.num_voices(), 3);
assert_eq!(session.steps(), 2);
```

With three or more voices, the pairwise analysis catches interactions you'd miss with simple "are agents different?" checks. Agent A and B might have good counterpoint while B and C have parallel motion. The full pairwise matrix reveals these hidden relationships.

### Voice Direction

```rust
let mut voice = Voice::new("agent-x", 60);
voice.move_to(65); // going up
assert_eq!(voice.direction(), 1);

voice.move_to(58); // coming down
assert_eq!(voice.direction(), -1);
```

## API Reference

| Type | Purpose |
|------|---------|
| `Voice` | Agent with pitch and movement history |
| `Interval` | 12 interval types (Unison through Octave) |
| `Consonance` | 4-level classification (Dissonant → Perfect) |
| `Motion` | Parallel / Contrary / Oblique / Static |
| `CounterpointSession` | Multi-voice tracking and analysis |

### Key Functions

| Function | Returns |
|----------|---------|
| `classify_interval(a, b)` | Interval between two pitches |
| `classify_motion(...)` | Motion type between two time steps |

### CounterpointSession Methods

| Method | Returns |
|--------|---------|
| `step(pitches)` | Record a time step |
| `contrary_fraction()` | Fraction of contrary motion |
| `consonance_fraction()` | Fraction of consonant intervals |
| `parallel_fifths_count()` | Count of forbidden parallel motion |
| `quality_score()` | Overall score (0.0-1.0) |
| `steps()` | Number of recorded steps |
| `num_voices()` | Number of voices |

## The Deeper Idea

The quality score formula reveals the philosophy:

```
quality = contrary × 0.4 + consonance × 0.4 + (1 - parallel_penalty) × 0.2
```

Contrary motion and consonance are weighted equally. This isn't arbitrary — it encodes a deep truth about coordination. **Independence** (contrary motion) and **harmony** (consonance) are both valuable, and they're in tension. Maximum independence means every voice goes its own way, but that risks dissonance. Maximum harmony means all voices agree, but that risks redundancy.

The sweet spot is the center: agents that are independent but not antagonistic, harmonious but not identical. Counterpoint has been finding that sweet spot for 500 years.

The parallel penalty is weighted at 0.2 but caps at 1.0 (10+ parallel fifths → zero bonus). This reflects the real rule: one parallel fifth is a minor flaw, but habitual parallel motion means the voices aren't really independent. It's not a bug — it's a design failure.

## Related Crates

- **`agent-groove`** — Timing and feel for scheduling (the *pocket*)
- **`agent-phrasing`** — Energy contour detection (the *shape*)
- **`agent-intonation`** — Accuracy measurement (how *in tune*)
- **`agent-orchestration`** — Fleet dynamics as orchestral composition (who plays *loud*)
- **`agent-ensemble`** — The experiment proving musical coordination outperforms the alternatives

## License

MIT
