use std::{
    fmt::Display,
    ops::{Index, IndexMut},
    time::Duration,
};

pub use Square::*;
pub use Turn::*;

// Engine model

pub trait Engine {
    fn get_info(&self) -> EngineInfo;
    fn get_search_result(&self, request: Request) -> SearchResult;

    fn get_move(&self, request: Request) -> Move {
        let SearchResult {
            mv,
            eval: _,
            pv: _,
            depth_searched: _,
            time_spent: _,
        } = self.get_search_result(request);
        mv
    }
}

// Board models

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Blocks {
    #[default]
    B0,
    B1,
    B2,
    B3,
    B4,
}
impl Blocks {
    pub fn is_reachable(&self, other: &Blocks) -> bool {
        match (self, other) {
            (Blocks::B3, _) => panic!("Can't start movement from 3 blocks high, because the mover already won!"),
            (Blocks::B4, _) => panic!("Can't start movement from 4 blocks high!"),
            (Blocks::B0, Blocks::B2) => false,
            (Blocks::B0, Blocks::B3) => false,
            (Blocks::B1, Blocks::B3) => false,
            (_, Blocks::B4) => false,
            (_, _) => true,
        }
    }
}
impl Into<usize> for Blocks {
    fn into(self) -> usize {
        match self {
            Blocks::B0 => 0,
            Blocks::B1 => 1,
            Blocks::B2 => 2,
            Blocks::B3 => 3,
            Blocks::B4 => 4,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Board {
    blocks: [Blocks; 25],
    workers: [Option<Worker>; 25],
    turn: Turn,
    victory: Option<Turn>,
}
impl Board {
    pub fn new(workers_p1: [Square ; 2], workers_p2: [Square ; 2]) -> Self {
        assert_ne!(workers_p1[0], workers_p1[1], "Can't have two workers in the same square {}!", workers_p1[0]);
        assert_ne!(workers_p1[0], workers_p2[0], "Can't have two workers in the same square {}!", workers_p1[0]);
        assert_ne!(workers_p1[0], workers_p2[1], "Can't have two workers in the same square {}!", workers_p1[0]);
        assert_ne!(workers_p1[1], workers_p2[0], "Can't have two workers in the same square {}!", workers_p1[1]);
        assert_ne!(workers_p1[1], workers_p2[1], "Can't have two workers in the same square {}!", workers_p1[1]);
        assert_ne!(workers_p2[0], workers_p2[1], "Can't have two workers in the same square {}!", workers_p2[0]);

        let mut workers: [Option<Worker>; 25] = Default::default();
        workers[workers_p1[0]] = Some(Worker { turn: Turn::P1 });
        workers[workers_p1[1]] = Some(Worker { turn: Turn::P1 });
        workers[workers_p2[0]] = Some(Worker { turn: Turn::P2 });
        workers[workers_p2[1]] = Some(Worker { turn: Turn::P2 });

        Board {
            blocks: Default::default(),
            workers,
            turn: Default::default(),
            victory: Default::default(),
        }
    }

    pub fn apply_move(&mut self, mv: Move) {
        let Move {
            from,
            to,
            at,
        } = mv;

        self.move_worker(from, to);

        self.check_normal_victory();
        if self.victory.is_some() {
            return;
        }

        self.build(at.unwrap());
        self.next_turn();

        self.check_smother_victory();
    }
    pub fn get_request<'a>(&'a self, time_left: Duration) -> Request<'a> {
        Request {
            blocks: &self.blocks,
            workers: &self.workers,
            turn: &self.turn,
            time_left,
        }
    }

    fn build(&mut self, at: Square) {
        assert!(self.workers[at].is_none(), "Can't build over worker at {}!", at);

        self.blocks[at] = match self.blocks[at] {
            Blocks::B0 => Blocks::B1,
            Blocks::B1 => Blocks::B2,
            Blocks::B2 => Blocks::B3,
            Blocks::B3 => Blocks::B4,
            Blocks::B4 => panic!("Can't build over 4 blocks at {}!", at),
        };
    }
    fn check_normal_victory(&mut self) {
        for square in 0..self.workers.len() {
            if let Some(worker) = self.workers[square] {
                if self.blocks[square] == Blocks::B3 {
                self.victory = Some(worker.turn);
                return;
                }
            }
        }
    }
    fn check_smother_victory(&mut self) {
        for square in Square::squares() {
            if let Some(Worker { turn }) = self.workers[square] {
                if turn == self.turn {
                    for neighbour in square.get_neighbours() {
                        if self.blocks[square].is_reachable(&self.blocks[neighbour]) && self.workers[neighbour].is_none() {
                            return;
                        }
                    }
                }
            }
        }

        self.victory = Some(self.turn.next());
    }
    fn move_worker(&mut self, from: Square, to: Square) {
        assert!(self.workers[to].is_none(), "Can't move over another worker in {}!", to);
        assert!(self.workers[from].is_some(), "Can't move because there's no worker in {}!", from);
        assert!(from.get_neighbours().contains(&to),
            "Can't move from {} to {} because they aren't neighbours!", from, to);
        assert!(self.blocks[from].is_reachable(&self.blocks[to]),
            "Can't move from {} to {} because it's too high!", from, to);

        self.workers[to] = self.workers[from];
        self.workers[from] = None;
    }
    fn next_turn(&mut self) {
        self.turn = self.turn.next();
    }
}

#[derive(Debug)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub at: Option<Square>,
}

