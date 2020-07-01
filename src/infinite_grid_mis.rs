use std::cmp::max;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};


fn get_cell_seq(x: i32, y: i32,t: u32) -> u64 {
    let mut hasher = DefaultHasher::new();
    x.hash(&mut hasher);
    y.hash(&mut hasher);
    t.hash(&mut hasher);
    return hasher.finish();
}

#[derive(Clone)]
#[derive(Copy)]
enum CellState{
    Waiting,
    Accept,
    Reject,
}

fn get_cell_at_memo(cache: &mut HashMap<(i32,i32,u32),CellState>, x: i32, y: i32, t: u32) -> CellState {
    fn memoize_val(cache: &mut HashMap<(i32,i32,u32),CellState>, x: i32, y: i32, t: u32, val: CellState) -> CellState {
        cache.insert((x,y,t),val);
        return val;
    }
    if let Some(state)=cache.get(&(x,y,t)){
        return state.clone();
    }
    if t == 0 {
        return CellState::Waiting;
    }
    match get_cell_at_memo(cache,x,y,t-1) {
        CellState::Waiting => {},
        CellState::Accept => return memoize_val(cache,x,y,t,CellState::Accept),
        CellState::Reject => return memoize_val(cache,x,y,t,CellState::Reject),
    }

    let mut score: u64 = 0;
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx==0 && dy==0 { continue; }
            match get_cell_at_memo(cache,x+dx,y+dy,t-1) {
                CellState::Waiting => score = max(score,get_cell_seq(x+dx,y+dy,t-1)),
                CellState::Accept => return memoize_val(cache,x,y,t,CellState::Reject),
                CellState::Reject => {},
            }
        }
    }
    if get_cell_seq(x,y,t-1)>score {
        return memoize_val(cache,x,y,t,CellState::Accept);
    } else {
        return memoize_val(cache,x,y,t,CellState::Waiting);
    }
}

fn get_cell_memo(cache: &mut HashMap<(i32,i32,u32),CellState>, x: i32, y: i32) -> bool {
    let mut t = 0;
    loop {
        match get_cell_at_memo(cache,x,y,t){
            CellState::Waiting => t+=1,
            CellState::Accept => return true,
            CellState::Reject => return false,
        }
    }
}

pub struct Cache{
    cache: HashMap<(i32,i32,u32),CellState>,
}

impl Cache{
    pub fn new() -> Cache{
        Cache{
            cache: HashMap::new(),
        }
    }
    pub fn get(&mut self, x: i32, y: i32) -> bool {
        get_cell_memo(&mut self.cache, x, y)
    }
}
