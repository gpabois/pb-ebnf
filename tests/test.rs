use std::collections::HashSet;

use pb_ebnf::{ebnf, StaticSyntax, SymbolRef};

const BASIC_SYNTAX: StaticSyntax = ebnf! {
    <Lines> = <Lines>, <Line> | <Line>;
    <Line> = <Integer>, <Statements>, <NewLine>;

    <Statements> = <Statements>, ":", <Statement> | <Statement>;

    <Statement> = "CLOSE", <Integer>
        | "CLR"
        | "CMD",  <Expression>
        | "CONT"
        | "DATA", <Constant List>
        | "DEF", <FunctionID>, "(", <ID List>, ")", "=", <Expression>
        | "DIM", "ID", "(", <Expression List>, ")"
        | "END"
        | "FOR", "ID", "=", <Expression>, "TO", <Expression>, <Step Opt>
        | "GET", "ID"
        | "GET", "#", <Integer>, ",", "ID"
        | "GOSUB", <Expression>
        | "GOTO", <Expression>
        | "IF", <Expression>, "THEN", <Then Clause>
        | "INPUT", <ID List>
        | "INPUT", "#", <Integer>, ",", <ID List>
        | "LET", "ID", "=", <Expression>
        | "LIST", <Line Range>
        | "LOAD", <Value List>
        | "ID", "=", <Expression>
        | "NEW"
        | "NEXT", <ID List>
        | "ON", "ID", "GOTO", <Expression List>
        | "OPEN", <Expression List>
        | "POKE", <Expression>, ",", <Expression>
        | "PRINT", <Print List>
        | "PRINT", "#", <Integer>, ",", <Print List>
        | "READ", <ID List>
        | "RETURN"
        | "RESTORE"
        | "RUN"
        | "RUN", <Expression>
        | "STOP"
        | "SYS", <Expression>
        | "WAIT", <Expression List>
        | "VERIFY", <Expression List>
        | <Remark>;

    <Step Opt> = "STEP", <Expression> | ;
    <ID List>  = "ID", ",", <ID List> | "ID";
    <Value List> = <Value>, ',', <Value List> | <Value>;
    <Constant List> = <Constant>, ',', <Constant List> | <Constant>;
    <Expression List> = <Expression>, ',', <Expression List> | <Expression>;
    <Print List> = <Expression>, ';', <Print List> | <Expression> |;
    <Line Range> = <Integer> | <Integer>, '-' | <Integer>, '-', <Integer>;
    <Then Clause> = <Integer> | <Statement>;

    <Expression> = <And Exp>, "OR", <Expression> | <And Exp>;
    <And Exp> = <Not Exp>, "AND", <And Exp> | <Not Exp>;
    <Not Exp> = "NOT", <Compare Exp> | <Compare Exp>;
    <Compare Exp> = <Add Exp>, '=',  <Compare Exp>
        | <Add Exp>, "<>", <Compare Exp>
        | <Add Exp>, '>',  <Compare Exp>
        | <Add Exp>, ">=", <Compare Exp>
        | <Add Exp>, '<',  <Compare Exp>
        | <Add Exp>, "<=", <Compare Exp>
        | <Add Exp>;

    <Add Exp> = <Mult Exp>, '+', <Add Exp>
        | <Mult Exp>, '-', <Add Exp>
        | <Mult Exp>;
    <Mult Exp> = <Negate Exp>, '*', <Mult Exp>
        | <Negate Exp>, '/', <Mult Exp>
        | <Negate Exp>;

    <Negate Exp> = '-', <Power Exp>
        | <Power Exp>;

    <Power Exp> = <Power Exp>, '^', <Sub Exp>
        | <Sub Exp>;

    <Sub Exp> = '(', <Expression>, ')'
        | <Value>;

    <Value> = "ID"
        | "ABS", '(', <Expression>, ')'
        | "ASC", '(', <Expression>, ')'
        | "ATN", '(', <Expression>, ')'
        | "CHR$",'(', <Expression>, ')'
        | "COS", '(', <Expression>, ')'
        | "EXP", '(', <Expression>, ')'
        | "FunctionID", '(', <Expression List>, ')'
        | "FRE", '(', <Value>, ')'
        | "INT", '(', <Expression>, ')'
        | "LEFT$", '(', <Expression>, ',', <Expression>, ')'
        | "LEN", '(', <Expression>, ')'
        | "PEEK", '(', <Expression>, ')'
        | "POS", '(', <Value>, ')'
        | "RIGHT$", '(', <Expression>, ',', <Expression>, ')'
        | "RND", '(', <Expression>, ')'
        | "SGN", '(', <Expression>, ')'
        | "SPC", '(', <Expression>, ')'
        | "SQR", '(', <Expression>, ')'
        | "TAB", '(', <Expression>, ')'
        | "TAN", '(', <Expression>, ')'
        | "VAL", '(', <Expression>, ')'
        | <Constant>;

    <Constant> = <Integer> | <String> | <Real>;
};

