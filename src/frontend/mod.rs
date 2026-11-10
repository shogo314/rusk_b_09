//! Parsing, AST construction, and source/resource loading.
pub(crate) mod ast;
pub(crate) mod documentation;
mod rusk_grammar;
pub(crate) mod source;
pub(crate) mod text;
#[allow(warnings)]
pub(crate) mod rusk_grammar_trait {
    include!(concat!(env!("OUT_DIR"), "/rusk_grammar_trait.rs"));
}
#[allow(warnings)]
mod rusk_parser {
    include!(concat!(env!("OUT_DIR"), "/rusk_parser.rs"));
}

use parol_runtime::Report;
struct ErrorReporter;
impl Report for ErrorReporter {}

pub(crate) fn parse_cst<'t>(
    text: &'t str,
    file: &str,
) -> Result<rusk_grammar_trait::Rusk<'t>, String> {
    Ok(parse_grammar(text, file, false)?
        .root
        .expect("successful parser must produce a root"))
}

pub(crate) fn parse_layout(text: &str, file: &str) -> Result<text::LayoutHints, String> {
    Ok(parse_grammar(text, file, true)?.layout)
}

fn parse_grammar<'t>(
    text: &'t str,
    file: &str,
    collect_layout: bool,
) -> Result<rusk_grammar::RuskGrammar<'t>, String> {
    let _stage = crate::diagnostics::enter(crate::diagnostics::Stage::Parsing);
    let mut grammar = rusk_grammar::RuskGrammar {
        collect_layout,
        ..Default::default()
    };
    rusk_parser::parse(text, file, &mut grammar).map_err(|e| {
        if ErrorReporter::report_error(&e, file).is_err() {
            eprintln!("{e:?}");
        }
        format!("{file}: syntax error")
    })?;
    Ok(grammar)
}
pub(crate) fn parse(text: &str, file: &str) -> Result<Vec<ast::Stmt>, String> {
    let cst = parse_cst(text, file)?;
    let _stage = crate::diagnostics::enter(crate::diagnostics::Stage::Ast);
    ast::lower(&cst)
}
