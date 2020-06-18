mod infinite_grid_mis;

fn main() {
    let mut cache : infinite_grid_mis::Cache = infinite_grid_mis::Cache::new();
    for x in 0..100 {
        for y in 0..100 {
            let s: String = match cache.get(x,y) {
                true => "\x1b[42m  \x1b[49m".to_string(),
                false => "\x1b[40m  \x1b[49m".to_string(),
            };
            print!("{}",s);
        }
        println!();
    }
    //println!("Cache size: {}",cache.len());
}