const BASIC_NON_TERMINALS: &[SymbolRef<'static>] = &[
    SymbolRef::new("Lines"),
    SymbolRef::new("Line"),
    SymbolRef::new("Statements"),
    SymbolRef::new("Statement"),
    SymbolRef::new("Step Opt"),
    SymbolRef::new("ID List"),
    SymbolRef::new("Value List"),
    SymbolRef::new("Constant List"),
    SymbolRef::new("Expression List"),
    SymbolRef::new("Print List"),
    SymbolRef::new("Line Range"),
    SymbolRef::new("Then Clause"),
    SymbolRef::new("Expression"),
    SymbolRef::new("And Exp"),
    SymbolRef::new("Not Exp"),
    SymbolRef::new("Compare Exp"),
    SymbolRef::new("Add Exp"),
    SymbolRef::new("Mult Exp"),
    SymbolRef::new("Negate Exp"),
    SymbolRef::new("Power Exp"),
    SymbolRef::new("Sub Exp"),
    SymbolRef::new("Value"),
    SymbolRef::new("Constant"),
];

const BASIC_TERMINALS: &[SymbolRef<'static>] = &[
    SymbolRef::new("Integer"),
    SymbolRef::new("String"),
    SymbolRef::new("Real"),
    SymbolRef::new(":"),
    SymbolRef::new("NewLine"),
    SymbolRef::new("CLOSE"),
    SymbolRef::new("CLR"),
    SymbolRef::new("CMD"),
    SymbolRef::new("CONT"),
    SymbolRef::new("DATA"),
    SymbolRef::new("DEF"),
    SymbolRef::new("FunctionID"),
    SymbolRef::new("("),
    SymbolRef::new(")"),
    SymbolRef::new("="),
    SymbolRef::new("DIM"),
    SymbolRef::new("ID"),
    SymbolRef::new("END"),
    SymbolRef::new("FOR"),
    SymbolRef::new("TO"),
    SymbolRef::new("GET"),
    SymbolRef::new("#"),
    SymbolRef::new(","),
    SymbolRef::new("GOSUB"),
    SymbolRef::new("GOTO"),
    SymbolRef::new("IF"),
    SymbolRef::new("THEN"),
    SymbolRef::new("INPUT"),
    SymbolRef::new("LET"),
    SymbolRef::new("LIST"),
    SymbolRef::new("LOAD"),
    SymbolRef::new("NEW"),
    SymbolRef::new("NEXT"),
    SymbolRef::new("ON"),
    SymbolRef::new("OPEN"),
    SymbolRef::new("POKE"),
    SymbolRef::new("PRINT"),
    SymbolRef::new("READ"),
    SymbolRef::new("RETURN"),
    SymbolRef::new("RESTORE"),
    SymbolRef::new("RUN"),
    SymbolRef::new("STOP"),
    SymbolRef::new("SYS"),
    SymbolRef::new("WAIT"),
    SymbolRef::new("VERIFY"),
    SymbolRef::new("Remark"),
    SymbolRef::new("STEP"),
    SymbolRef::new("-"),
    SymbolRef::new("OR"),
    SymbolRef::new("AND"),
    SymbolRef::new("NOT"),
    SymbolRef::new("="),
    SymbolRef::new("<>"),
    SymbolRef::new(">"),
    SymbolRef::new(">="),
    SymbolRef::new("<"),
    SymbolRef::new("<="),
    SymbolRef::new("+"),
    SymbolRef::new("*"),
    SymbolRef::new("/"),
    SymbolRef::new("^"),
    SymbolRef::new("ABS"),
    SymbolRef::new("ASC"),
    SymbolRef::new("ATN"),
    SymbolRef::new("CHR$"),
    SymbolRef::new("COS"),
    SymbolRef::new("EXP"),
    SymbolRef::new("FRE"),
    SymbolRef::new("INT"),
    SymbolRef::new("LEFT$"),
    SymbolRef::new("LEN"),
    SymbolRef::new("PEEK"),
    SymbolRef::new("POS"),
    SymbolRef::new("RIGHT$"),
    SymbolRef::new("RND"),
    SymbolRef::new("SGN"),
    SymbolRef::new("SPC"),
    SymbolRef::new("SQR"),
    SymbolRef::new("TAB"),
    SymbolRef::new("TAN"),
    SymbolRef::new("VAL"),
    SymbolRef::new(";"),
];

#[test]
fn test_terminals() {
    let terminals = BASIC_SYNTAX.iter_terminals().collect::<HashSet<_>>();
    let expected = BASIC_TERMINALS.iter().copied().collect::<HashSet<_>>();

    assert_eq!(terminals, expected, "{:?}", terminals.difference(&expected));
}

#[test]
fn test_non_terminals() {
    let non_terminals = BASIC_SYNTAX.iter_non_terminals().collect::<HashSet<_>>();
    let expected = BASIC_NON_TERMINALS.iter().copied().collect::<HashSet<_>>();

    assert_eq!(
        non_terminals,
        expected,
        "{:?}",
        non_terminals.difference(&expected)
    );
}
