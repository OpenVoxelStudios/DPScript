use crate::dpscript::validator::{Result, Validator};
use ast::special::{SpecialData, SpecialNode};

impl<'a> Validator<'a> {
    pub fn validate_special(&mut self, node: &mut SpecialNode<'a>) -> Result<()> {
        match &node.data {
            SpecialData::Selector(_) => {} // TODO: add uuid validation & selector checking?

            SpecialData::Pos(x, y, z) => {
                let _ = x;
                let _ = y;
                let _ = z;

                // TODO: Type checking

                // let Some(xt) = x.returns(self.scope()?) else {
                //     self.errors.push(Err::CannotComputeType {
                //         span: x.span().into(),
                //     });

                //     return Ok(());
                // };

                // let Some(yt) = y.returns(self.scope()?) else {
                //     self.errors.push(Err::CannotComputeType {
                //         span: y.span().into(),
                //     });

                //     return Ok(());
                // };

                // let Some(zt) = z.returns(self.scope()?) else {
                //     self.errors.push(Err::CannotComputeType {
                //         span: z.span().into(),
                //     });

                //     return Ok(());
                // };

                // if !xt.is_numeric() {
                //     self.errors.push(Err::NonNumericPos {
                //         span: x.span().into(),
                //         got: xt,
                //     });
                // }

                // if !yt.is_numeric() {
                //     self.errors.push(Err::NonNumericPos {
                //         span: y.span().into(),
                //         got: yt,
                //     });
                // }

                // if !zt.is_numeric() {
                //     self.errors.push(Err::NonNumericPos {
                //         span: z.span().into(),
                //         got: zt,
                //     });
                // }
            }

            SpecialData::Component(_nbt) => {} // TODO: Validate the schema
        };

        Ok(())
    }
}
