use crate::prdoc_analyzer::ChangeCategory;

const EMBEDDING_DIM: usize = 8;

type Embedding = [f32; EMBEDDING_DIM];

const PROTOTYPE_NEW_FEATURE: Embedding =
    [0.80, 0.60, 0.05, 0.05, 0.05, 0.05, 0.05, 0.10];
const PROTOTYPE_BUG_FIX: Embedding =
    [0.05, 0.05, 0.80, 0.60, 0.05, 0.05, 0.05, 0.05];
const PROTOTYPE_BREAKING: Embedding =
    [0.05, 0.05, 0.05, 0.05, 0.80, 0.60, 0.05, 0.05];
const PROTOTYPE_REFACTOR: Embedding =
    [0.05, 0.05, 0.05, 0.05, 0.05, 0.10, 0.80, 0.10];
const PROTOTYPE_DOCS: Embedding =
    [0.05, 0.05, 0.05, 0.05, 0.05, 0.05, 0.10, 0.80];
const PROTOTYPE_INTERNAL: Embedding =
    [0.05, 0.05, 0.05, 0.80, 0.05, 0.80, 0.05, 0.05];

struct Prototype {
    category: ChangeCategory,
    embedding: Embedding,
}

fn prototypes() -> Vec<Prototype> {
    vec![
        Prototype {
            category: ChangeCategory::NewFeature,
            embedding: PROTOTYPE_NEW_FEATURE,
        },
        Prototype {
            category: ChangeCategory::BugFix,
            embedding: PROTOTYPE_BUG_FIX,
        },
        Prototype {
            category: ChangeCategory::BreakingChange,
            embedding: PROTOTYPE_BREAKING,
        },
        Prototype {
            category: ChangeCategory::Refactor,
            embedding: PROTOTYPE_REFACTOR,
        },
        Prototype {
            category: ChangeCategory::Documentation,
            embedding: PROTOTYPE_DOCS,
        },
        Prototype {
            category: ChangeCategory::Internal,
            embedding: PROTOTYPE_INTERNAL,
        },
    ]
}

pub fn classify_by_embedding(text: &str) -> Option<ChangeCategory> {
    let embedding = compute_embedding(text);
    let mut best_cat = ChangeCategory::Internal;
    let mut best_sim = -1.0f32;

    for proto in prototypes() {
        let sim = cosine_similarity(&embedding, &proto.embedding);
        if sim > best_sim {
            best_sim = sim;
            best_cat = proto.category;
        }
    }

    if best_sim > 0.3 { Some(best_cat) } else { None }
}

fn compute_embedding(text: &str) -> Embedding {
    let lower = text.to_lowercase();
    let mut emb = [0.0f32; EMBEDDING_DIM];

    let add_keywords =
        ["add", "new", "create", "implement", "introduce", "support"];
    let feature_keywords =
        ["feature", "capability", "functionality", "extend", "enable"];
    let fix_keywords =
        ["fix", "resolve", "correct", "patch", "repair", "workaround"];
    let resolve_keywords =
        ["bug", "error", "issue", "problem", "crash", "failure"];
    let break_keywords =
        ["remove", "delete", "rename", "deprecate", "break", "drop"];
    let change_keywords =
        ["change", "modify", "replace", "restructure", "reorganize"];
    let refactor_keywords =
        ["refactor", "clean", "simplify", "optimize", "improve"];
    let doc_keywords =
        ["document", "readme", "comment", "guide", "doc", "explain"];

    let count_matches = |keywords: &[&str]| -> f32 {
        keywords
            .iter()
            .map(|k| if lower.contains(k) { 1.0 } else { 0.0 })
            .sum()
    };

    emb[0] = count_matches(&add_keywords);
    emb[1] = count_matches(&feature_keywords);
    emb[2] = count_matches(&fix_keywords);
    emb[3] = count_matches(&resolve_keywords);
    emb[4] = count_matches(&break_keywords);
    emb[5] = count_matches(&change_keywords);
    emb[6] = count_matches(&refactor_keywords);
    emb[7] = count_matches(&doc_keywords);

    let norm: f32 = emb.iter().map(|v| v * v).sum::<f32>().sqrt();
    if norm > 0.0 {
        for v in emb.iter_mut() {
            *v /= norm;
        }
    }

    emb
}

fn cosine_similarity(a: &Embedding, b: &Embedding) -> f32 {
    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|v| v * v).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|v| v * v).sum::<f32>().sqrt();
    if norm_a > 0.0 && norm_b > 0.0 {
        dot / (norm_a * norm_b)
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_new_feature() {
        let result = classify_by_embedding(
            "Add new database support and implement feature",
        );
        assert_eq!(result, Some(ChangeCategory::NewFeature));
    }

    #[test]
    fn test_classify_bug_fix() {
        let result =
            classify_by_embedding("Fix crash and resolve error in bug");
        assert_eq!(result, Some(ChangeCategory::BugFix));
    }

    #[test]
    fn test_classify_breaking() {
        let result = classify_by_embedding(
            "Remove deprecated API and rename change struct",
        );
        assert_eq!(result, Some(ChangeCategory::BreakingChange));
    }

    #[test]
    fn test_classify_docs() {
        let result =
            classify_by_embedding("Document the readme and add guide comments");
        assert_eq!(result, Some(ChangeCategory::Documentation));
    }

    #[test]
    fn test_classify_refactor() {
        let result = classify_by_embedding(
            "Refactor and simplify optimize the codebase",
        );
        assert_eq!(result, Some(ChangeCategory::Refactor));
    }

    #[test]
    fn test_cosine_similarity_identical() {
        let v = [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        assert!((cosine_similarity(&v, &v) - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_cosine_similarity_orthogonal() {
        let a = [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let b = [0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        assert!((cosine_similarity(&a, &b) - 0.0).abs() < 0.001);
    }
}
