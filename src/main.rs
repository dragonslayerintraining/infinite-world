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
#[derive(PartialEq)]
enum CellState{
    Waiting,
    Trying,
    Accept,
    Reject,
}

fn state_is_done(state: CellState) -> bool{
    match state {
        CellState::Waiting => false,
        CellState::Trying => false,
        CellState::Accept => true,
        CellState::Reject => true,
    }
}

fn cellular_automata_step(input: [[CellState; 3]; 3],r: u64) -> CellState{
    if state_is_done(input[1][1]){
        return input[1][1];
    }
    for i in 0..3 {
        for j in 0..3 {
            if input[i][j] == CellState::Accept {
                return CellState::Reject;
            }
        }
    }
    if input[1][1] == CellState::Trying {
        let mut flag: bool = false;
        for i in 0..3 {
            for j in 0..3 {
                if i==1 && j==1 {
                    continue;
                }
                if input[i][j] == CellState::Trying {
                    flag = true;
                }
            }
        }
        if !flag {
            return CellState::Accept;
        }
    }
    if r%9 == 0 {
        return CellState::Trying;
    }else{
        return CellState::Waiting;
    }
}

fn get_cell_at_memo(cache: &mut HashMap<(i32,i32,u32),CellState>, x: i32, y: i32, t: u32) -> CellState{
    if let Some(state)=cache.get(&(x,y,t)){
        return state.clone();
    }
    if t == 0 {
        return CellState::Waiting;
    }
    let old : CellState = get_cell_at_memo(cache,x,y,t-1);
    if state_is_done(old) {
        cache.insert((x,y,t),old);
        return old;
    }
    let pred: [[CellState; 3]; 3] = [[get_cell_at_memo(cache,x-1,y-1,t-1),get_cell_at_memo(cache,x  ,y-1,t-1),get_cell_at_memo(cache,x+1,y-1,t-1)],
    [get_cell_at_memo(cache,x-1,y  ,t-1),get_cell_at_memo(cache,x  ,y  ,t-1),get_cell_at_memo(cache,x+1,y  ,t-1)],
    [get_cell_at_memo(cache,x-1,y+1,t-1),get_cell_at_memo(cache,x  ,y+1,t-1),get_cell_at_memo(cache,x+1,y+1,t-1)]];

    let res: CellState = cellular_automata_step(pred,get_cell_seq(x,y,t));
    cache.insert((x,y,t),res);
    return res;
}

fn cell_to_char(state: CellState) -> char {
    match state{
        CellState::Waiting=>{'?'}
        CellState::Trying=>{'!'}
        CellState::Accept=>{'#'}
        CellState::Reject=>{'.'}
    }
}

fn get_cell_memo(cache: &mut HashMap<(i32,i32,u32),CellState>,x: i32, y: i32) -> char {
    let mut t = 0;
    while get_cell_at_memo(cache,x,y,t) == CellState::Waiting || get_cell_at_memo(cache,x,y,t) == CellState::Trying {
        t += 1;
    }
    return cell_to_char(get_cell_at_memo(cache,x,y,t));
}

fn main() {
    let mut cache : HashMap<(i32,i32,u32),CellState> = HashMap::new();
    for x in 0..100 {
        for y in 0..100 {
            print!("{}",get_cell_memo(&mut cache,x,y));
        }
        println!();
    }
    println!("Cache size: {}",cache.len());
}
