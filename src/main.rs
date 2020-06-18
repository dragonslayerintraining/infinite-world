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
    WAITING,
    TRYING,
    ACCEPT,
    REJECT,
}

fn cellular_automata_step(input: [[CellState; 3]; 3],r: u64) -> CellState{
    if input[1][1] == CellState::ACCEPT || input[1][1] == CellState::REJECT {
        return input[1][1];
    }
    for i in 0..3 {
        for j in 0..3 {
            if input[i][j] == CellState::ACCEPT {
                return CellState::REJECT;
            }
        }
    }
    if input[1][1] == CellState::TRYING {
        let mut flag: bool = false;
        for i in 0..3 {
            for j in 0..3 {
                if i==1 && j==1 {
                    continue;
                }
                if input[i][j] == CellState::TRYING {
                    flag = true;
                }
            }
        }
        if !flag {
            return CellState::ACCEPT;
        }
    }
    if r%7 == 0 {
        return CellState::TRYING;
    }else{
        return CellState::WAITING;
    }
}

fn get_cell_at_memo(cache: &mut HashMap<(i32,i32,u32),CellState>, x: i32, y: i32, t: u32) -> CellState{
    if let Some(state)=cache.get(&(x,y,t)){
        return state.clone();
    }
    if t == 0 {
        return CellState::WAITING;
    }
    let old : CellState = get_cell_at_memo(cache,x,y,t-1);
    if old == CellState::ACCEPT || old == CellState::REJECT {
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
        CellState::WAITING=>{'?'}
        CellState::TRYING=>{'!'}
        CellState::ACCEPT=>{'#'}
        CellState::REJECT=>{' '}
    }
}

fn get_cell(x: i32, y: i32) -> char {
    let mut cache : HashMap<(i32,i32,u32),CellState> = HashMap::new();
    let mut t = 0;
    while get_cell_at_memo(&mut cache,x,y,t) == CellState::WAITING || get_cell_at_memo(&mut cache,x,y,t) == CellState::TRYING {
        t += 1;
    }
    return cell_to_char(get_cell_at_memo(&mut cache,x,y,t));
}

fn main() {
    for x in 0..50 {
        for y in 0..50 {
            print!("{}",get_cell(x,y));
        }
        println!();
    }
}
