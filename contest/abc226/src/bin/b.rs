#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars, marker::Usize1, marker::Bytes};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet, VecDeque, BinaryHeap};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use superslice::Ext;

fn main() {
    input!{
        n : usize,
    }

    let mut set = HashSet::new();
    for _ in 0..n {
        input!{
            l: usize,
            a: [i32; l],
        };

        // let s = a.into_iter().collect::<String>();
        set.insert(a);
    }

    println!("{}", set.len());
}
