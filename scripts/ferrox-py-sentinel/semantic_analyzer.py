import math
from collections import Counter

class SemanticTextAnalyzer:
    """Computes subword token frequency vectors and Cosine Similarity (NLP with Transformers Ch. 2)."""
    
    @staticmethod
    def tokenize(text: str) -> list[str]:
        return [w.lower() for w in text.split() if w.isalnum()]

    @classmethod
    def get_vector(cls, text: str) -> Counter:
        return Counter(cls.tokenize(text))

    @classmethod
    def cosine_similarity(cls, text1: str, text2: str) -> float:
        vec1 = cls.get_vector(text1)
        vec2 = cls.get_vector(text2)
        
        intersection = set(vec1.keys()) & set(vec2.keys())
        numerator = sum(vec1[x] * vec2[x] for x in intersection)

        sum1 = sum(v**2 for v in vec1.values())
        sum2 = sum(v**2 for v in vec2.values())
        denominator = math.sqrt(sum1) * math.sqrt(sum2)

        if not denominator:
            return 0.0
        return float(numerator) / denominator

    @classmethod
    def is_out_of_distribution(cls, text: str, reference_corpus: list[str], threshold: float = 0.1) -> bool:
        """Detects if input query is Out-Of-Distribution (OOD) compared to reference corpus."""
        max_sim = max((cls.cosine_similarity(text, ref) for ref in reference_corpus), default=0.0)
        return max_sim < threshold

if __name__ == "__main__":
    corpus = ["user login request", "fetch user profile", "get game table status"]
    query_benign = "fetch profile data"
    query_anomalous = "UNION SELECT password FROM admin_credentials"

    sim_benign = max(SemanticTextAnalyzer.cosine_similarity(query_benign, ref) for ref in corpus)
    sim_anomalous = max(SemanticTextAnalyzer.cosine_similarity(query_anomalous, ref) for ref in corpus)

    print(f"Benign Similarity: {sim_benign:.3f}, Anomalous Similarity: {sim_anomalous:.3f}")
    print("OOD Anomalous Query Detected:", SemanticTextAnalyzer.is_out_of_distribution(query_anomalous, corpus))
