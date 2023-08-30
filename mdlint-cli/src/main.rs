use mdlint::{all, process, RuleResult};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.iter().len() <= 1 {
        eprintln!("You need to pass args");
        std::process::exit(1)
    }

    if let Some(file) = args.get(1) {
        let rules = Some(all());
        let result = process(file, rules);
        print(result);
    } else {
        eprintln!("Provide an input file.");
        eprintln!("For instance: ./mdlint/fixtures/md014/md014_ko.md");
        std::process::exit(1)
    }
}

fn print(result: Vec<RuleResult>) {
    for x in result {
        println!("{x}\r\n");
    }
}
