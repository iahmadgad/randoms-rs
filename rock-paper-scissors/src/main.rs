pub use rock_paper_scissors::{choice::Choice, game::Game, mode::Mode, util::mreadln};

fn main() {
    let mode: String = mreadln("\x1b[01;97mRock Paper Scissors \x1b[0;92mby \x1b[01;34mAhmad\n\x1b[0;33mchoose play mode:\x1b[0m\n1. singleplayer\n2. multiplayer\n\x1b[0;33minput:\x1b[0m ");
    let mode = match mode.as_str() {
        "1" => Mode::Singleplayer,
        "2" => {
            let (player1, player2) = (
                mreadln("\x1b[0;33mEnter first player name:\x1b[0m "),
                mreadln("\x1b[0;33mEnter second player name:\x1b[0m "),
            );
            Mode::Multiplayer(player1, player2)
        }
        x => panic!("{x} is not a valid mode"),
    };
    let max: u32 = mreadln("\x1b[0;33mEnter max points number:\x1b[0m ")
        .parse()
        .unwrap();
    let mut game = Game::new(mode, max);
    game.play();
}
