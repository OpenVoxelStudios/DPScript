use crate::{
    mc::{
        BlockPos, Command, ConcatLiteral, DataCommand, DataModifyAction, DataModifyArg, DataSource,
        ExecuteCommand, ExecuteIf, ExecuteIfData, ExecuteStore, Literal, OptionLiteral,
        ReturnCommand, Rotation, ScoreboardCommand, ScoreboardObjectivesCommand,
        ScoreboardPlayersCommand,
    },
    util::Identifier,
};

impl Command {
    pub fn needs_macro(&self) -> bool {
        match self {
            Command::Execute { inner } => inner.needs_macro(),
            Command::Tellraw { selector, message } => selector.is_macro() || message.is_macro(),
            Command::Data { inner } => inner.needs_macro(),
            Command::Scoreboard { inner } => inner.needs_macro(),
            Command::Return { inner } => inner.needs_macro(),
            Command::Custom { inner } => inner.is_macro(),
            Command::Function { name } => name.is_macro(),
            Command::FunctionWith { name, data, path } => {
                name.is_macro() || data.needs_macro() || path.is_macro()
            }
        }
    }
}

impl ReturnCommand {
    pub fn needs_macro(&self) -> bool {
        match self {
            ReturnCommand::Value { value } => value.is_macro(),
            ReturnCommand::Fail {} => false,
            ReturnCommand::Run { command } => command.needs_macro(),
        }
    }
}

impl ScoreboardCommand {
    pub fn needs_macro(&self) -> bool {
        match self {
            ScoreboardCommand::Objectives { inner } => inner.needs_macro(),
            ScoreboardCommand::Players { inner } => inner.needs_macro(),
        }
    }
}

impl ScoreboardObjectivesCommand {
    pub fn needs_macro(&self) -> bool {
        match self {
            ScoreboardObjectivesCommand::List {} => false,
            ScoreboardObjectivesCommand::Add {
                objective,
                criteria,
                display_name,
            } => objective.is_macro() || criteria.is_macro() || display_name.is_macro(),
            ScoreboardObjectivesCommand::Remove { objective } => objective.is_macro(),
            ScoreboardObjectivesCommand::SetDisplay { slot, objective } => {
                slot.is_macro() || objective.is_macro()
            }
        }
    }
}

impl ScoreboardPlayersCommand {
    pub fn needs_macro(&self) -> bool {
        match self {
            ScoreboardPlayersCommand::List { target } => target.is_macro(),
            ScoreboardPlayersCommand::Get { target, objective } => {
                target.is_macro() || objective.is_macro()
            }
            ScoreboardPlayersCommand::Set {
                targets,
                objective,
                score,
            } => targets.is_macro() || objective.is_macro() || score.is_macro(),
            ScoreboardPlayersCommand::Add {
                targets,
                objective,
                score,
            } => targets.is_macro() || objective.is_macro() || score.is_macro(),
            ScoreboardPlayersCommand::Remove {
                targets,
                objective,
                score,
            } => targets.is_macro() || objective.is_macro() || score.is_macro(),
            ScoreboardPlayersCommand::Reset { targets, objective } => {
                targets.is_macro() || objective.is_macro()
            }
            ScoreboardPlayersCommand::Enable { targets, objective } => {
                targets.is_macro() || objective.is_macro()
            }
            ScoreboardPlayersCommand::Operation {
                targets,
                target_objective,
                operation,
                source,
                source_objective,
            } => {
                targets.is_macro()
                    || target_objective.is_macro()
                    || operation.is_macro()
                    || source.is_macro()
                    || source_objective.is_macro()
            }
        }
    }
}

impl DataCommand {
    pub fn needs_macro(&self) -> bool {
        match self {
            DataCommand::Get {
                source,
                path,
                scale,
            } => source.needs_macro() || path.is_macro() || scale.is_macro(),
            DataCommand::Merge { source, nbt } => source.needs_macro() || nbt.is_macro(),
            DataCommand::Remove { source, path } => source.needs_macro() || path.is_macro(),
            DataCommand::Modify {
                source,
                target_path,
                action,
            } => source.needs_macro() || target_path.is_macro() || action.needs_macro(),
        }
    }
}

