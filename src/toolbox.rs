use std::env;
use Vec;
    
pub fn env_str(var: &str)
-> String
{
    env::var(var).unwrap()
}

pub fn env_f64(var: &str)
-> f64
{
    env_str(var).parse().unwrap()
}

pub fn charind(s: &str, i: usize)
-> char
{
    s.chars().nth(i).unwrap()
}

pub fn strsz(s: &str)
-> usize
{
    s.chars().count()
}

pub fn vecstring(v: Vec<&str>)
-> Vec<String>
{
    v.iter().map(|s| s.to_string()).collect()
}
