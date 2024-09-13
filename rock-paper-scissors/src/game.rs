use std::cmp::Ordering;

use crate::{
    choice::Choice, 
    mode::Mode,
    util::mreadln
};

pub struct Game
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
    pub fn new(mode: Mode, max_points: u32) -> Game
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

    pub fn play(&mut self)
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