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


#[derive(Clone)]
#[derive(Copy)]
enum CellResult{
    Tree,
    Empty,
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
    for dx in -1..2 {
        for dy in -1..2 {
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

fn get_cell_memo(cache: &mut HashMap<(i32,i32,u32),CellState>, x: i32, y: i32) -> CellResult {
    let mut t = 0;
    loop {
        match get_cell_at_memo(cache,x,y,t){
            CellState::Waiting => t+=1,
            CellState::Accept => return CellResult::Tree,
            CellState::Reject => return CellResult::Empty,
        }
    }
}

fn main() {
    let mut cache : HashMap<(i32,i32,u32),CellState> = HashMap::new();
    for x in 0..100 {
        for y in 0..100 {
            let s: String = match get_cell_memo(&mut cache,x,y) {
                CellResult::Tree => "\x1b[42m  \x1b[49m".to_string(),
                CellResult::Empty => "\x1b[40m  \x1b[49m".to_string(),
            };
            print!("{}",s);
        }
        println!();
    }
    println!("Cache size: {}",cache.len());
}
