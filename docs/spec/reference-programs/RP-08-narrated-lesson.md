# RP-08 Narrated projectile lesson

## Purpose

A short narrated lesson on the RP-01 projectile: watch a launch, stop at the landing, replay it slowly, then let the learner choose an angle and see the result. The program checks the explanation timeline (D-009, PK section 9): waiting on model events, holding and seeking the simulation, deterministic presentation timing, an explore beat with and without `keep` (D-025), learner restrictions outside explore beats, captions (D-026), and the linear-media fallback (PK-9.10, PK-12.3).

It uses D-027: the model exposes a requestable `relaunch` event that the timeline and the learner can trigger.

## Model

The RP-01 model with one addition (D-027):

```text
  event relaunch on request {
    set pos    = origin
    set vel    = speed * (cos(angle), sin(angle))
    set flying = true
  }
```

## Presentation and timeline

**working syntax (D-028, non-binding)**

```text
presentation ProjectileLesson for Projectile {
  view scene: spatial(Plane, scale: 1 m -> 10 px, y: up) {
    axes
  }
  permit learner { timeline_controls; zoom; pan }
  observe {
    landings = (elapsed, pos.x) on landed
    log      = event_log
  }

  timeline {
    scene launch {
      beat b1 {
        in scene { marker(pos) as ball; arrow(vel, from: pos) }
        narrate "A ball is launched at 45 degrees." for 4 s
      }
      beat b2 { run rate 1 until landed }
      beat b3 { hold; highlight ball; narrate "It lands here. Why this distance?" for 3 s }
      beat b4 {
        seek t0                                  // applies first (PK-9.2a, D-033)
        show formula("R", speed^2 * sin(2 * angle) / g, live: true)
        narrate "Watch the horizontal speed." for 3 s
      }
      beat b5 {
        in scene { arrow((vel.x, 0), from: pos) }  // horizontal component as a vector
        run rate 0.5 until landed
      }
    }
    scene try_it {
      beat b6 { hold; narrate "Choose your own angle." for 3 s }
      beat b7 {
        explore limit 60 s keep angle {
          slider(angle, range: [10 deg, 80 deg])
        } fallback {
          sequence { intervene { set angle = 60 deg }; wait 3 s }
        }
      }
      beat b8 { request relaunch; run rate 1 until landed }
      beat b9 { narrate "Compare the distance with the first launch." for 3 s }
    }
  }
}
```

Narration durations are given explicitly so that timing is checkable. In a real lesson they come from audio or reading time.

## Cases

| Case | Medium | Learner script | Change |
|---|---|---|---|
| `A-keep` | interactive | at 23.0 s set the slider to `60 deg`; at 25.0 s continue | none |
| `B-no-keep` | interactive | as A | `b7` without `keep(angle)` |
| `C-video` | video export | none | none: `b7` uses its fallback |
| `D-restricted` | interactive | at 10.0 s try to set `angle` to `30 deg` (outside an explore beat) | as A otherwise |

## Expected results

Exact values used: flight time at 45 degrees `T45 = 2.88320807823261 s`, at 60 degrees `T60 = 3.53119430697019 s`; ranges `R45 = 40.7747196738022 m`, `R60 = 35.3119430697019 m` (RP-01).

**Presentation timing.** A beat that runs the simulation until `landed` at rate `r` lasts `T / r` of presentation time (PK-9.3). Event instants are located within `ε_t` (RC-7.6), so presentation instants are checked to `1e-8 s`.

### Case A-keep

| ID | Observation | Expected | Tolerance | Kind | Status |
|---|---|---|---|---|---|
| RP-08.E1 | start of `b2`, `b3`, `b4`, `b5` (presentation) | 4 s, 6.88320807823261 s, 9.88320807823261 s, 12.8832080782326 s | abs 1e-8 s | analytic | fixed |
| RP-08.E2 | end of `b5` | 18.6496242346978 s (`12.883... + 2 T45`) | abs 1e-8 s | analytic | fixed |
| RP-08.E3 | `landings` in `b2` and in `b5` | both `(T45, R45)`, and bit-identical to each other (RC-12.2) | exact between the two | behavior | fixed |
| RP-08.E4 | end of `b7` | 25.0 s (the learner's continue) | exact | behavior | fixed |
| RP-08.E5 | `b8` | runs on the learner's branch with `angle = 60 deg`; landing at elapsed `T45 + T60` = 6.41440238520280 s, range `R60` = 35.3119430697019 m | abs 1e-8 | analytic | fixed |
| RP-08.E6 | end of `b8`, end of lesson | 28.5311943069702 s, 31.5311943069702 s | abs 1e-8 s | analytic | fixed |
| RP-08.E7 | lesson run's own log | unchanged by the learner: it contains no intervention on `angle` (PK-9.8) | exact | behavior | fixed |
| RP-08.E8 | captions | every `narrate` has a caption shown for its duration; `landed` is announced to assistive technology each time it occurs (PK-11.3) | exact | behavior | fixed |

### Case B-no-keep

| ID | Observation | Expected | Tolerance | Kind | Status |
|---|---|---|---|---|---|
| RP-08.E9 | `b8` | runs on the lesson run, `angle = 45 deg`: landing at elapsed `2 T45` = 5.76641615646522 s, range `R45` | abs 1e-8 | analytic | fixed |
| RP-08.E10 | end of `b8`, end of lesson | 27.8832080782326 s, 30.8832080782326 s | abs 1e-8 s | analytic | fixed |
| RP-08.E11 | the learner's branch | kept as a separate run with its own log (RC-12.3), not used by later beats | exact | behavior | fixed |

### Case C-video

| ID | Observation | Expected | Tolerance | Kind | Status |
|---|---|---|---|---|---|
| RP-08.E12 | `b7` | replaced by its fallback: the angle is set to 60 degrees on the lesson run and the beat lasts 3 s | exact | behavior | fixed |
| RP-08.E13 | end of `b7`, `b8`, lesson | 24.6496242346978 s, 28.1808185416680 s, 31.1808185416680 s | abs 1e-8 s | analytic | fixed |
| RP-08.E14 | two exports of the same lesson | identical sequences of frame descriptions (PK-8.7, PK-12.1) | exact | behavior | fixed |
| RP-08.E15 | export | no element dropped silently: the export reports no unsupported element without a fallback (PK-12.3) | exact | behavior | fixed |

### Case D-restricted

| ID | Observation | Expected | Kind | Status |
|---|---|---|---|---|
| RP-08.E16 | the attempt at 10.0 s | refused by the presentation: outside an explore beat the learner has no model actions (PK-9.8); the lesson run is unaffected; timing as in case A | behavior | fixed |

## History

- 2026-09-29 written. D-027 accepted the same day.
- 2026-09-30 programs rewritten in the working syntax (D-028); corrections from the syntax study applied: `b5` shows the horizontal velocity as the vector `(vel.x, 0)` (an arrow needs a vector, PK-6.3); `b4` shows a `formula` with display symbols (D-034); the order of `seek` and `request` within a beat follows D-033; representation names (`ball`) per PK-6.1.
