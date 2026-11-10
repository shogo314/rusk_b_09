use crate::frontend::{
    rusk_grammar_trait::{self as g, Rusk, RuskGrammarTrait},
    text::{Declaration, LayoutHints},
};
#[derive(Default)]
pub struct RuskGrammar<'t> {
    pub root: Option<Rusk<'t>>,
    pub collect_layout: bool,
    pub layout: LayoutHints,
}
impl<'t> RuskGrammarTrait<'t> for RuskGrammar<'t> {
    fn rusk(&mut self, arg: &Rusk<'t>) -> parol_runtime::Result<()> {
        self.root = Some(arg.clone());
        Ok(())
    }

    fn type_parameters(&mut self, arg: &g::TypeParameters<'t>) -> parol_runtime::Result<()> {
        if self.collect_layout {
            self.layout
                .type_starts
                .insert(arg.identifier.identifier.location.start());
        }
        Ok(())
    }
    fn type_arguments(&mut self, arg: &g::TypeArguments<'t>) -> parol_runtime::Result<()> {
        fn start(ty: &g::TypeName<'_>) -> usize {
            match ty {
                g::TypeName::NamedType(n) => n.named_type.identifier.identifier.location.start(),
                g::TypeName::TupleType(t) => start(&t.tuple_type.type_name),
            }
        }
        if self.collect_layout {
            self.layout.type_starts.insert(start(&arg.type_name));
        }
        Ok(())
    }
    fn unary_op(&mut self, arg: &g::UnaryOp<'t>) -> parol_runtime::Result<()> {
        if self.collect_layout {
            let token = match arg {
                g::UnaryOp::Plus(v) => &v.plus,
                g::UnaryOp::Minus(v) => &v.minus,
                g::UnaryOp::Bang(v) => &v.bang,
            };
            self.layout.unary.insert(token.location.start());
        }
        Ok(())
    }
    fn negative_pattern(&mut self, arg: &g::NegativePattern<'t>) -> parol_runtime::Result<()> {
        if self.collect_layout {
            self.layout
                .negative_patterns
                .insert(arg.int_literal.int_literal.location.start());
        }
        Ok(())
    }
    fn block(&mut self, arg: &g::Block<'t>) -> parol_runtime::Result<()> {
        if self.collect_layout {
            self.layout.blocks.insert(arg.l_brace.location.start());
        }
        Ok(())
    }
    fn index(&mut self, arg: &g::Index<'t>) -> parol_runtime::Result<()> {
        if self.collect_layout {
            self.layout.indexes.insert(arg.l_bracket.location.start());
        }
        Ok(())
    }
    fn function(&mut self, arg: &g::Function<'t>) -> parol_runtime::Result<()> {
        self.declaration("function", &arg.identifier);
        Ok(())
    }
    fn extern_fn(&mut self, arg: &g::ExternFn<'t>) -> parol_runtime::Result<()> {
        self.declaration("extern function", &arg.identifier);
        Ok(())
    }
    fn struct_def(&mut self, arg: &g::StructDef<'t>) -> parol_runtime::Result<()> {
        self.declaration("struct", &arg.identifier);
        Ok(())
    }
    fn struct_field(&mut self, arg: &g::StructField<'t>) -> parol_runtime::Result<()> {
        self.declaration("field", &arg.identifier);
        Ok(())
    }
    fn enum_def(&mut self, arg: &g::EnumDef<'t>) -> parol_runtime::Result<()> {
        self.declaration("enum", &arg.identifier);
        Ok(())
    }
    fn enum_variant(&mut self, arg: &g::EnumVariant<'t>) -> parol_runtime::Result<()> {
        self.declaration("variant", &arg.identifier);
        Ok(())
    }
    fn extern_symbol(&mut self, arg: &g::ExternSymbol<'t>) -> parol_runtime::Result<()> {
        self.declaration("extern symbol", &arg.identifier);
        Ok(())
    }
    fn extern_type(&mut self, arg: &g::ExternType<'t>) -> parol_runtime::Result<()> {
        if let g::TypeName::NamedType(n) = arg.type_name.as_ref() {
            self.declaration("extern type", &n.named_type.identifier);
        }
        Ok(())
    }
}

impl RuskGrammar<'_> {
    fn declaration(&mut self, kind: &'static str, name: &g::Identifier<'_>) {
        if self.collect_layout {
            self.layout.declarations.push(Declaration {
                kind,
                name: name.identifier.text().to_string(),
                name_offset: name.identifier.location.start(),
            });
        }
    }
}
