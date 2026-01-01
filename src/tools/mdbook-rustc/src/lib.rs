use rustc_target::spec::{TARGETS, Target, TargetMetadata, TargetTuple};

pub struct AllTargets {
    pub tier1_host: Vec<TargetInfo>,
    pub tier1_nohost: Vec<TargetInfo>,
    pub tier2_host: Vec<TargetInfo>,
    pub tier2_nohost: Vec<TargetInfo>,
    pub tier3: Vec<TargetInfo>,
}

pub struct TargetInfo {
    pub tuple: &'static str,
    pub meta: TargetMetadata,
}

impl AllTargets {
    fn load() -> Self {
        let mut tier1_host = Vec::new();
        let mut tier1_nohost = Vec::new();
        let mut tier2_host = Vec::new();
        let mut tier2_nohost = Vec::new();
        let mut tier3 = Vec::new();

        for tuple in TARGETS {
            let target = TargetInfo {
                tuple,
                meta: Target::expect_builtin(TargetTuple::from_tuple(target)).metadata,
            };
            let host_tools = target.meta.host_tools.unwrap_or(false);

            match target.meta.tier {
                Some(1) if host_tools => tier1_host.push(target),
                Some(1) => tier1_nohost.push(target),
                Some(2) if host_tools => tier2_host.push(target),
                Some(2) => tier2_nohost.push(target),
                Some(3) => tier3.push(target),
                tier => panic!("unknown target tier: {tier}"),
            }
        }

        tier1_host.sort_by_key(|t| t.tuple);
        tier1_nohost.sort_by_key(|t| t.tuple);
        tier2_host.sort_by_key(|t| t.tuple);
        tier2_nohost.sort_by_key(|t| t.tuple);
        tier3.sort_by_key(|t| t.tuple);

        Self { tier1_host, tier1_nohost, tier2_host, tier2_nohost, tier3 }
    }
}