#[derive(Debug)]
pub struct Request<'a> {
    pub blocks: &'a [Blocks ; 25],
    pub workers: &'a [Option<Worker> ; 25],
    pub turn: &'a Turn,
    pub time_left: Duration,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Square {
    A1,
    A2,
    A3,
    A4,
    A5,
    B1,
    B2,
    B3,
    B4,
    B5,
    C1,
    C2,
    C3,
    C4,
    C5,
    D1,
    D2,
    D3,
    D4,
    D5,
    E1,
    E2,
    E3,
    E4,
    E5,
}
impl Square {
    pub fn get_neighbours(&self) -> Vec<Square> {
        match self {
            A1 => vec![A2, B1, B2],
            A2 => vec![A1, A3, B1, B2, B3],
            A3 => vec![A2, A4, B2, B3, B4],
            A4 => vec![A3, A5, B3, B4, B5],
            A5 => vec![A4, B4, B5],
            B1 => vec![A1, A2, B2, C1, C2],
            B2 => vec![A1, A2, A3, B1, B3, C1, C2, C3],
            B3 => vec![A2, A3, A4, B2, B4, C2, C3, C4],
            B4 => vec![A3, A4, A5, B3, B5, C3, C4, C5],
            B5 => vec![A4, A5, B4, C4, C5],
            C1 => vec![B1, B2, C2, D1, D2],
            C2 => vec![B1, B2, B3, C1, C3, D1, D2, D3],
            C3 => vec![B2, B3, B4, C2, C4, D2, D3, D4],
            C4 => vec![B3, B4, B5, C3, C5, D3, D4, D5],
            C5 => vec![B4, B5, C4, D4, D5],
            D1 => vec![C1, C2, D2, E1, E2],
            D2 => vec![C1, C2, C3, D1, D3, E1, E2, E3],
            D3 => vec![C2, C3, C4, D2, D4, E2, E3, E4],
            D4 => vec![C3, C4, C5, D3, D5, E3, E4, E5],
            D5 => vec![C4, C5, D4, E4, E5],
            E1 => vec![D1, D2, E2],
            E2 => vec![D1, D2, D3, E1, E2],
            E3 => vec![D2, D3, D4, E2, E4],
            E4 => vec![D3, D4, D5, E3, E5],
            E5 => vec![D4, D5, E4],
        }
    }
    pub fn squares() -> Vec<Square> {
        vec![
            A1, A2, A3, A4, A5,
            B1, B2, B3, B4, B5,
            C1, C2, C3, C4, C5,
            D1, D2, D3, D4, D5,
            E1, E2, E3, E4, E5,
        ]
    }
}
impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            A1 => "A1",
            A2 => "A2",
            A3 => "A3",
            A4 => "A4",
            A5 => "A5",
            B1 => "B1",
            B2 => "B2",
            B3 => "B3",
            B4 => "B4",
            B5 => "B5",
            C1 => "C1",
            C2 => "C2",
            C3 => "C3",
            C4 => "C4",
            C5 => "C5",
            D1 => "D1",
            D2 => "D2",
            D3 => "D3",
            D4 => "D4",
            D5 => "D5",
            E1 => "E1",
            E2 => "E2",
            E3 => "E3",
            E4 => "E4",
            E5 => "E5",
        })
    }
}
impl<T> Index<Square> for [T] {
    type Output = T;

