use pb_ebnf::{ebnf, StaticSyntax};

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
