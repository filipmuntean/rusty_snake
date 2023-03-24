use crate::snake::Snake;
use crate::point::Point;
use crate::direction::Direction;

use std::io::Stdout;
use std::time::{Duration, Instant};
use crossterm::terminal::size;

use crate::command::Command
user rand::Rng;

const MAX_INTERVAL : u16 = 700;
const MIN_INTERVAL : u16 = 200;
const MAX_SPEED : u16 = 20;
