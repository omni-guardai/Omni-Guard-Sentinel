/// 📊 Calculates the mathematical Shannon Entropy of a file slice.
/// Limited to the first 4KB to maintain zero disk reading latency.
pub fn calculate_entropy(file_bytes: &[u8]) -> f64 {
    if file_bytes.is_empty() {
        return 0.0;
    }

    let mut byte_frequencies = [0u32; 256];
    let chunk_size = std::cmp::min(file_bytes.len(), 4096);
    
    for &byte in &file_bytes[..chunk_size] {
        byte_frequencies[byte as usize] += 1;
    }

    let mut entropy_total = 0.0;
    let total_count_f64 = chunk_size as f64;

    for &count in &byte_frequencies {
        if count > 0 {
            let probability = count as f64 / total_count_f64;
            entropy_total -= probability * probability.log2();
        }
    }

    entropy_total
}
