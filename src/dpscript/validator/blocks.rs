use crate::dpscript::{
    ast::{
        ast::Scope,
        block::{BlockKind, BlockNode},
    },
    validator::{Result, Validator, err::Err},
};

impl Validator {
    pub fn validate_blocks(&mut self) -> Result<()> {
        let mut out = Vec::new();

        for mut node in self.ast.scope.blocks.clone() {
            self.validate_block(&mut node)?;
            out.push(node);
        }

        self.ast.scope.blocks = out;

        Ok(())
    }

    pub fn validate_block(&mut self, node: &mut BlockNode) -> Result<()> {
        if node.kind == BlockKind::None {
            self.errors.push(Err::UntypedBlock { span: node.span });
        }

        self.scopes.push(Scope::default());

        for item in &mut node.body {
            self.validate(item)?;
        }

        node.scope = self.scopes.pop();

        Ok(())
    }
}
