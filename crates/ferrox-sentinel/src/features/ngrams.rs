/// Extracts character or token N-grams from request sequences and path trajectories
pub fn extract_character_ngrams(text: &str, n: usize) -> Vec<String> {
    if text.len() < n {
        return vec![text.to_string()];
    }

    let chars: Vec<char> = text.chars().collect();
    let mut ngrams = Vec::new();

    for i in 0..=(chars.len() - n) {
        let ngram: String = chars[i..i + n].iter().collect();
        ngrams.push(ngram);
    }

    ngrams
}

/// Computes the Jaccard Similarity Index between two sets of feature n-grams: J(A, B) = |A \cap B| / |A \cup B|
pub fn jaccard_similarity(set_a: &[String], set_b: &[String]) -> f64 {
    use std::collections::HashSet;

    let a: HashSet<_> = set_a.iter().collect();
    let b: HashSet<_> = set_b.iter().collect();

    let intersection_count = a.intersection(&b).count();
    let union_count = a.union(&b).count();

    if union_count == 0 {
        return 1.0;
    }

    intersection_count as f64 / union_count as f64
}
