use std::env;
    
pub fn env_f64(var: &str) -> f64 { env::var(var).unwrap().parse().unwrap() }

//pub fn strsz(s: &str) -> i64 { s.chars().count() as i64 }
