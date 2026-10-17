use soroban_sdk::{symbol_short, Env, Symbol, Vec};

pub fn supported_severities(env: &Env) -> Vec<Symbol> {
    let mut out = Vec::new(env);
    out.push_back(symbol_short!("critical"));
    out.push_back(symbol_short!("high"));
    out.push_back(symbol_short!("medium"));
    out.push_back(symbol_short!("low"));
    out
}

/// Relative weight of a severity for scoring and prioritisation:
/// critical 4, high 3, medium 2, low 1. Unknown severities have no weight.
pub fn severity_weight(severity: &Symbol) -> Option<u32> {
    if *severity == symbol_short!("critical") {
        Some(4)
    } else if *severity == symbol_short!("high") {
        Some(3)
    } else if *severity == symbol_short!("medium") {
        Some(2)
    } else if *severity == symbol_short!("low") {
        Some(1)
    } else {
        None
    }
}

pub fn is_known_severity(env: &Env, severity: &Symbol) -> bool {
    supported_severities(env).contains(severity)
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{symbol_short, Env};

    #[test]
    fn test_severity_list_is_deterministic() {
        let env = Env::default();
        let first  = supported_severities(&env);
        let second = supported_severities(&env);
        assert_eq!(first, second);
    }

    #[test]
    fn test_severity_list_contains_expected_values() {
        let env = Env::default();
        let sevs = supported_severities(&env);
        assert_eq!(sevs.len(), 4);
        assert!(sevs.contains(&symbol_short!("critical")));
        assert!(sevs.contains(&symbol_short!("high")));
        assert!(sevs.contains(&symbol_short!("medium")));
        assert!(sevs.contains(&symbol_short!("low")));
    }

    #[test]
    fn test_unknown_severity_is_rejected() {
        let env = Env::default();
        assert!(!is_known_severity(&env, &symbol_short!("ultra")));
    }

    #[test]
    fn test_weights_are_ordered_by_severity() {
        assert_eq!(severity_weight(&symbol_short!("critical")), Some(4));
        assert_eq!(severity_weight(&symbol_short!("high")), Some(3));
        assert_eq!(severity_weight(&symbol_short!("medium")), Some(2));
        assert_eq!(severity_weight(&symbol_short!("low")), Some(1));
    }

    #[test]
    fn test_unknown_severity_has_no_weight() {
        assert_eq!(severity_weight(&symbol_short!("ultra")), None);
    }

    #[test]
    fn test_every_supported_severity_has_a_distinct_weight() {
        let env = Env::default();
        let sevs = supported_severities(&env);
        let mut seen: [u32; 4] = [0; 4];
        for (i, sev) in sevs.iter().enumerate() {
            seen[i] = severity_weight(&sev).expect("supported severity must have a weight");
        }
        for i in 0..4 {
            for j in (i + 1)..4 {
                assert_ne!(seen[i], seen[j]);
            }
        }
    }
}
