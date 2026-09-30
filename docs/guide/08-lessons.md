# 8. Lessons

A **lesson** is a presentation with a **timeline**: a sequence of scenes and beats that narrate, run the simulation, pause it, show and highlight things, and give the learner moments to explore. The timeline directs the simulation (play, pause, seek, change); the model never depends on it (D-009).

## Timeline structure

```prismal
timeline {
  scene name {
    beat name { actions }
    beat name { actions }
  }
  scene name { ... }
}
```

Beats play one after another. Within a beat, the actions that direct the run (`run`, `hold`, `seek`, `reset`, `branch`, `intervene`, `request`) apply first, in written order, at the beat's start; then the other actions start together, and the beat ends when the last of them ends (D-033). `sequence { ... }` runs actions one after another inside a beat.

## Actions

| Action | Effect | Duration |
|---|---|---|
| `narrate "text" for 3 s` | a caption (and narration) | the given time, or a reading time |
| `run rate r` | simulation time advances at `r` seconds per presentation second (`0.25` is slow motion) | none |
| `run rate r until E` | as `run rate r`, and the beat waits until event `E` occurs | until `E` |
| `hold` | the simulation stops advancing; the picture stays | none |
| `seek τ` | shows simulation instant `τ` (`seek t0` goes back to the start) | none |
| `reset` | a new run from the start | none |
| `highlight name` | marks a named representation until the beat ends | none |
| `hide name` | stops showing a named representation | none |
| `in view { reps }` | adds representations to a view from now on | none |
| `show rep` | adds a representation over the presentation (for example a formula) | none |
| `intervene { set p = value }` | changes a parameter at the instant shown | none |
| `request E` | makes an `on request` event of the model happen now (D-027) | none |
| `wait 2 s` | waits | the given time |
| `explore limit L keep p { controls } fallback { actions }` | the learner's turn (below) | until the learner continues, or `L` |

The simulation keeps the rate set last: a beat without `run` or `hold` continues at the previous rate.

## Requested events

A lesson can only change the model in ways the model offers. A model offers an action with an event triggered `on request`:

```prismal
event drop on request {
  set pos = origin + (0 m, h)
  set vel = 0
  set airborne = true
}
```

`request drop` in a beat makes it happen at the instant shown, as if the learner had pressed a button. The handler is ordinary model code: it is checked, validated and logged like any event.

## Explore beats

```prismal
beat choose {
  hold
  explore limit 30 s keep h {
    slider(h, range: [1 m, 50 m])
  } fallback {
    sequence { intervene { set h = 20 m }; wait 2 s }
  }
}
```

- During an explore beat the learner may use the listed controls, and only those (D-025). Outside explore beats the learner controls the timeline and the view, never the model.
- The explore beat runs on a **branch** of the lesson's run: the learner's changes do not rewrite the lesson. `keep h` carries the learner's `h` into the rest of the lesson; without `keep`, the lesson returns to its own run where the beat started.
- The beat ends when the learner continues, or after `limit`.
- `fallback { ... }` is what plays instead in media without interaction (a video export, PK-9.10).

## A program

```text
space Plane = euclidean(2)

model FreeFall in Plane {
  param {
    g: Acceleration = 9.81 m/s^2
    h: Length       = 10 m     in [1 m, 50 m]
  }
  state {
    pos: Point            = origin + (0 m, h)
    vel: Vector<Velocity> = 0
  }
  discrete {
    airborne: Boolean = true
  }
  flow {
    der(pos) = if airborne then vel else 0
    der(vel) = if airborne then (0, -g) else 0
  }
  event landed on falling(pos.y) {
    set airborne = false
    set vel = 0
  }
  event drop on request {
    set pos = origin + (0 m, h)
    set vel = 0
    set airborne = true
  }
}
```

```text
presentation DropLesson for FreeFall {
  view scene: spatial(Plane, scale: 1 m -> 12 px, y: up) {
    axes
    marker(pos) as ball
  }
  permit learner { timeline_controls }
  observe {
    fall_times = elapsed on landed
  }

  timeline {
    scene first_drop {
      beat intro { narrate "A ball is dropped from 10 metres." for 3 s }
      beat fall  { run rate 1 until landed }
      beat look  {
        hold
        highlight ball
        narrate "How long did the fall take?" for 3 s
      }
      beat slow  {
        seek t0
        show formula("t", sqrt(2 * h / g), live: true)
        run rate 0.25 until landed
      }
    }
    scene your_turn {
      beat choose {
        hold
        explore limit 30 s keep h {
          slider(h, range: [1 m, 50 m])
        } fallback {
          sequence { intervene { set h = 20 m }; wait 2 s }
        }
      }
      beat again   { request drop; run rate 1 until landed }
      beat compare { narrate "Twice the height does not give twice the time." for 3 s }
    }
  }
}
```

