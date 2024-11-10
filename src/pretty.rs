use crate::term::*;
use std::fmt;

/// Pretty prints a term.
pub fn pretty_print(term: &Term) -> String {
    // format!("{:?}", term)
    // TODO: Implement pretty printing for lambda calculus terms.
    let mut result = String::new();

    match term {
        Term::Var(a) => result.push_str(&format!("{}", a)),
        Term::App(a, b) => {
            let inner_expr_a = pretty_print(a);
            if inner_expr_a.contains('λ') {
                result.push_str(&format!("({}) {}", inner_expr_a, pretty_print(b)))
            } else {
                result.push_str(&format!("{} {}", a, pretty_print(b)))
            }
        }
        Term::Abs(a, b) => result.push_str(&format!("λ{}. {}", a, pretty_print(b))),
    }
    result
}

/// Display trait implementation for Term.
impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", pretty_print(self))
    }
}
