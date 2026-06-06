//! Counterpoint Demo — Two melodies analyzed for motion type.
//!
//! Shows parallel, contrary, and oblique motion between agent value sequences,
//! and demonstrates why contrary motion = more productive collaboration.

use agent_counterpoint::*;

fn main() {
    println!("🎵 ══════════════════════════════════════════════════════════");
    println!("🎵  COUNTERPOINT — Motion Analysis Between Agents");
    println!("🎵 ══════════════════════════════════════════════════════════\n");

    // ═══ SCENARIO 1: Parallel Motion (bad — both doing the same thing) ═══
    println!("━━━ SCENARIO 1: Parallel Motion ━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("   Both agents move in the same direction.\n");

    let mut session_parallel = CounterpointSession::new(
        vec!["Agent A", "Agent B"],
        vec![60, 64],  // major third
    );

    let parallel_steps = vec![
        vec![62, 66],  // both up 2 → parallel
        vec![64, 68],  // both up 2 → parallel
        vec![65, 69],  // both up 1 → parallel
        vec![67, 71],  // both up 2 → parallel
    ];

    for (step, pitches) in parallel_steps.iter().enumerate() {
        let prev_a = session_parallel.voices[0].pitch;
        let prev_b = session_parallel.voices[1].pitch;
        let interval = classify_interval(pitches[0], pitches[1]);
        session_parallel.step(pitches.clone());

        println!("  Step {}: A {}→{}, B {}→{} | Interval: {:?} ({:?})",
            step + 1, prev_a, pitches[0], prev_b, pitches[1],
            interval, interval.consonance());
    }

    println!("\n  ⚠️  Parallel motion: agents are duplicating effort");
    println!("  Quality score: {:.2}", session_parallel.quality_score());
    println!("  Contrary fraction: {:.0}%", session_parallel.contrary_fraction() * 100.0);
    println!("  Parallel fifths: {}", session_parallel.parallel_fifths_count());

    // ═══ SCENARIO 2: Contrary Motion (good — complementary approaches) ═══
    println!("\n━━━ SCENARIO 2: Contrary Motion ━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("   Agents move in opposite directions — complementing each other.\n");

    let mut session_contrary = CounterpointSession::new(
        vec!["Agent A", "Agent B"],
        vec![60, 64],
    );

    let contrary_steps = vec![
        vec![62, 62],  // A up, B down → contrary
        vec![60, 60],  // A down, B down... wait
        vec![63, 57],  // A up, B down → contrary
        vec![67, 55],  // A up, B down → contrary
        vec![65, 57],  // A down, B up → contrary
        vec![64, 60],  // A down, B up → contrary
    ];

    for (step, pitches) in contrary_steps.iter().enumerate() {
        let prev_a = session_contrary.voices[0].pitch;
        let prev_b = session_contrary.voices[1].pitch;
        let motion = classify_motion(prev_a, pitches[0], prev_b, pitches[1]);
        let interval = classify_interval(pitches[0], pitches[1]);
        session_contrary.step(pitches.clone());

        let motion_icon = match motion {
            Motion::Contrary => "↕️",
            Motion::Parallel => "↗↗",
            Motion::Oblique => "→↕",
            Motion::Static => "══",
        };

        println!("  Step {}: A {}→{}, B {}→{} | {} {:?} | Interval: {:?} ({:?})",
            step + 1, prev_a, pitches[0], prev_b, pitches[1],
            motion_icon, motion, interval, interval.consonance());
    }

    println!("\n  ✅  Contrary motion: agents cover more ground independently");
    println!("  Quality score: {:.2}", session_contrary.quality_score());
    println!("  Contrary fraction: {:.0}%", session_contrary.contrary_fraction() * 100.0);
    println!("  Consonance fraction: {:.0}%", session_contrary.consonance_fraction() * 100.0);

    // ═══ SCENARIO 3: Oblique Motion (one active, one stable) ═══
    println!("\n━━━ SCENARIO 3: Oblique Motion ━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("   One agent moves while the other holds position.\n");

    let mut session_oblique = CounterpointSession::new(
        vec!["Agent A", "Agent B"],
        vec![60, 67],
    );

    let oblique_steps = vec![
        vec![62, 67],  // A moves, B holds → oblique
        vec![64, 67],  // A moves, B holds → oblique
        vec![62, 67],  // A moves, B holds → oblique
        vec![60, 67],  // A moves, B holds → oblique
    ];

    for (step, pitches) in oblique_steps.iter().enumerate() {
        let prev_a = session_oblique.voices[0].pitch;
        let prev_b = session_oblique.voices[1].pitch;
        let motion = classify_motion(prev_a, pitches[0], prev_b, pitches[1]);
        session_oblique.step(pitches.clone());

        println!("  Step {}: A {}→{}, B {}→{} | {:?}",
            step + 1, prev_a, pitches[0], prev_b, pitches[1], motion);
    }

    println!("\n  🔵  Oblique motion: one agent anchors while the other explores");

    // ═══ COMPARISON ═══
    println!("\n╔═══════════════════════════════════════════════════════╗");
    println!("║  COUNTERPOINT COMPARISON                              ║");
    println!("╠═══════════════════════════════════════════════════════╣");
    println!("║  Parallel: quality={:.2}  contrary={:.0}%             ║",
        session_parallel.quality_score(), session_parallel.contrary_fraction() * 100.0);
    println!("║  Contrary: quality={:.2}  contrary={:.0}%              ║",
        session_contrary.quality_score(), session_contrary.contrary_fraction() * 100.0);
    println!("║  Oblique:  quality={:.2}  contrary={:.0}%              ║",
        session_oblique.quality_score(), session_oblique.contrary_fraction() * 100.0);
    println!("╚═══════════════════════════════════════════════════════╝");

    println!("\n💡 In music, contrary motion creates richer harmony.");
    println!("   In multi-agent systems, agents with complementary (opposite)");
    println!("   approaches cover more ground than agents that think alike.");
}
