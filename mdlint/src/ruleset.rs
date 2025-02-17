use crate::emoji;
use crate::parser;
use comrak::nodes::{Ast, AstNode};
use console::style;
use std::{cell::Ref, fmt};
use typed_arena::Arena;

pub(crate) type CheckFn = Box<dyn for<'a> Fn(&'a AstNode<'a>) -> RuleResult>;

pub(crate) struct RuleSet {
    pub(crate) rules: Vec<CheckFn>,
}

impl RuleSet {
    pub(crate) fn new(rules: Vec<CheckFn>) -> Self {
        Self { rules }
    }

    pub(crate) fn run(&self, file_path: &str) -> Vec<RuleResult> {
        let arena = Arena::new();
        let root = parser::get_ast(file_path, &arena);
        self.rules
            .iter()
            .map(|f| f(root))
            .filter(|r| r.details.is_some())
            .collect()
    }
}

#[derive(Debug)]
pub struct RuleResult {
    pub name: String,
    pub alias: String,
    pub description: String,
    pub details: Option<Vec<RuleResultDetails>>,
}

impl RuleResult {
    pub(crate) fn new(
        name: &str,
        alias: &str,
        description: &str,
        details: Option<Vec<RuleResultDetails>>,
    ) -> Self {
        Self {
            name: name.to_string(),
            alias: alias.to_string(),
            description: description.to_string(),
            details,
        }
    }
}

impl fmt::Display for RuleResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write!(f, "{}", self._to_string())
        let title = format!("{}/{}", self.name, self.alias);
        let mut final_str = format!(
            "{}{}\r\n{}\r\n",
            emoji::ERROR,
            style(title).bold().red(),
            style(&self.description).underlined().yellow()
        );
        if let Some(ref details) = self.details {
            for detail in details {
                final_str.push_str(&format!("\r\n{}{}", emoji::INFO, detail));
            }
        }

        write!(f, "{final_str}")
    }
}

#[derive(Debug)]
pub struct RuleResultDetails {
    pub line: u32,
    pub column: usize,
    pub content: String,
    pub previous_content: Option<String>,
    pub next_content: Option<String>,
}

impl RuleResultDetails {
    pub(crate) fn new(line: u32, column: usize, content: String) -> Self {
        Self {
            line,
            column,
            content,
            previous_content: None,
            next_content: None,
        }
    }

    pub(crate) fn from_node(node: &Ref<'_, Ast>) -> Self {
        Self::new(
            node.start_line,
            node.start_column,
            parser::content_to_string(node.content.clone()),
        )
    }

    // pub(crate) fn to_string(&self) -> String {
    //     format!("ln. {}, col. {}: {}", self.line, self.column, self.content)
    // }
}

impl fmt::Display for RuleResultDetails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            format_args!("ln. {}, col. {}: {}", self.line, self.column, self.content)
        )
        // write!(f, "{}", self.to_string())
    }
}
