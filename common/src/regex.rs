use pb_ebnf::{ebnf, StaticSyntax};

/// Perl-style regex syntax
pub const REGEX_SYNTAX: StaticSyntax = ebnf! {
    <regex> = <union> | <simple regex>;
    <union> = <regex>, '|', <simple regex>;
    <simple regex> = <concatenation> | <basic regex>;
    <concatenation> = <simple regex>, <basic regex>;
    <basic regex> = <star> | <plus> | <elementary regex>;
    <star> = <elementary regex>, '*';
    <plus> = <elementary regex>, '+';
    <elementary regex> = <group> | <any> | <eos> | <char> | <set>;
    <group> = '(', <regex>, ')';
    <any> = '.';
    <eos> = '$';
    <set> = <positive set> | <negative set>;
    <positive set> = '[', <set items>, ']';
    <negative set> = '[', '^', <set items>, ']';
    <set items> = <set item> | <set items>, <set item>;
    <set item> = <range> | <char>;
    <range> = <char>, '-', <char>;
};