impl DataModifyAction {
    pub fn needs_macro(&self) -> bool {
        match self {
            DataModifyAction::Append { inner } => inner.needs_macro(),
            DataModifyAction::Insert { index, inner } => index.is_macro() || inner.needs_macro(),
            DataModifyAction::Merge { inner } => inner.needs_macro(),
            DataModifyAction::Prepend { inner } => inner.needs_macro(),
            DataModifyAction::Set { inner } => inner.needs_macro(),
        }
    }
}

impl DataModifyArg {
    pub fn needs_macro(&self) -> bool {
        match self {
            DataModifyArg::From {
                source,
                source_path,
            } => source.needs_macro() || source_path.is_macro(),
            DataModifyArg::String {
                source,
                source_path,
                start,
                end,
            } => {
                source.needs_macro() || source_path.is_macro() || start.is_macro() || end.is_macro()
            }
            DataModifyArg::Value { value } => value.is_macro(),
        }
    }
}

impl DataSource {
    pub fn needs_macro(&self) -> bool {
        match self {
            DataSource::Block { pos } => pos.needs_macro(),
            DataSource::Entity { target } => target.is_macro(),
            DataSource::Storage { target } => target.is_macro(),
        }
    }
}

impl BlockPos {
    pub fn needs_macro(&self) -> bool {
        self.x.is_macro() || self.y.is_macro() || self.z.is_macro()
    }
}

impl ExecuteIf {
    pub fn needs_macro(&self) -> bool {
        match self {
            ExecuteIf::Biome { pos, biome } => pos.needs_macro() || biome.is_macro(),
            ExecuteIf::Block { pos, block } => pos.needs_macro() || block.is_macro(),
            ExecuteIf::Data { inner } => inner.needs_macro(),
            ExecuteIf::Dimension { dimension } => dimension.is_macro(),
            ExecuteIf::Entity { target } => target.is_macro(),
            ExecuteIf::Function { function } => function.is_macro(),
            ExecuteIf::Score {
                target,
                target_objective,
                operation,
                source,
                source_objective,
            } => {
                target.is_macro()
                    || target_objective.is_macro()
                    || operation.is_macro()
                    || source.is_macro()
                    || source_objective.is_macro()
            }
            ExecuteIf::ScoreMatches {
                target,
                target_objective,
                range,
            } => target.is_macro() || target_objective.is_macro() || range.is_macro(),
        }
    }
}

impl ExecuteIfData {
    pub fn needs_macro(&self) -> bool {
        match self {
            ExecuteIfData::Block { pos, data: path } => pos.needs_macro() || path.is_macro(),
            ExecuteIfData::Entity { target, data: path } => target.is_macro() || path.is_macro(),
            ExecuteIfData::Storage { source, data: path } => source.is_macro() || path.is_macro(),
        }
    }
}

impl ExecuteStore {
    pub fn needs_macro(&self) -> bool {
        match self {
            ExecuteStore::Block {
                target_pos,
                path,
                kind,
                scale,
            } => target_pos.needs_macro() || path.is_macro() || kind.is_macro() || scale.is_macro(),
            ExecuteStore::Bossbar { id, op } => id.is_macro() || op.is_macro(),
            ExecuteStore::Entity {
                target,
                path,
                kind,
                scale,
            } => target.is_macro() || path.is_macro() || kind.is_macro() || scale.is_macro(),
            ExecuteStore::Score { targets, objective } => {
                targets.is_macro() || objective.is_macro()
            }
            ExecuteStore::Storage {
                target,
                path,
                kind,
                scale,
            } => target.is_macro() || path.is_macro() || kind.is_macro() || scale.is_macro(),
        }
    }
}

impl Rotation {
    pub fn needs_macro(&self) -> bool {
        match self {
            Rotation::Direct { inner } => inner.is_macro(),
            Rotation::Specific { yaw, pitch } => yaw.is_macro() || pitch.is_macro(),
        }
    }
}

