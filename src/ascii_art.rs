// ascii_art.rs — Multi-frame ASCII art for the cat in each pet state.
//
// Each state has 2–3 animation frames. The frames are static string slices
// for zero-cost storage. The `get_frames` function returns the slice for a
// given state; `current_frame` picks the frame based on an animation counter.

use crate::pet::PetState;

// ─── Idle frames (tail flick) ───────────────────────────────────────────────

const IDLE_1: &str = r#"
      /\_/\
     ( o.o )
      > ^ <
     /     \
    /       \
   /   | |   \
  (___(|_|_)__)~
"#;

const IDLE_2: &str = r#"
      /\_/\
     ( o.o )
      > ^ <
     /     \   _
    /       \ ( \
   /   | |   \ ) )
  (___(|_|_)__)_/
"#;

// ─── Eating frames ────────────────────────────────────────────────────────

const EAT_1: &str = r#"
      /\_/\
     ( >.< )
      > ^ <
     /     \
    /       \
   /   | |   \
  (___(|_|_)__)   🐟
"#;

const EAT_2: &str = r#"
      /\_/\
     ( ^.^ )
      > v <
     /     \
    /       \
   /   | |   \
  (___(|_|_)__)  🐟
"#;

// ─── Playing frames ───────────────────────────────────────────────────────

const PLAY_1: &str = r#"
      /\_/\
     ( O.O )
      > ^ <
     /     \
    /  / \  \
   /  |   |  \
  (___)___(___)   🧶
"#;

const PLAY_2: &str = r#"
      /\_/\
     ( >.< )
      > ^ <
     /     \
    /  \ /  \
   /    |    \
  (____( )____)    🧶
"#;

const PLAY_3: &str = r#"
      /\_/\
     ( ^.^ )
      > ^ <
     /     \
    /       \
   /   | |   \
  (___(|_|_)__)      🧶
"#;

// ─── Being patted frames ──────────────────────────────────────────────────

const PAT_1: &str = r#"
      /\_/\
     ( -.- )
      > ^ <
     /     \
    /       \
   /   | |   \
  (___(|_|_)__)~  ❤️
"#;

const PAT_2: &str = r#"
      /\_/\
     ( v.v )
      > ^ <
     /     \
    /       \
   /   | |   \
  (___(|_|_)__)~    ❤️❤️
"#;

// ─── Sleeping frames ──────────────────────────────────────────────────────

const SLEEP_1: &str = r#"
      /\_/\
     ( -.- )
      > ^ <
     /     \
    /       \
   /   | |   \
  (___(|_|_)__)~  zZ
"#;

const SLEEP_2: &str = r#"
      /\_/\
     ( -.- )
      > ^ <
     /     \
    /       \
   /   | |   \
  (___(|_|_)__)~    zZ
"#;

// ─── Sad frame ────────────────────────────────────────────────────────────

const SAD_1: &str = r#"
      /\_/\
     ( ;.; )
      > ^ <
     /     \
    /       \
   /   | |   \
  (___(|_|_)__)~
"#;

const SAD_2: &str = r#"
      /\_/\
     ( T.T )
      > ^ <
     /     \
    /       \
   /   | |   \
  (___(|_|_)__)~  💧
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
