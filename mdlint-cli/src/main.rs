use mdlint::{all, process, RuleResult};

fn main() {
    let file = "mdlint/fixtures/md014/md014_ko.md";
    let rules = Some(all());
    let result = process(file, rules);
    print(result);
}

fn print(result: Vec<RuleResult>) {
    for x in result {
        println!("{x}\r\n");
    }
}