impl ExecuteCommand {
    pub fn needs_macro(&self) -> bool {
        match self {
            ExecuteCommand::If { condition, action } => {
                condition.needs_macro() || action.needs_macro()
            }
            ExecuteCommand::Unless { condition, action } => {
                condition.needs_macro() || action.needs_macro()
            }
            ExecuteCommand::Run { command } => command.needs_macro(),
            ExecuteCommand::StoreResult { inner, next } => {
                inner.needs_macro() || next.needs_macro()
            }
            ExecuteCommand::StoreSuccess { inner, next } => {
                inner.needs_macro() || next.needs_macro()
            }
            ExecuteCommand::Align { axes, next } => axes.is_macro() || next.needs_macro(),
            ExecuteCommand::Anchored { anchor, next } => anchor.is_macro() || next.needs_macro(),
            ExecuteCommand::As { targets, next } => targets.is_macro() || next.needs_macro(),
            ExecuteCommand::At { targets, next } => targets.is_macro() || next.needs_macro(),
            ExecuteCommand::Facing { pos, next } => pos.needs_macro() || next.needs_macro(),
            ExecuteCommand::FacingEntity {
                targets,
                anchor,
                next,
            } => targets.is_macro() || anchor.is_macro() || next.needs_macro(),
            ExecuteCommand::In { dimension, next } => dimension.is_macro() || next.needs_macro(),
            ExecuteCommand::On { relation, next } => relation.is_macro() || next.needs_macro(),
            ExecuteCommand::Positioned { pos, next } => pos.needs_macro() || next.needs_macro(),
            ExecuteCommand::PositionedAs { targets, next } => {
                targets.is_macro() || next.needs_macro()
            }
            ExecuteCommand::PositionedOver { heightmap, next } => {
                heightmap.is_macro() || next.needs_macro()
            }
            ExecuteCommand::Rotated { rot, next } => rot.needs_macro() || next.needs_macro(),
            ExecuteCommand::RotatedAs { targets, next } => targets.is_macro() || next.needs_macro(),
            ExecuteCommand::Summon { entity, next } => entity.is_macro() || next.needs_macro(),
            ExecuteCommand::None {} => false,
        }
    }
}

impl Literal {
    pub fn is_macro(&self) -> bool {
        match self {
            Literal::Inline { inner: _ } => false,
            Literal::Macro { inner: _ } => true,
            Literal::Concat { inner } => inner.needs_macro(),
        }
    }
}

impl ConcatLiteral {
    pub fn needs_macro(&self) -> bool {
        self.0.iter().any(|it| it.is_macro())
    }
}

impl OptionLiteral {
    pub fn is_macro(&self) -> bool {
        matches!(self, OptionLiteral::Macro { inner: _ })
    }
}

impl Into<Command> for ExecuteCommand {
    fn into(self) -> Command {
        Command::Execute { inner: self }
    }
}

impl Into<Command> for DataCommand {
    fn into(self) -> Command {
        Command::Data { inner: self }
    }
}

impl Into<Command> for ReturnCommand {
    fn into(self) -> Command {
        Command::Return { inner: self }
    }
}

impl<T: AsRef<str>> From<T> for Literal {
    fn from(value: T) -> Self {
        Self::Inline {
            inner: value.as_ref().into(),
        }
    }
}

impl From<Identifier> for Literal {
    fn from(value: Identifier) -> Self {
        value.to_string().into()
    }
}

impl<T: Into<Literal> + Copy> From<[T; 3]> for BlockPos {
    fn from(value: [T; 3]) -> Self {
        Self {
            x: value[0].into(),
            y: value[1].into(),
            z: value[2].into(),
        }
    }
}

impl<T: AsRef<str>> From<T> for OptionLiteral {
    fn from(value: T) -> Self {
        OptionLiteral::Inline {
            inner: value.as_ref().into(),
        }
    }
}

// impl<T: Into<Literal> + Clone> From<[T; 3]> for BlockPos {
//     fn from(value: [T; 3]) -> Self {
//         Self {
//             x: value[0].clone().into(),
//             y: value[1].clone().into(),
//             z: value[2].clone().into(),
//         }
//     }
// }

