extern crate cfg_if;
extern crate wasm_bindgen;
extern crate typed_arena;
extern crate regex;
extern crate web_sys;
use web_sys::console;


mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log(s: &str);
}

use typed_arena::Arena;

struct Tree<'a> {
    children: Option<(&'a Tree<'a>, &'a Tree<'a>)>,
}

fn bottom_up_tree<'r>(arena: &'r Arena<Tree<'r>>, depth: i32)
                  -> &'r Tree<'r> {
    let mut tree = arena.alloc(Tree { children: None });
    if depth > 0 {
        let right = bottom_up_tree(arena, depth - 1);
        let left = bottom_up_tree(arena, depth - 1);
        tree.children = Some((left, right))
    }
    tree
}

#[wasm_bindgen]
pub fn run_binary_tree(depth: i32) {
    let arena = Arena::new();
    let tree = bottom_up_tree(&arena, depth);
}





use std::io::{self, Read};
use std::thread;

macro_rules! regex { ($re:expr) => {
    ::regex::bytes::Regex::new($re).unwrap() 
} }

fn count_reverse_complements(sequence : Vec<u8>) -> Vec<String> {
    // Search for occurrences of the following patterns:
    let variants = vec![
        regex!("agggtaaa|tttaccct"),
        regex!("[cgt]gggtaaa|tttaccc[acg]"),
        regex!("a[act]ggtaaa|tttacc[agt]t"),
        regex!("ag[act]gtaaa|tttac[agt]ct"),
        regex!("agg[act]taaa|ttta[agt]cct"),
        regex!("aggg[acg]aaa|ttt[cgt]ccct"),
        regex!("agggt[cgt]aa|tt[acg]accct"),
        regex!("agggta[cgt]a|t[acg]taccct"),
        regex!("agggtaa[cgt]|[acg]ttaccct"),
    ];
    variants.iter()
	    .map(|ref variant| 
		format!("{} {}", 
			variant.to_string(), 
			variant.find_iter(&sequence).count()) ) 
            .collect()
}

fn find_replaced_sequence_length(sequence : Vec<u8>) -> usize {
    // Replace the following patterns, one at a time:
    let substs = vec![
        (regex!("tHa[Nt]"), &b"<4>"[..]),
        (regex!("aND|caN|Ha[DS]|WaS"), &b"<3>"[..]),
        (regex!("a[NSt]|BY"), &b"<2>"[..]),
        (regex!("<[^>]*>"), &b"|"[..]),
        (regex!("\\|[^|][^|]*\\|"), &b"-"[..]),
    ];
    let mut seq = sequence;
    // Perform the replacements in sequence:
    for (re, replacement) in substs {
        seq = re.replace_all(&seq, replacement).into_owned();
    }
    seq.len()
}


#[wasm_bindgen]
pub fn run_regex(depth: i32) {
    let input = include_bytes!("regexredux-input.txt");
    let input_len = input.len();
    let sequence = regex!(">[^\n]*\n|\n")
			.replace_all(input, &b""[..]).into_owned();
    let clen = sequence.len();
    let seq_clone = sequence.clone();
    find_replaced_sequence_length(seq_clone);
    count_reverse_complements(sequence);
}


use std::{cmp, mem};
const NUM_BLOCKS: u32 = 24;

