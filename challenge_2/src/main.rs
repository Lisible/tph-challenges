use{rand as r,r::*};fn main(){(0..9).for_each(|_|print!("{}",r::thread_rng().gen_range(32u8,127)as char))}