Timing, from the fall time `T = sqrt(2 h / g) = 1.42784 s` for 10 m:

| Beat | Starts | Ends |
|---|---|---|
| `intro` | 0 s | 3 s (narration) |
| `fall` | 3 s | `3 + T` (the landing) |
| `look` | `3 + T` | `6 + T` (narration; the run holds) |
| `slow` | `6 + T` | `6 + 5T`: back to the start, then the fall at a quarter of real speed takes `4T` |
| `choose` | `6 + 5T` | when the learner continues, or 30 s later |
| `again` | | the drop from the chosen height, `sqrt(2 h / g)` |
| `compare` | | 3 s later |

## Testing a lesson

A case can play a lesson with a **learner script**: inputs at presentation instants.

```cases
run with_learner of FreeFall with DropLesson {
  learner {
    at 15 s: set slider h = 20 m
    at 16 s: continue
  }
  expect {
    end of fall   == 4.42784312292706 s within 1e-8 s
    end of slow   == 13.1392156146353 s within 1e-8 s
    end of choose == 16 s within 1e-9 s
    end of again  == 18.0192751093846 s within 1e-8 s
    fall_times[1] == 1.42784312292706 s within 1e-8 s
    fall_times[2] == 3.44711823231167 s within 1e-8 s    // T + sqrt(2 * 20 m / g)
  }
}

run without_learner of FreeFall with DropLesson {
  expect {
    end of choose == 43.1392156146353 s within 1e-8 s    // the 30 s limit
    fall_times[2] == 2.85568624585413 s within 1e-8 s    // dropped again from 10 m
  }
}
```

