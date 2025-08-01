// use std::collections::HashMap;
use std::mem::swap;
use std::fs;
use indexmap::IndexMap;
use rand::Rng;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Pair {
    l: u32,
    r: u32,
}

fn max_pair_freq(tokens_freq: &mut IndexMap<Pair, u32>, pairs: &mut Vec<Pair>) -> Pair {
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

// this is the fast version of the tokenize function
fn tokenize_fast(tokens_in: &Vec<u32>, tokens_freq: &mut IndexMap<Pair, u32>, pairs: &mut Vec<Pair>, max_pair: Pair) -> Vec<u32> {
    let mut i = 0;
    let n = tokens_in.len();
    let mut tokens_out = Vec::<u32>::new();
    while i < n {
        if i+1 < n {
            let mut pair = Pair {l: 0,r: 0,};
            if tokens_in[i] == max_pair.l && tokens_in[i+1] == max_pair.r {
                if tokens_out.len() > 0  {
                    pair.l = tokens_out[tokens_out.len()-1];
                    pair.r = tokens_in[i];
                    if let Some(count) = tokens_freq.get_mut(&pair) {
                        *count -= 1;
                        // to delete the pair from the map if count is 0
                        if *count == 0 {
                            tokens_freq.swap_remove(&pair);
                        }
                    }
                    pair.r = (pairs.len()-1) as u32;
                    *tokens_freq.entry(pair.clone()).or_insert(0) += 1;
                }
                if let Some(count) = tokens_freq.get_mut(&max_pair) {
                    *count -= 1;
                    if *count == 0 {
                        tokens_freq.swap_remove(&max_pair);
                    }
                }
                tokens_out.push((pairs.len()-1) as u32);
                i += 2;

                if i < tokens_in.len() {
                    pair.l = tokens_in[i-1];
                    pair.r = tokens_in[i];
                    if let Some(count) = tokens_freq.get_mut(&pair) {
                        *count -= 1;
                        if *count == 0 {
                            tokens_freq.swap_remove(&pair);
                        }
                    }
                    pair.l = tokens_out[tokens_out.len()-1];
                    *tokens_freq.entry(pair.clone()).or_insert(0) += 1;
                }
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

fn render_text(text: &mut String, pairs: &Vec<Pair>, token:u32) {
    if token < pairs.len() as u32 {
        if pairs[token as usize].l == token {
            text.push(pairs[token as usize].l as u8 as char);
            return;
        }

        render_text(text, pairs, pairs[token as usize].l);
        render_text(text, pairs, pairs[token as usize].r);

    }
    else {
        println!("token is out of range: {}", token);
        return;
    }
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
    // Read from the comprehensive test file
    let text_in = fs::read_to_string("bpe_test_data.txt")
        .expect("Failed to read bpe_test_data.txt file");
    
    println!("Processing {} characters of text...", text_in.len());
    
    
    // create a vector of pairs table which keeps all the encoded pairs
    let mut pairs = Vec::<Pair>::new();
    for i in 0..256 {
        pairs.push(Pair {
            l: i as u32,
            r: u32::MAX,
        });
    }
    
    let mut tokens_in = Vec::<u32>::new();
    for i in 0..text_in.len() {
        tokens_in.push(text_in.as_bytes()[i] as u32);
    }

    let mut tokens_freq = IndexMap::<Pair, u32>::new();
    for i in 0..tokens_in.len()-1 {
        let pair = Pair {
            l: tokens_in[i],
            r: tokens_in[i+1],
        };
        *tokens_freq.entry(pair).or_insert(0) += 1;
    }

    let initial_length = tokens_in.len();
    let mut iteration_count = 0;
    
    loop {
        let max_pair = max_pair_freq(&mut tokens_freq, &mut pairs);
        if tokens_freq[&max_pair] == 1 {
            // println!("tokens_out: {}", tokens_out.len());
            println!("tokens length: {}", tokens_in.len());
            println!("total pairs: {}", pairs.len());
            render_tokens(&tokens_in, &pairs);
            break;
        }
        let mut tokens_out = tokenize_fast(&tokens_in, &mut tokens_freq, &mut pairs, max_pair);
        tokens_in.clear();
        swap(&mut tokens_in, &mut tokens_out);
        tokens_out.clear();
        iteration_count += 1;
        
        if iteration_count % 10 == 0 {
            println!("Iteration {}: {} tokens (compression: {:.2}%)", 
                     iteration_count, tokens_in.len(), 
                     100.0 * (1.0 - tokens_in.len() as f64 / initial_length as f64));
        }
    }
    
    println!("\nBPE Compression Results:");
    println!("Original length: {} characters", initial_length);
    println!("Compressed length: {} tokens", tokens_in.len());
    println!("Compression ratio: {:.2}%", 100.0 * (1.0 - tokens_in.len() as f64 / initial_length as f64));
    println!("Total merge operations: {}", iteration_count);
    println!("Vocabulary size: {} pairs", pairs.len());

    let mut iteration = 0;
    let mut token = rand::thread_rng().gen_range(0..pairs.len() as u32);
    let mut next_tokens = Vec::<u32>::new();
    while iteration < 100 {
        // println!("token: {}, pairs.len(): {}", token, pairs.len());
        if token == u32::MAX {
            break;
        }
        let mut text_out = String::new();
        render_text(&mut text_out, &pairs, token);
        println!("{}", text_out);
        
        loop {
            for i in 0..pairs.len() {
                if pairs[i].l == token && pairs[i].r != u32::MAX {
                    next_tokens.push(pairs[i].r);
                }
            }
            if next_tokens.len() > 0 || token < 256 {
                break;
            }
            
            if pairs[token as usize].r != u32::MAX {
                token = pairs[token as usize].r;
                next_tokens.clear();
                // break;
            }
        }

        if next_tokens.len() == 0 {
            break;
        }
        token = next_tokens[rand::thread_rng().gen_range(0..next_tokens.len())];
        iteration += 1;
    }
}