fn fannkuch(n: i32) -> (i32, i32) {
    // Precompute a table a factorials to reuse all over the place.
    let mut factorials = [1; 16];
    for i in 1..n as usize + 1 {
        factorials[i] = factorials[i - 1] * i as u32;
    }
    let perm_max = factorials[n as usize];

    // Compute the number of blocks and their size. If n! is less than
    // NUM_BLOCKS then use a single block (perform the work serially for small
    // values of n). If n! doesn't divide exactly by NUM_BLOCKS, then add one
    // extra block to compute the remainder.
    let (num_blocks, block_size) = if perm_max < NUM_BLOCKS {
        (1, perm_max)
    } else {
        (NUM_BLOCKS + if perm_max % NUM_BLOCKS == 0 { 0 } else { 1 },
         perm_max / NUM_BLOCKS)
    };

    // Compute the `checksum` and `maxflips` for each block in parallel.
    (0..num_blocks).map(|i_block| {
        let initial = i_block * block_size;
        let mut count = [0i32; 16];
        let mut temp = [0i32; 16];
        let mut current = [0i32; 16];

        // Initialise `count` and the current permutation (`current`)
        for (i, value) in current.iter_mut().enumerate() {
            *value = i as i32;
        }

        let mut permutation_index = initial as i32;
        for i in (1..n as usize).rev() {
            let factorial = factorials[i] as i32;
            let d = permutation_index / factorial;
            permutation_index %= factorial;
            count[i] = d;

            temp.copy_from_slice(&current);
            let d = d as usize;
            for j in 0..i + 1 {
                current[j] = if j + d <= i {
                    temp[j + d]
                } else {
                    temp[j + d - i - 1]
                };
            }
        }

        // Iterate over each permutation in the block.
        let last_permutation_in_block = cmp::min(initial + block_size,
                                                 perm_max) - 1;
        let mut permutation_index = initial;
        let (mut checksum, mut maxflips) = (0, 0);
        loop {
            // If the first value in the current permutation is not 1 (0) then
            // we will need to do at least one flip for `current`.
            if current[0] > 0 {
                // Copy the current permutation to work on it.
                temp.copy_from_slice(&current);

                // Flip `temp` (the copy of the current permutation) until its
                // first element is 1 (0).
                let mut flip_count = 1;
                let mut first_value = current[0] as usize;
                while temp[first_value] != 0 {
                    let new_first_value = mem::replace(&mut temp[first_value],
                                                       first_value as i32);
                    // If the first value is greater than 3 (2), then we are
                    // flipping a series of four or more values so we will need
                    // to flip additional elements in the middle of `temp`.
                    if first_value > 2 {
                        temp[1..first_value].reverse();
                    }

                    // Update `first_value` to the value we saved earlier and
                    // record a flip in `flip_count`.
                    first_value = new_first_value as usize;
                    flip_count += 1;
                }

                // Update the `checksum` and `maxflips` of this block.
                checksum += if permutation_index % 2 == 0 {
                    flip_count
                } else {
                    -flip_count
                };
                maxflips = cmp::max(maxflips, flip_count);
            }

            // If this was the last permutation in the block, we're done: return
            // the `checksum` and `maxflips` values which get reduced across
            // blocks in parallel by `rayon`.
            if permutation_index >= last_permutation_in_block {
                return (checksum, maxflips);
            }
            permutation_index += 1;

            // Generate the next permutation.
            let mut first_value = current[1];
            current[1] = current[0];
            current[0] = first_value;
            let mut i = 1;
            while count[i] >= i as i32 {
                count[i] = 0;
                i += 1;
                let new_first_value = current[1];
                current[0] = new_first_value;
                for j in 1..i {
                    current[j] = current[j + 1];
                }
                current[i] = mem::replace(&mut first_value, new_first_value);
            }
            count[i] += 1;
        }
    }).fold((0, 0),
              |(cs1, mf1), (cs2, mf2)| (cs1 + cs2, cmp::max(mf1, mf2)))
}

#[wasm_bindgen]
pub fn run_fannkuch(n: i32) {
    let (checksum, maxflips) = fannkuch(n);
    // log(&format!("{}", checksum));
}
    

struct NQueen {
    n: usize,
    ret: u32,
}
impl NQueen {
    fn new(n: usize) -> NQueen {
        NQueen {
            n, ret: 0,
        }
    }
    fn solve(&mut self) -> u32 {
        let mut c = vec![false; self.n]; // column |
        let mut d1 = vec![false; 2*self.n - 1]; // diagonal /
        let mut d2 = vec![false; 2*self.n - 1]; // diagonal \
        
        self.helper(0, &mut c, &mut d1, &mut d2);

        return self.ret;
    }

    fn helper(&mut self, i: usize, c: &mut [bool], d1: &mut [bool], d2: &mut [bool]) {
        if i == self.n {
            self.ret += 1;
            return
        }
        for j in 0..self.n {
            let idx1: usize = i + j;
            let idx2: usize = i+self.n-j-1;
            if !c[j] && !d1[idx1] && !d2[idx2] {
                c[j] = true;
                d1[idx1] = true;
                d2[idx2] = true;
                self.helper(i+1, c, d1, d2);
                c[j] = false;
                d1[idx1] = false;
                d2[idx2] = false;
            }
        }
    }
}

#[wasm_bindgen]
pub fn run_nqueen(n: usize) -> u32 {
    let mut q = NQueen::new(n);
    q.solve()
}

fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    fibonacci(n-1) + fibonacci(n-2)
}

#[wasm_bindgen]
pub fn run_fibonacci(n: u32) -> u32 {
    return fibonacci(n);
}

