# agent-counterpoint

*Two independent melodies. Neither knows what the other is playing. Together, they create something neither could write alone.*

---

Species counterpoint for multi-agent coordination. Not a metaphor — an isomorphism.

In counterpoint, each voice pursues its own melodic line independently. The rules aren't about making the voices agree. They're about making the voices *complement*. Parallel fifths are forbidden because two voices moving in lockstep produce nothing that one voice couldn't produce alone. Contrary motion is preferred because when one voice goes up and the other goes down, the space between them creates harmonic tension and release — emergence.

This crate maps those rules directly to agent coordination:

- **Interval** = relationship between two agents' approaches. Perfect fifth = strong complement. Tritone = working at cross-purposes.
- **Motion** = how agents change relative to each other. Contrary = one escalates while the other de-escalates. Parallel = both do the same thing (wasteful).
- **Consonance** = whether the interaction is productive. Perfect = optimal alignment without duplication. Dissonant = conflict that needs resolution.
- **Quality score** = contrary fraction × 0.4 + consonance × 0.4 + no-parallel penalty × 0.2.

The `CounterpointSession` tracks multiple agents across time steps, recording pairwise motions and intervals. The quality score tells you whether your fleet is producing counterpoint (independent + complementary) or just noise (parallel + dissonant).

**The key insight**: good multi-agent systems don't coordinate through agreement. They coordinate through complementarity. The agents don't need to know what each other is doing. They just need to move in ways that create productive intervals. The harmony emerges from the independence.

11 tests: interval classification, consonance levels, motion types, session quality, parallel fifth detection, three-voice sessions.

Part of [SuperInstance](https://github.com/SuperInstance/SuperInstance). Connects [agent-sync](https://github.com/SuperInstance/agent-sync) (timing), [agent-orchestration](https://github.com/SuperInstance/agent-orchestration) (dynamics), [agent-voice-leading](https://github.com/SuperInstance/agent-voice-leading) (transitions).

License: MIT