pub fn exec() -> ExecCommandBuilder {
    ExecCommandBuilder
}

pub fn call_func(ident: &Identifier) -> Command {
    Command::Function {
        name: format!("{ident}").into(),
    }
}

pub struct ExecCommandBuilder;

pub struct ExecIfBuilder {
    unless: bool,
    cond: Option<ExecuteIf>,
}

impl ExecCommandBuilder {
    pub fn run(&self, cmd: impl Into<Command>) -> ExecuteCommand {
        ExecuteCommand::Run {
            command: Box::new(cmd.into()),
        }
    }

    pub fn if_(&self) -> ExecIfBuilder {
        ExecIfBuilder::new(false)
    }

    pub fn unless_(&self) -> ExecIfBuilder {
        ExecIfBuilder::new(true)
    }

    pub fn as_(&self, targets: impl Into<Literal>, next: ExecuteCommand) -> ExecuteCommand {
        ExecuteCommand::As {
            targets: targets.into(),
            next: Box::new(next),
        }
    }

    pub fn at_(&self, targets: impl Into<Literal>, next: ExecuteCommand) -> ExecuteCommand {
        ExecuteCommand::At {
            targets: targets.into(),
            next: Box::new(next),
        }
    }

    pub fn pos(&self, pos: impl Into<BlockPos>, next: ExecuteCommand) -> ExecuteCommand {
        ExecuteCommand::Positioned {
            pos: pos.into(),
            next: Box::new(next),
        }
    }
}

impl ExecIfBuilder {
    pub fn new(unless: bool) -> Self {
        Self { cond: None, unless }
    }

    pub fn biome(mut self, pos: impl Into<BlockPos>, biome: impl Into<Literal>) -> Self {
        self.cond = Some(ExecuteIf::Biome {
            pos: pos.into(),
            biome: biome.into(),
        });

        self
    }

    pub fn block(mut self, pos: impl Into<BlockPos>, block: impl Into<Literal>) -> Self {
        self.cond = Some(ExecuteIf::Block {
            pos: pos.into(),
            block: block.into(),
        });

        self
    }

    pub fn data(mut self, cond: ExecuteIfData) -> Self {
        self.cond = Some(ExecuteIf::Data { inner: cond });
        self
    }

    pub fn dimension(mut self, dimension: impl Into<Literal>) -> Self {
        self.cond = Some(ExecuteIf::Dimension {
            dimension: dimension.into(),
        });

        self
    }

    pub fn entity(mut self, target: impl Into<Literal>) -> Self {
        self.cond = Some(ExecuteIf::Entity {
            target: target.into(),
        });

        self
    }

    pub fn func(mut self, func: impl Into<Literal>) -> Self {
        self.cond = Some(ExecuteIf::Function {
            function: func.into(),
        });

        self
    }

    pub fn score(
        mut self,
        target: impl Into<Literal>,
        target_objective: impl Into<Literal>,
        operation: impl Into<Literal>,
        source: impl Into<Literal>,
        source_objective: impl Into<Literal>,
    ) -> Self {
        self.cond = Some(ExecuteIf::Score {
            target: target.into(),
            target_objective: target_objective.into(),
            operation: operation.into(),
            source: source.into(),
            source_objective: source_objective.into(),
        });

        self
    }

    pub fn score_matches(
        mut self,
        target: impl Into<Literal>,
        target_objective: impl Into<Literal>,
        range: impl Into<Literal>,
    ) -> Self {
        self.cond = Some(ExecuteIf::ScoreMatches {
            target: target.into(),
            target_objective: target_objective.into(),
            range: range.into(),
        });

        self
    }

    pub fn then(self, cmd: ExecuteCommand) -> ExecuteCommand {
        if self.unless {
            ExecuteCommand::Unless {
                condition: self.cond.unwrap(),
                action: Box::new(cmd),
            }
        } else {
            ExecuteCommand::If {
                condition: self.cond.unwrap(),
                action: Box::new(cmd),
            }
        }
    }
}
