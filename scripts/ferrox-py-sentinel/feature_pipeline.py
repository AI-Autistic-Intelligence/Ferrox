import math
import hashlib

def calculate_shannon_entropy(data: bytes) -> float:
    """Calculates Shannon Entropy H(X) = -sum(p(x) * log2(p(x))) for bytes."""
    if not data:
        return 0.0
    
    counts = {}
    for b in data:
        counts[b] = counts.get(b, 0) + 1
        
    length = float(len(data))
    entropy = 0.0
    for count in counts.values():
        p = count / length
        if p > 0:
            entropy -= p * math.log2(p)
            
    return entropy

class FeatureHasherPython:
    """Feature Hashing Trick (Malware Data Science Ch. 8)."""
    def __init__(self, num_buckets=1024):
        self.num_buckets = num_buckets

    fn_hash = lambda self, s: int(hashlib.md5(s.encode('utf-8')).hexdigest(), 16) % self.num_buckets

    def transform(self, feature_strings):
        vec = [0.0] * self.num_buckets
        for feat in feature_strings:
            idx = self.fn_hash(feat)
            vec[idx] += 1.0
            
        # L2 Normalization
        norm = math.sqrt(sum(x*x for x in vec))
        if norm > 0:
            vec = [x / norm for x in vec]
            
        return vec
