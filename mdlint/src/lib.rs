mod emoji;
mod parser;
mod rules;
mod ruleset;

pub use crate::rules::all;
pub use crate::ruleset::RuleResult;

use crate::ruleset::{CheckFn, RuleSet};

#[must_use]
pub fn process(path: &str, rules: Option<Vec<CheckFn>>) -> Vec<RuleResult> {
    let final_rules = if let Some(rules) = rules {
        rules
    } else {
        rules::all()
    };

    let rs = RuleSet::new(final_rules);
    rs.run(path)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = super::process("fixtures/md004/md004_ko.md", None);
        assert!(!result.is_empty());
    }
}
