//! > Rock Paper Scissors
//! # Guide
//! firstly run this crate via cargo:
//! ```sh
//! cargo run --package rock-paper-scissors
//! ```
//! then it will prompt you for play mode, whether singleplayer or multiplayer:
//! ```sh
//! Rock Paper Scissors by Ahmad
//! choose play mode:
//! 1. singleplayer
//! 2. multiplayer
//! input: 
//! ```
//! Type `1` for singleplayer or `2` for multiplayer.
//! If you chose multiplayer it will prompt you for player names:
//! ```sh
//! Enter first player name:
//! Enter second player name:
//! ```
//! then it will prompt you for max points number:
//! ```sh
//! Enter max points number: 
//! ```
//! Supposing you typed `3` it will continue to run the game & prompt each turn for player's choice until someone gets `3` points.
//! this are valid choices:
//! | Choice | Alias |
//! | -------- | ------- |
//! | `rock` | `r` |
//! | `paper` | `p` |
//! | `scissors` | `s` |

pub mod choice;
pub mod game;
pub mod mode;
pub mod util;
