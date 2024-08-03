use pb_ebnf::{DefinitionsList, Rule, Symbol, Syntax};
use pb_ebnf_macros::ebnf;

const SYNTAX: Syntax<'_> = ebnf! {
    // Rule 1
    syntax = rule_1 | rule_2, rule_3 | rule_4, {",", rule_5};
    <long meta identifier> = rule_1;
};

#[test]
fn test_syntax() {
    let rule = Rule::new(Symbol::new("rule_1"), DefinitionsList::new(&[]));
}

#[test]
fn test_syntax_kernel() {
    let kernel = SYNTAX[0].kernel().copied().collect::<Vec<_>>();
    let expected = vec![Symbol::new("rule_1")];

    assert_eq!(expected, kernel);
}