- `at 15 s: set slider h = 20 m` moves the slider for `h` at presentation time 15 s; `at 16 s: continue` ends the explore beat.
- `end of fall` and `start of fall` are beat times in presentation seconds.
- An input outside an explore beat is refused (the model is not the learner's to change then), and a case can check that the lesson is unaffected.
- The observation `fall_times` reads the lesson's run as it is at the end: the first landing, and the landing after the second drop. `elapsed` is simulation time since `t0`; the second drop happens at simulation time `T`, where the run was held.

In the player, a lesson has play, pause, a time scrubber and a bar of beats; during `choose` the slider appears and a Continue button ends the beat. Choosing the video medium plays the fallback instead.

## Animation

Three actions animate the presentation itself, never the model (D-042):

| Action | Effect | Duration |
|---|---|---|
| `reveal fade [for d] [in view] { reps }` | shows representations, their opacity rising from 0 to 1 | `d`, 1 s by default |
| `reveal draw [for d] [in view] { reps }` | shows lines and paths drawn from start to end (points fade) | `d`, 1 s by default |
| `hide name for d` | fades a representation out | `d` |
| `camera view [to P] [zoom z] [for d]` | moves a spatial view's camera to centre on the point `P` and magnify `z` times | `d`, 1 s by default |

The camera's point is evaluated at every frame: `camera scene to pos` keeps following the ball after the move. `zoom 1` returns to the view's own scale. Animations are functions of presentation time, so a lesson looks the same every time it plays and in a video export.

```text
presentation DropMovie for FreeFall {
  view scene: spatial(Plane, scale: 1 m -> 12 px, y: up) {
    axes
  }
  timeline {
    scene only {
      beat appear   { reveal fade in scene { marker(pos) as ball } }
      beat ground   { reveal draw for 2 s in scene { polyline(origin + (-5 m, 0 m), origin + (5 m, 0 m)) } }
      beat close_up { camera scene to pos zoom 2 for 1.5 s }
      beat fall     { run rate 0.5 until landed }
      beat away     { camera scene zoom 1 for 1 s; hide ball for 1 s }
    }
  }
}
```

```cases
run movie of FreeFall with DropMovie {
  expect {
    end of appear   == 1 s exactly
    end of close_up == 4.5 s exactly
    end of fall     == 7.35568624585413 s within 1e-8 s    // 4.5 s + T / 0.5
    end of away     == 8.35568624585413 s within 1e-8 s
  }
}
```

## Exporting a video

A lesson can be exported as a video file (D-052). The export opens the lesson in the video medium, so explore beats play their `fallback`, and draws every frame at a fixed rate:

```sh
cargo run --release -p prismal-media -- lesson.md DropMovie drop.mp4 --fps 30 --scale 2
```

- The first argument is the program (a source file, or a Markdown document whose `text` blocks form it); the second names the presentation.
- The output's extension chooses the format: `mp4`, `mov`, `mkv`, `webm` or `gif`. The encoding is done by ffmpeg, which must be installed (or named with `--encoder` or the `PRISMAL_FFMPEG` variable).
- `drop.png --at 4` writes the single frame at 4 s instead. Any other name is a directory: the frames as PNG images, the captions, and the ffmpeg command that makes a video of them.
- Narration appears as captions, drawn into the frames. `--captions track` puts them only in a subtitle track the viewer can turn off; `--captions both` does both. They are always written beside the video as `drop.vtt`. Narration has no sound yet.
- An explore beat without a `fallback` cannot be shown in a video: the export says so, and plays on.
- A presentation without a timeline is recorded as a run from its start; `--until 10` sets its length in simulation seconds.
- `--dark` uses the dark theme.

Since frames depend only on the program and presentation time, exporting twice gives the same video.

## A lab for the same model

The requested event also serves a presentation without a timeline: a `button` requests it whenever the learner presses it, at the instant shown.

```text
presentation DropLab for FreeFall {
  view scene: spatial(Plane, scale: 1 m -> 12 px, y: up) {
    axes
    marker(pos) as ball
    polyline(origin + (-2 m, 0 m), origin + (2 m, 0 m))
  }
  panel controls {
    slider(h, range: [1 m, 50 m])
    button(drop, label: "Drop again")
    table((pos.y, vel.y) every 0.25 s)
  }
  observe {
    drops = elapsed on drop
  }
}
```

- `button(drop, label: "Drop again")` requests the event `drop`, which must be declared `on request`. A button is an action of the learner like a slider: it takes effect at the instant shown and is logged as an intervention; undo removes it.
- `table((pos.y, vel.y) every 0.25 s)` lists the height and vertical speed every quarter second up to the instant shown, one column per component of the tuple, with the time first.
- `polyline(P, Q, ...)` draws a path through points (here the ground); `polygon` closes it.

Changing `h` with the slider and pressing the button drops the ball from the new height: the handler of `drop` reads `h` when it happens.

```cases
run lab_drops of FreeFall with DropLab {
  until t0 + 10 s
  expect { drops == [] }
}
```

A case plays the model without a learner, so the button is never pressed. The player's tests press it (`crates/prismal-present/tests/labs.rs`).

## Mistakes

```error
// error: SX-E03
space Plane = euclidean(2)
model M in Plane {
  state { pos: Point = origin }
}
presentation L for M {
  view scene: spatial(Plane, scale: 1 m -> 10 px, y: up) { marker(pos) as ball }
  timeline {
    scene s { beat b { highlight bal } }
  }
}
```

A highlight names a representation of the presentation; `bal` is not one.

## Exercises

1. Add a beat after `slow` that shows the velocity as an arrow from the ball (`in scene { arrow(vel, from: pos, scale: 1 m/s -> 5 px) }`) and replays the fall.
2. Make `choose` return to the lesson's own run (remove `keep h`). What does `again` drop from now? Write the case.
3. RP-08 in the reference programs is a complete narrated lesson with an explore beat, a requested relaunch, and cases A to D. Read it and predict `end of b8` for a learner who sets 30 degrees.

<details>
<summary>A solution to exercise 2</summary>

Without `keep h`, the lesson returns to its own run, where `h` is 10 m, so the second drop is again from 10 m whatever the learner chose:

```text
space Plane = euclidean(2)

model FreeFall in Plane {
  param {
    g: Acceleration = 9.81 m/s^2
    h: Length       = 10 m     in [1 m, 50 m]
  }
  state {
    pos: Point            = origin + (0 m, h)
    vel: Vector<Velocity> = 0
  }
  discrete { airborne: Boolean = true }
  flow {
    der(pos) = if airborne then vel else 0
    der(vel) = if airborne then (0, -g) else 0
  }
  event landed on falling(pos.y) { set airborne = false; set vel = 0 }
  event drop on request {
    set pos = origin + (0 m, h)
    set vel = 0
    set airborne = true
  }
}

presentation NoKeep for FreeFall {
  view scene: spatial(Plane, scale: 1 m -> 12 px, y: up) { marker(pos) as ball }
  observe { fall_times = elapsed on landed }
  timeline {
    scene only {
      beat fall   { run rate 1 until landed }
      beat choose {
        hold
        explore limit 30 s { slider(h, range: [1 m, 50 m]) }
      }
      beat again  { request drop; run rate 1 until landed }
    }
  }
}
```

```cases
run learner_ignored of FreeFall with NoKeep {
  learner {
    at 2 s: set slider h = 20 m
    at 3 s: continue
  }
  expect {
    end of again  == 4.42784312292706 s within 1e-8 s   // 3 s + T
    fall_times[2] == 2.85568624585413 s within 1e-8 s   // 2T: from 10 m again
  }
}
```

</details>
