pub use chumsky::prelude::*;
pub use itertools::Itertools;
pub use ndarray::prelude::*;
use num_traits::Num;
pub use rayon::prelude::*;
use std::fmt::Debug;
use std::str::FromStr;

pub fn offset_coords(coords: (usize, usize), offset: (isize, isize)) -> Option<(usize, usize)> {
    let a = coords.0 as isize + offset.0;
    let b = coords.1 as isize + offset.1;
    if a < 0 || b < 0 {
        None
    } else {
        Some((a as usize, b as usize))
    }
}

pub fn num<N: Num + FromStr>() -> impl Parser<char, N, Error = Simple<char>>
where
    <N as FromStr>::Err: Debug,
{
    text::int::<char, Simple<char>>(10).from_str().unwrapped()
}

pub fn int<N: Num + FromStr + Clone>() -> impl Parser<char, N, Error = Simple<char>>
where
    <N as FromStr>::Err: Debug,
{
    let neg = just('-').ignore_then(num::<N>().map(|x: N| N::zero() - x));
    let pos = num::<N>();
    neg.or(pos)
}

pub fn literal(s: &str) -> impl Parser<char, (), Error = Simple<char>> {
    let s = String::from(s);
    any().repeated().at_most(s.len()).try_map(move |ss, span| {
        if ss.into_iter().collect::<String>() == s {
            Ok(())
        } else {
            Err(Simple::expected_input_found(span, None, None))
        }
    })
}
