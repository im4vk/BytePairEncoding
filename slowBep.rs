// use std::collections::HashMap;
use std::mem::swap;
use indexmap::IndexMap;


#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Pair {
    l: u32,
    r: u32,
}

fn max_pair_freq(tokens_freq: &IndexMap<Pair, u32>, pairs: &mut Vec<Pair>) -> Pair {
    let mut max_pair = Pair {
        l: 0,
        r: 0,
    };
    let mut max_value = 0;
    for (key, value) in tokens_freq.iter() {
        if *value > max_value || (*value == max_value && key.l < max_pair.l) {
            max_pair = key.clone();
            max_value = *value;
        }
    }
    if max_value > 1 {
        pairs.push(max_pair.clone());
    }
    max_pair
}

fn tokenize(tokens_in: &Vec<u32>, pairs: &mut Vec<Pair>, pair: Pair) -> Vec<u32> {
    let mut i = 0;
    let n = tokens_in.len();
    let mut tokens_out = Vec::<u32>::new();
    while i < n {
        if i+1 < n {
            if tokens_in[i] == pair.l && tokens_in[i+1] == pair.r {
                tokens_out.push((pairs.len()-1) as u32);
                i += 2;
            }
            else{
                tokens_out.push(tokens_in[i]);
                i += 1;
            }
        }
        else {
            tokens_out.push(tokens_in[i]);
            i += 1;
        }
    }
    tokens_out
}

fn render_tokens(tokens: &Vec<u32>, pairs: &Vec<Pair>) {
    for i in 0..tokens.len() {
        if tokens[i] < pairs.len() as u32 {
            if pairs[tokens[i] as usize].l == tokens[i] {
                print!("{}", pairs[tokens[i] as usize].l as u8 as char);
            }
            else {
                print!("[{}]", tokens[i]);
            }
        }
        else {
            print!("[{}]", tokens[i]);
        }
    }
    println!();
}

fn main() {
    let text_in = String::from("The original BPE algorithm operates by iteratively replacing the most common contiguous \
    sequences of characters in a target text with unused 'placeholder' bytes. The iteration ends when no sequences can be found, \
    leaving the target text effectively compressed. Decompression can be performed by reversing this process, querying known \
    placeholder terms against their corresponding denoted sequence, using a lookup table. In the original paper, this lookup table \
    is encoded and stored alongside the compressed text.");
    
    
    // create a vector of pairs table which keeps all the encoded pairs
    let mut pairs = Vec::<Pair>::new();
    for i in 0..256 {
        pairs.push(Pair {
            l: i as u32,
            r: 555 as u32,
        });
    }
    
    let mut tokens_in = Vec::<u32>::new();
    for i in 0..text_in.len() {
        tokens_in.push(text_in.as_bytes()[i] as u32);
    }

    loop {
        let mut tokens_freq = IndexMap::<Pair, u32>::new();
        for i in 0..tokens_in.len()-1 {
            let pair = Pair {
                l: tokens_in[i],
                r: tokens_in[i+1],
            };
            *tokens_freq.entry(pair).or_insert(0) += 1;
        }
        let max_pair = max_pair_freq(&tokens_freq, &mut pairs);
        if tokens_freq[&max_pair] == 1 {
            // println!("tokens_out: {}, tokens_in: {}", tokens_out.len(), tokens_in.len());
            println!("tokens length: {}", tokens_in.len());
            println!("total pairs: {}", pairs.len());
            render_tokens(&tokens_in, &pairs);
            break;
        }
        let mut tokens_out = tokenize(&tokens_in, &mut pairs, max_pair);
        tokens_in.clear();
        swap(&mut tokens_in, &mut tokens_out);
        tokens_out.clear();
        tokens_freq.clear();
    }
}
