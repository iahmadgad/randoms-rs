use {
    rand::Rng,
    std::{
	io::{
	    self,
	    Write
	},
	cmp::Ordering
    }
};

fn main()
{
    let mode: String = mreadln("\x1b[01;97mRock Paper Scissors \x1b[0;92mby \x1b[01;34mAhmad\n\x1b[0;33mchoose play mode:\x1b[0m\n1. singleplayer\n2. multiplayer\n\x1b[0;33minput:\x1b[0m ");
    let mode = match mode.as_str()
    {
	"1" => Mode::Singleplayer,
	"2" =>
	{
	    let (player1, player2) = (mreadln("\x1b[0;33mEnter first player name:\x1b[0m "), mreadln("\x1b[0;33mEnter second player name:\x1b[0m "));
	    Mode::Multiplayer(player1, player2)
	},
	x => panic!("{x} is not a valid mode")
    };
    let max: u32 = mreadln("\x1b[0;33mEnter max points number:\x1b[0m ").parse().unwrap();
    let mut game = Game::new(mode, max);
    game.play();
}

struct Game
{
    mode: Mode,
    points1: u32,
    points2: u32,
    max_points: u32,
    last_choice1: Choice,
    last_choice2: Choice
}

impl Game
{
    fn new(mode: Mode, max_points: u32) -> Game
    {
	Game
	{
	    mode,
	    points1: 0,
	    points2: 0,
	    max_points,
	    last_choice1: Choice::Rock,
	    last_choice2: Choice::Rock
	}
    }

    fn play(&mut self)
    {
	while self.points1 < self.max_points && self.points2 < self.max_points
	{
	    match &self.mode
	    {
		Mode::Singleplayer =>
		{
		    self.last_choice1 = Choice::parse(&mreadln("\x1b[0;33mplayer:\x1b[0m "));
		    self.last_choice2 = Choice::random();
		    println!("\x1b[0;34mcomputer:\x1b[0m {:?}", self.last_choice2);
		},
		Mode::Multiplayer(p1, p2) =>
		{
		    self.last_choice1 = Choice::parse(&mreadln(format!("\x1b[0;34m{p1}:\x1b[8m ").as_str()));
		    self.last_choice2 = Choice::parse(&mreadln(format!("\x1b[0;31m{p2}:\x1b[8m ").as_str()));
		}
	    }
	    self.update_points();
	    println!("{:?} {:?}", self.last_choice1, self.last_choice2);
	}
	self.print_result();
    }

    fn update_points(&mut self)
    {
	match (&self.last_choice1, &self.last_choice2)
	{
	     (a, b) if a.defeats(b) => self.points1 += 1,
	    (a, b) if b.defeats(a) => self.points2 += 1,
	    _ => ()
	}
    }
    
    fn get_result(&self) -> u32
    {
	match self.points1.cmp(&self.points2)
	{
	    Ordering::Greater => 1,
	    Ordering::Less => 2,
	    _ => 0
	}
    }

    fn print_result(&self)
    {
	match self.points1.cmp(&self.points2)
	{
	    Ordering::Greater => match &self.mode {
		Mode::Singleplayer => println!("\x1b[0;33mPlayer \x1b[0;97mwins!\x1b[0m"),
		Mode::Multiplayer(p1, _) => println!("\x1b[0;34m{p1} \x1b[97mwins!\x1b[0m")
	    }
	    Ordering::Less => match &self.mode {
		Mode::Singleplayer => println!("\x1b[0;34mComputer \x1b[0;97mwins!\x1b[0m"),
		Mode::Multiplayer(_, p2) => println!("\x1b[0;31m{p2} \x1b[0;97mwins!\x1b[0m")
	    },
	    _ => println!("\x1b[0;97mTie!\x1b[0m")
	}
    }

}

#[derive(Debug)]
enum Choice
{
    Rock,
    Paper,
    Scissors
}

impl Choice
{
    fn random() -> Choice
    {
	let r = rand::thread_rng().gen_range(0..3);
	match r
	{
	    0 => Choice::Rock,
	    1 => Choice::Paper,
	    2 => Choice::Scissors,
	    _ => unreachable!()
	}
    }

    fn defeats(&self, other: &Choice) -> bool
    {
	match (self, other) {
	    (Choice::Rock, Choice::Scissors) | (Choice::Paper, Choice::Rock) | (Choice::Scissors, Choice::Paper) => true,
	    _ => false
	}
    }

    fn parse(choice: &str) -> Choice
    {
	match choice
	{
	    "r" | "rock" => Choice::Rock,
	    "p" | "paper" => Choice::Paper,
	    "s" | "scissors" => Choice::Scissors,
		x => panic!("{x} is not a valid choice")
	}
    }
}


enum Mode
{
    Singleplayer,
    Multiplayer(String, String)
}

fn mreadln(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read from stdin");
    print!("\x1b[0m");
    input.trim().to_string()
}

