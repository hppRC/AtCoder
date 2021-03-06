#![allow(non_snake_case)]
#[allow(unused_imports)]
use std::io::{self, Write};
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::cmp::{max, min, Ordering};
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}
macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}
macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, char) => {
        read_value!($next, String).chars().collect::<Vec<char>>()[0]
     };
    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
     };
    ($next:expr, isize1) => {
        read_value!($next, isize) - 1
    };
     ($next:expr, $t:ty) => {
         $next().parse::<$t>().expect("Parse error")
    };
}
macro_rules! printvec {
    ( $item:expr ) => {
        for &i in &$item {
            print!("{} ", i);
        }
        println!("");
    }
}
macro_rules! debug {
    ($($a:expr),*) => {
        println!(concat!($(stringify!($a), " = {:?}, "),*), $($a),*);
    }
}
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Rev<T>(pub T);
impl<T: PartialOrd> PartialOrd for Rev<T> {
    fn partial_cmp(&self, other: &Rev<T>) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}
impl<T: Ord> Ord for Rev<T> {
    fn cmp(&self, other: &Rev<T>) -> Ordering {
        other.0.cmp(&self.0)
    }
}
#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub struct Total<T>(pub T);
impl<T: PartialEq> Eq for Total<T> {}
impl<T: PartialOrd> Ord for Total<T> {
    fn cmp(&self, other: &Total<T>) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}
#[allow(dead_code)]
const MOD: usize = 1000000007;
#[allow(dead_code)]
fn factorial(n: usize) -> usize {
    (1..n+1).into_iter().fold(1, |acc, i| acc * i)
}
#[allow(dead_code)]
fn comb(n: usize, r: usize) -> usize {
    if n - r < r {
        comb(n, n - r)
    } else {
        (1..r+1).into_iter().fold(1, |acc, i| acc * (n - r + i) / i)
    }
}


fn main() {
    input!{
        N: usize, M: usize,
        ab: [(usize1, usize1); M],
    }

    let v1 = (0..N/2).collect::<Vec<usize>>();
    let v2 = (N/2..N).collect::<Vec<usize>>();

    let v1l = v1.len();
    let v2l = v2.len();

    let mut independent1: Vec<bool> = vec![true; 1 << v1l];

    for &(a, b) in &ab {
        if a < v1l && b < v1l {
            independent1[(1 << a) | (1 << b)] = false;
        }
    }

    for i in 0..(1 << v1l) {
        if !independent1[i] {
            for j in 0..v1l {
                independent1[i | (1 << j)] = false;
            }
        }
    }
    //v2すべてのbitを立てつつv2の頂点数分のvectorを確保
    let mut set: Vec<usize> = vec![(1 << v2l) - 1; 1 << v1l];

    for &(a, b) in &ab {
        if a < v1l && b >= v1l {
            set[1 << a] &= !(1 << (b - v1l));
        } else if a >= v1l && b < v1l {
            set[1 << b] &= !(1 << (a - v1l));
        }
    }

    for i in 0..(1 << v1l) {
        for j in 0..v1l {
            set[i | (1 << j)] = set[i] & set[1 << j];
        }
    }

    let mut independent2: Vec<bool> = vec![true; 1 << v2l];

    for &(a, b) in &ab {
        if a >= v1l && b >= v1l {
            independent2[(1 << (a - v1l)) | (1 << (b - v1l))] = false;
        }
    }

    for i in 0..(1 << v2l) {
        if !independent2[i] {
            for j in 0..v2l {
                independent2[i | (1 << j)] = false;
            }
        }
    }


    let mut dp: Vec<usize> = vec![0; 1 << v2l];

    for i in 0..(1 << v2l) {
        if independent2[i] {
            dp[i] = i.count_ones() as usize;
        }
    }

    for i in 0..(1 << v2l) {
        for j in 0..v2l {
            dp[i | (1 << j)] = max(dp[i | (1 << j)], dp[i]);
        }
    }

    let ans = independent1.into_iter()
        .enumerate()
        .filter_map(|(i, x)| if x {Some(i)} else {None})
        .map(|i| i.count_ones() as usize + dp[set[i]])
        .max()
        .unwrap();
    println!("{}", ans);
}