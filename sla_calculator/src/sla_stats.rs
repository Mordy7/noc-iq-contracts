use soroban_sdk::{contracttype, symbol_short, Env, Symbol};

const STATS_KEY: Symbol = symbol_short!("STATS2");

#[contracttype]
#[derive(Clone)]
pub struct SLAStats {
    pub total_calculations: u64,
    pub met_count: u64,
    pub violation_count: u64,
    pub total_penalty: i128,
}

/// Loads the running SLA stats counter, defaulting to all-zero.
pub fn get_stats(env: &Env) -> SLAStats {
    env.storage()
        .instance()
        .get(&STATS_KEY)
        .unwrap_or(SLAStats {
            total_calculations: 0,
            met_count: 0,
            violation_count: 0,
            total_penalty: 0,
        })
}

/// Atomically records the outcome of one SLA calculation.
pub fn record_calculation(env: &Env, met: bool, penalty: i128) {
    let mut stats = get_stats(env);
    stats.total_calculations += 1;
    if met {
        stats.met_count += 1;
    } else {
        stats.violation_count += 1;
        stats.total_penalty += penalty;
    }
    env.storage().instance().set(&STATS_KEY, &stats);
}

/// Aggregate `(success, failure)` counts: SLAs met and SLAs violated.
pub fn get_success_failure_counts(env: &Env) -> (u64, u64) {
    let stats = get_stats(env);
    (stats.met_count, stats.violation_count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SLACalculatorContract;

    #[test]
    fn test_counts_start_at_zero() {
        let env = Env::default();
        let id = env.register_contract(None, SLACalculatorContract);
        env.as_contract(&id, || assert_eq!(get_success_failure_counts(&env), (0, 0)));
    }

    #[test]
    fn test_counts_track_successes_and_failures_separately() {
        let env = Env::default();
        let id = env.register_contract(None, SLACalculatorContract);
        env.as_contract(&id, || {
            record_calculation(&env, true, 0);
            record_calculation(&env, true, 0);
            record_calculation(&env, false, 250);
            assert_eq!(get_success_failure_counts(&env), (2, 1));
        });
    }

    #[test]
    fn test_counts_sum_to_total_calculations() {
        let env = Env::default();
        let id = env.register_contract(None, SLACalculatorContract);
        env.as_contract(&id, || {
            for i in 0..10u32 {
                record_calculation(&env, i % 3 != 0, 10);
            }
            let (ok, bad) = get_success_failure_counts(&env);
            assert_eq!(ok + bad, get_stats(&env).total_calculations);
            assert_eq!((ok, bad), (6, 4));
        });
    }
}
