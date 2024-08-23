use std::ops::{Deref, DerefMut};

use crate::syntax::Syntax;
use crate::{DefinitionsList, IntoPrimary, Primary, Rule, SingleDefinition, Term};

use pb_bnf::definition::Definition as BnfDefinition;
use pb_bnf::definition_set::DefinitionSet as BnfDefinitionSet;
use pb_bnf::literal::Literal as BnfLiteral;
use pb_bnf::rule::Rule as BnfRule;
use pb_bnf::symbol::Symbol;
use pb_bnf::syntax::Syntax as BnfSyntax;
use pb_bnf::term::Term as BnfTerm;

impl From<Syntax> for BnfSyntax {
    fn from(value: Syntax) -> Self {
        let mut ctx = Converter::default();

        let mut_ref_ctx = &mut ctx;

        value
            .into_iter()
            .for_each(move |rule| convert_rule(rule, mut_ref_ctx));

        ctx.syntax
    }
}

#[derive(Default)]
struct Converter {
    syntax: BnfSyntax,
    anonymous_counter: u32,
}

impl Converter {
    pub fn new_anonymous_rule_name(&mut self) -> Symbol {
        let id = self.anonymous_counter;
        self.anonymous_counter += 1;
        Symbol::from(format!("a{id}"))
    }
}

impl Deref for Converter {
    type Target = BnfSyntax;

    fn deref(&self) -> &Self::Target {
        &self.syntax
    }
}

impl DerefMut for Converter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.syntax
    }
}

fn convert_rule(rule: Rule, ctx: &mut Converter) {
    let rhs = convert_definitions_list(rule.rhs, ctx);
    ctx.push(BnfRule::new(rule.lhs.into_symbol(), rhs));
}

fn convert_definitions_list(defs: DefinitionsList, ctx: &mut Converter) -> BnfDefinitionSet {
    defs.into_iter()
        .map(|def| convert_single_definition(def, ctx))
        .collect()
}

fn convert_single_definition(def: SingleDefinition, ctx: &mut Converter) -> BnfDefinition {
    def.into_iter()
        .flat_map(|term| convert_term(term, ctx))
        .collect()
}

fn convert_term(term: Term, ctx: &mut Converter) -> impl Iterator<Item = BnfTerm> {
    let n = term.repetition.unwrap_or(1);
    let term = convert_primary(term.into_primary(), ctx);

    std::iter::repeat(term).take(n as usize).flatten()
}

fn convert_primary(primary: Primary, ctx: &mut Converter) -> Option<BnfTerm> {
    match primary {
        // Convert every option [ E ] to a fresh non-terminal X and add
        // X = $\epsilon$ | E.
        // (We can convert X = A [ E ] B. to X = A E B | A B.)
        Primary::Optional(seq) => {
            let symbol = ctx.new_anonymous_rule_name();

            let mut set = convert_definitions_list(seq.into_definitions_list(), ctx);
            set.insert(0, BnfDefinition::default());

            ctx.push(BnfRule::new(symbol.clone(), set));

            Some(BnfTerm::Symbol(symbol))
        }
        // Convert every repetition { E } to a fresh non-terminal X and add
        // X = $\epsilon$ | X E.
        Primary::Repeated(seq) => {
            let symbol = ctx.new_anonymous_rule_name();

            let mut set = convert_definitions_list(seq.into_definitions_list(), ctx);

            set.iter_mut()
                .for_each(|def| def.insert(0, BnfTerm::Symbol(symbol.clone())));

            ctx.push(BnfRule::new(symbol.clone(), set));

            Some(BnfTerm::Symbol(symbol))
        }
        Primary::Grouped(seq) => {
            let symbol = ctx.new_anonymous_rule_name();
            let set = convert_definitions_list(seq.into_definitions_list(), ctx);
            ctx.push(BnfRule::new(symbol.clone(), set));
            Some(BnfTerm::Symbol(symbol))
        }
        Primary::MetaIdentifier(meta) => Some(BnfTerm::Symbol(meta.into_symbol())),
        Primary::Literal(lit) => Some(BnfTerm::Literal(BnfLiteral::from(lit.into_symbol()))),
        Primary::Empty => None,
    }
}
