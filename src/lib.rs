#[derive(Debug, Clone)]
pub enum Algorithms {
    Astar,
    FloodFill,
    DFS,
    BFS,
}

#[derive(Debug, Clone)]
pub enum Message {
    Run,
    Pause,
    Stop,
    SetSpeed(f64),
    ChangeAlgorithm(Algorithms),
}

pub struct State {
    cur_algorithm: Algorithms,   
}

impl Default for State {
    fn default() -> State {
        State {
            cur_algorithm: Algorithms::Astar,
        }
    }
}
