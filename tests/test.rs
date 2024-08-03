use pb_ebnf::{DefinitionsList, Rule, Symbol, Syntax};
use pb_ebnf_macros::ebnf;

const SYNTAX: Syntax<'_> = ebnf! {
    // Rule 1
    syntax = rule_1 | rule_2, rule_3 | rule_4, {",", rule_5};
    <long meta identifier> = rule_1;
    <literal> = <signed numeric literal> | <general literal>;
    <unsigned literal> = <unsigned numeric literal> | <general literal>;
    <general literal> = <character string literal>
        | <national character string literal>
        | <Unicode character string literal>
        | <binary string literal>
        | <datetime literal>
        | <interval literal>
        | <boolean literal>;


};

pub const SQL: Syntax = ebnf! {
    ////////////////////////////////////////
    // 5.2 Tokens and seperators (p. 134) //
    ////////////////////////////////////////

    ///////////////////////////
    // 5.3 Literal (p. 143)  //
    ///////////////////////////

    <literal> = <signed numeric literal> | <general literal>;
    <unsigned literal> = <unsigned numeric literal> | <general literal>;
    <general literal> = <character string literal>
        | <national character string literal>
        | <Unicode character string literal>
        | <binary string literal>
        | <datetime literal>
        | <interval literal>
        | <boolean literal>;

    <character string literal> = [<introducer>, <character set specification>],
        <quote>, {<character representation>}, <quote>,
        {<seperator>, <quote>, {<character representation>} ,<quote>};

    <introducer> = <underscore>;

    <character representation> = <nonquote character> | <quote symbol>;
    <quote symbol> = <quote>, <quote>;
    <national character string literal> = "N", <quote>, {<character representation>}, <quote>,
        {<separator>, <quote>, {<character representation>}, <quote>};

    <Unicode character string literal> = [<introducer>, <character set specification>],
        "U", <ampersand>, <quote>, {<Unicode representation>}, <quote>,
        {<separator>, <quote>, {<Unicode representation>}, <quote>},
        ["ESCAPE", <escape character>];

    <Unicode representation> = <character representation> | <Unicode scape value>;

    <binary string literal> = "X", <quote>, {<hexit>, <hexit>}, <quote>,
        {<separator>, <quote>, {<hexit>, <hexit>}, <quote>},
        ["ESCAPE", <escape character>];

    <hexit> = <digit> | "A" | "B" | "C" | "D" | "E" | "F" | "a" | "b" | "c" | "d" | "e" | "f";

    <signed numeric literal> = [ <sign> ], <unsigned numeric literal>;

    <unsigned numeric literal> = <exact numeric literal> | <approximate numeric literal>;

    <exact numeric literal> = <unsigned integer>, [ <period>, [ <unsigned integer> ] ]
        | <period>, <unsigned integer>;

    <sign> = <plus sign> | <minus sign>;

    <approximate numeric literal> = <mantissa>, "E", <exponent>;

    <mantissa> = <exact numeric literal>;

    <exponent> = <signed integer>;

    <signed integer> = [ <sign> ], <unsigned integer>;

    <unsigned integer> = <digit>, {<digit>};

    <datetime literal> = <date literal> | <time literal> | <timestamp literal>;

    <date literal> = "DATE", <date string>;

    <time literal> = "TIME", <time string>;

    <timestamp literal> = "TIMESTAMP", <time string>;

    <date string> = <quote>, <unquoted date string>, <quote>;

    <time string> = <quote>, <unquoted time string>, <quote>;

    <timestamp string> = <quote>, <unquoted timestamp string>, <quote>;

    <time zone interval> = <sign>, <hours value>, <colon>, <minutes value>;

    <date value> = <years value>, <minus sign>, <months value>, <minus sign>, <days value>;

    <time value> = <hours value>, <colon>, <minutes value>, <colon>, <seconds value>;

    <interval literal> = "INTERVAL", [ <sign> ], <interval string>, <interval qualifier>;

    <interval string> = <quote>, <unquoted interval string>, <quote>;

    <unquoted date string> = <date value>;

    <unquoted time string> = <time value>, [ <time zone interval> ];

    <unquoted timestamp string> = <unquoted date string>, <space>, <unquoted time string>;

    <unquoted interval string> = [ <sign> ], { <year month literal> | <day time literal> };

    <year month literal> = <years value> | [ <years value>, <minus sign> ], <months value>;

    <day time literal> = <day time interval> | <time interval>;

    <day time interval> = <days value>,
        [ <space>, <hours value>,
            [ <colon>, <minutes value>,
                [ <colon>, <seconds value> ]
            ]
        ];

    <time interval> = <hours value>, [ <colon>, <minutes value>, [ <colon>, <seconds value> ] ]
            | <minutes value>, [ <colon>, <seconds value> ]
            | <seconds value>;

    <years value> = <datetime value>;

    <months value> = <datetime value>;

    <days value> =  <datetime value>;

    <hours value> = <datetime value>;

    <minutes value> = <datetime value>;

    <seconds value> = <seconds integer value>, [ <period>, [ <seconds fraction> ] ];

    <seconds integer value> = <unsigned integer>;

    <seconds fraction> = <unsigned integer>;

    <datetime value> = <unsigned integer>;

    <boolean literal> = "TRUE" | "FALSE" | "UNKNOWN";

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
