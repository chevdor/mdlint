use crate::ruleset::CheckFn;
mod common_checks;
#[cfg(test)]
mod common_tests;
mod extensions;
mod md001;
mod md002;
mod md003;
mod md004;
mod md009;
mod md010;
mod md011;
mod md012;
mod md014;
mod md018;
mod md025;
mod md041;

pub fn all() -> Vec<CheckFn> {
    vec![
        Box::new(md001::check),
        Box::new(md002::check),
        Box::new(md003::check),
        Box::new(md004::check),
        Box::new(md009::check),
        Box::new(md010::check),
        Box::new(md011::check),
        Box::new(md012::check),
        Box::new(md014::check),
        Box::new(md018::check),
        Box::new(md025::check),
        Box::new(md041::check),
    ]
}
