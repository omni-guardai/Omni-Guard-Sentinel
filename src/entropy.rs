// entropy.rs
// Calculates the Shannon Entropy of a given data buffer.

use std::collections::HashMap;

pub fn calculate_shannon_entropy(data: &[u8]) -> f64 {
    if data.is_empty() { return 0.0; }
    
    let mut frequency = HashMap::new();
    for &byte in data {
        *frequency.entry(byte).or_insert(0) += 1;
    }
    
    let mut entropy = 0.0;
    let len = data.len() as f64;
    
    for count in frequency.values() {
        let p = (*count as f64) / len;
        entropy -= p * p.log2();
    }
    
    entropy
}
