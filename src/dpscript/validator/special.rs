use crate::{
    common::traits::HasSpan,
    dpscript::{
        ast::special::{SpecialData, SpecialNode},
        data::NodeInfo,
        validator::{Result, Validator, err::Err},
    },
};

impl Validator {
    pub fn validate_special(&mut self, node: &mut SpecialNode) -> Result<()> {
        match &node.data {
            SpecialData::Selector(_) => {} // TODO: add uuid validation & selector checking?

            SpecialData::Pos(x, y, z) => {
                let Some(xt) = x.returns(self.scope()?) else {
                    self.errors.push(Err::CannotComputeType { span: x.span() });

                    return Ok(());
                };

                let Some(yt) = y.returns(self.scope()?) else {
                    self.errors.push(Err::CannotComputeType { span: y.span() });

                    return Ok(());
                };

                let Some(zt) = z.returns(self.scope()?) else {
                    self.errors.push(Err::CannotComputeType { span: z.span() });

                    return Ok(());
                };

                if !xt.is_numeric() {
                    self.errors.push(Err::NonNumericPos {
                        span: x.span(),
                        got: xt,
                    });
                }

                if !yt.is_numeric() {
                    self.errors.push(Err::NonNumericPos {
                        span: y.span(),
                        got: yt,
                    });
                }

                if !zt.is_numeric() {
                    self.errors.push(Err::NonNumericPos {
                        span: z.span(),
                        got: zt,
                    });
                }
            }

            SpecialData::Component(_nbt) => {} // TODO: Validate the schema
        };

        Ok(())
    }
}
