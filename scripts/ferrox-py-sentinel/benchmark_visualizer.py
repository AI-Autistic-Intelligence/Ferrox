import math
from dataset_generator import generate_synthetic_telemetry
from feature_pipeline import calculate_shannon_entropy

def compute_roc_and_confusion_matrix():
    """Computes ROC Metrics, Confusion Matrix, Precision, Recall & F1-Score (Malware Data Science Ch. 7 & 9)."""
    dataset = generate_synthetic_telemetry(500)

    tp = 0 # True Positives
    fp = 0 # False Positives
    tn = 0 # True Negatives
    fn = 0 # False Negatives

    threshold = 4.8

    for uri, body, label in dataset:
        entropy = calculate_shannon_entropy(uri.encode('utf-8'))
        has_keywords = "UNION" in uri or "SELECT" in uri or "script" in uri

        predicted = 1 if (entropy > threshold or has_keywords) else 0

        if predicted == 1 and label == 1:
            tp += 1
        elif predicted == 1 and label == 0:
            fp += 1
        elif predicted == 0 and label == 0:
            tn += 1
        elif predicted == 0 and label == 1:
            fn += 1

    precision = tp / (tp + fp) if (tp + fp) > 0 else 0.0
    recall = tp / (tp + fn) if (tp + fn) > 0 else 0.0
    f1_score = 2 * (precision * recall) / (precision + recall) if (precision + recall) > 0 else 0.0
    tpr = recall
    fpr = fp / (fp + tn) if (fp + tn) > 0 else 0.0

    print("=== FERROX SENTINEL BENCHMARK METRICS ===")
    print(f"True Positives  (TP) : {tp}")
    print(f"False Positives (FP) : {fp}")
    print(f"True Negatives  (TN) : {tn}")
    print(f"False Negatives (FN) : {fn}")
    print(f"Precision            : {precision:.4f}")
    print(f"Recall (TPR)         : {tpr:.4f}")
    print(f"False Positive Rate  : {fpr:.4f}")
    print(f"F1 Score             : {f1_score:.4f}")

if __name__ == "__main__":
    compute_roc_and_confusion_matrix()