    fn index(&self, index: Square) -> &Self::Output {
        &self[Into::<usize>::into(index)]
    }
}
impl<T> IndexMut<Square> for [T] {
    fn index_mut(&mut self, index: Square) -> &mut Self::Output {
        &mut self[Into::<usize>::into(index)]
    }
}
impl<T> Index<Square> for Vec<T> {
    type Output = T;

    fn index(&self, index: Square) -> &Self::Output {
        &self[Into::<usize>::into(index)]
    }
}
impl<T> IndexMut<Square> for Vec<T> {
    fn index_mut(&mut self, index: Square) -> &mut Self::Output {
        &mut self[Into::<usize>::into(index)]
    }
}
impl Into<usize> for Square {
    fn into(self) -> usize {
        match self {
            Square::A1 => 0,
            Square::A2 => 1,
            Square::A3 => 2,
            Square::A4 => 3,
            Square::A5 => 4,
            Square::B1 => 5,
            Square::B2 => 6,
            Square::B3 => 7,
            Square::B4 => 8,
            Square::B5 => 9,
            Square::C1 => 10,
            Square::C2 => 11,
            Square::C3 => 12,
            Square::C4 => 13,
            Square::C5 => 14,
            Square::D1 => 15,
            Square::D2 => 16,
            Square::D3 => 17,
            Square::D4 => 18,
            Square::D5 => 19,
            Square::E1 => 20,
            Square::E2 => 21,
            Square::E3 => 22,
            Square::E4 => 23,
            Square::E5 => 24,
        }
    }
}
impl TryFrom<usize> for Square {
    type Error = String;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value < 25 {
            Ok(match value {
                0 => A1,
                1 => A2,
                2 => A3,
                3 => A4,
                4 => A5,
                5 => B1,
                6 => B2,
                7 => B3,
                8 => B4,
                9 => B5,
                10 => C1,
                11 => C2,
                12 => C3,
                13 => C4,
                14 => C5,
                15 => D1,
                16 => D2,
                17 => D3,
                18 => D4,
                19 => D5,
                20 => E1,
                21 => E2,
                22 => E3,
                23 => E4,
                24 => E5,
                _ => unreachable!(),
            })
        } else {
            Err(format!("{} is not a valid square! Range is 0 <= value <= 24.", value))
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Turn {
    #[default]
    P1,
    P2,
}
impl Turn {
    pub fn next(&self) -> Turn {
        match self {
            Turn::P1 => Turn::P2,
            Turn::P2 => Turn::P1,
        }
    }
}
impl Display for Turn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            P1 => "P1",
            P2 => "P2",
        })
    }
}
impl<T> Index<Turn> for [T] {
    type Output = T;

    fn index(&self, index: Turn) -> &Self::Output {
        &self[Into::<usize>::into(index)]
    }
}
impl<T> IndexMut<Turn> for [T] {
    fn index_mut(&mut self, index: Turn) -> &mut Self::Output {
        &mut self[Into::<usize>::into(index)]
    }
}
impl<T> Index<Turn> for Vec<T> {
    type Output = T;

    fn index(&self, index: Turn) -> &Self::Output {
        &self[Into::<usize>::into(index)]
    }
}
impl<T> IndexMut<Turn> for Vec<T> {
    fn index_mut(&mut self, index: Turn) -> &mut Self::Output {
        &mut self[Into::<usize>::into(index)]
    }
}
impl Into<usize> for Turn {
    fn into(self) -> usize {
        match self {
            Turn::P1 => 0,
            Turn::P2 => 1,
        }
    }
}
impl TryFrom<usize> for Turn {
    type Error = String;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value < 25 {
            Ok(match value {
                0 => P1,
                1 => P2,
                _ => unreachable!(),
            })
        } else {
            Err(format!("{} is not a valid turn! Range is 0 <= value <= 1.", value))
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Worker {
    pub turn: Turn,
}






pub struct EngineInfo {
    pub name: String,
    pub eval_range: (isize, isize),
}

#[derive(Debug)]
pub struct SearchResult{
    pub mv: Move,
    pub eval: Option<isize>,
    pub pv: Option<Vec<Move>>,
    pub depth_searched: Option<usize>,
    pub time_spent: Option<Duration>,
}

pub struct BenchmarkRequest{
    pub position: Board,
    pub max_depth: usize,
    pub debug: bool,
}