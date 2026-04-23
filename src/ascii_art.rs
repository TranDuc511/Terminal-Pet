// ascii_art.rs — Multi-frame ASCII art for the cat in each pet state.
//
// Each state has 2–3 animation frames. The frames are static string slices
// for zero-cost storage. The `get_frames` function returns the slice for a
// given state; `current_frame` picks the frame based on an animation counter.

use crate::pet::PetState;

// ─── Idle frames (tail wag) ────────────────────────────────────────────────

const IDLE_1: &str = r#"
  /\_____/\
 /  o   o  \
( ==  v  == )
 )         (
(           )
 \         /
  )       (
 /|       |\
(_|       |_)
    |   |
   (_) (_)
"#;

const IDLE_2: &str = r#"
  /\_____/\
 /  o   o  \
( ==  ^  == )
 )         (
(           )
 \         /
  )       (
 /|       |\
(_|       |_)
    |   |
   (__)(_)
"#;

// ─── Eating frames ────────────────────────────────────────────────────────

const EAT_1: &str = r#"
  /\_____/\
 /  -   -  \
( ==  .  == ) nom!
 )         (
(           )
 \         /
  )       (
 /|       |\
(_|       |_)
    |   |
   (_) (_)    🍖
"#;

const EAT_2: &str = r#"
  /\_____/\
 /  ^   ^  \
( ==  w  == ) yum!
 )         (
(           )
 \         /
  )       (
 /|       |\
(_|       |_)
    |   |
   (_) (_)    🍖
"#;

// ─── Playing frames ───────────────────────────────────────────────────────

const PLAY_1: &str = r#"
  /\_____/\
 /  >   <  \
( ==  ∆  == )
 )         (\
(           )~  🧶
 \         /
  )       (
 /|       |\
(_|  /\   |_)
    |  |
   (_) (_)
"#;

const PLAY_2: &str = r#"
  /\_____/\
 /  ^   ^  \
( ==  ω  == ) !!
 )         (/
(           )  ~🧶
 \         /
  )       (
 /|       |\
(_|\__/   |_)
    |   |
   (_) (_)
"#;

const PLAY_3: &str = r#"
  /\_____/\
 /  ◕   ◕  \
( ==  ᴥ  == )
 )         (
(\          )  🧶
 \         /~
  )       (
 /|       |\
(_|       |_)
    |   |
   (_) (_)
"#;

// ─── Being patted frames ──────────────────────────────────────────────────

const PAT_1: &str = r#"
  /\_____/\
 /  ^   ^  \
( ==  ♥  == ) purrr~
 )         (
(           )
 \         /
  )~      (
 /|       |\
(_|       |_)
    |   |
   (_) (_)  ♥
"#;

const PAT_2: &str = r#"
  /\_____/\
 /  ◡   ◡  \
( ==  ♥  == ) ♫ purr ♫
 )         (
(           )
 \         /
  )       (~
 /|       |\
(_|       |_)
    |   |
   (_) (_)  ♥♥
"#;

// ─── Sleeping frames ──────────────────────────────────────────────────────

const SLEEP_1: &str = r#"
  /\_____/\
 /  -   -  \
( ==  .  == ) z
 )         (
(           )
 \_________/
  |         |
  |_________|
    |   |         Z
   (_) (_)      z
"#;

const SLEEP_2: &str = r#"
  /\_____/\
 /  ~   ~  \
( ==  .  == )  z
 )         (
(           )
 \_________/
  |         |
  |_________|
    |   |      Z
   (_) (_)        z
"#;

// ─── Sad frame ────────────────────────────────────────────────────────────

const SAD_1: &str = r#"
  /\_____/\
 /  ;   ;  \
( ==  ▽  == )
 )         (
(           )
 \         /
  )       (
 /|       |\
(_|       |_)
    |   |
   (_) (_)  ...
"#;

const SAD_2: &str = r#"
  /\_____/\
 /  T   T  \
( ==  ︿  == )
 )         (
(           )
 \         /
  )       (
 /|       |\
(_|       |_)
    |   |    💧
   (_) (_)
"#;

// ─── Frame dispatch ───────────────────────────────────────────────────────

/// Return all animation frames for the given pet state.
pub fn get_frames(state: PetState) -> &'static [&'static str] {
    match state {
        PetState::Idle      => &[IDLE_1, IDLE_2],
        PetState::Eating    => &[EAT_1, EAT_2],
        PetState::Playing   => &[PLAY_1, PLAY_2, PLAY_3],
        PetState::BeingPatted => &[PAT_1, PAT_2],
        PetState::Sleeping  => &[SLEEP_1, SLEEP_2],
        PetState::Sad       => &[SAD_1, SAD_2],
    }
}

/// Pick the current frame given a rolling animation tick counter.
pub fn current_frame(state: PetState, tick: u64) -> &'static str {
    let frames = get_frames(state);
    let idx = (tick as usize) % frames.len();
    frames[idx]
}
