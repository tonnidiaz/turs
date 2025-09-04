pub extern crate chrono;
pub mod marcros;
pub mod consts;
pub mod funcs;
mod tests;
pub use funcs::*;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

use std::{error, fmt::Debug, path::PathBuf, pin::Pin};

use rand::{distr::uniform::SampleUniform, Rng};
use serde_json::Value;

/// Generates an int or float random num
pub fn gen_rand<T: SampleUniform + std::cmp::PartialEq + PartialOrd>(start: T, end: T) -> T{
    let mut rng = rand::rng();
    let num: T = rng.random_range(start..=end);
    num
}

 pub trait StrTrait{
    fn dirname(&self) ->String;
}

impl StrTrait for String {
    fn dirname(&self) ->String {
        PathBuf::from(self).parent().unwrap().to_str().unwrap().to_string()
    }
}

pub trait ValueTr{
    fn tu_string(&self)-> String;
}

impl ValueTr for Value{
    fn tu_string(&self)-> String {
        self.to_string().replace(r#"""#, "")
    }
}
 pub trait EnumTr{
    fn as_str(&self) -> String where Self: Debug{
        format!("{self:?}")
    }
    fn to_vec() -> Vec<Self> where Self: Sized;
    fn from_str(s: &str) -> Option<Self> where Self: Sized + Debug {
        Self::to_vec()
            .into_iter()
            .find(|x| x.as_str().to_lowercase() == s.to_lowercase())
    }
 }

 pub trait ToValue{
    fn to_value(&self) -> serde_json::Value;
 }

pub type Fut<T> = Pin<Box<dyn Future<Output = T> + Send + Sync>>;
pub fn print(p: String){
    println!("\n[{}] {p}", ts());
}

#[cfg(test)]
mod tests1 {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
pub type Res<T> = Result<T, Box<dyn error::Error + Send + Sync>>;
