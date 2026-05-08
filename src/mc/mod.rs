pub mod macros;
pub mod util;

use crate::cmd_enums;
use dpscript_ast::prelude::Identifier;
use itertools::Itertools;
use std::{fmt, fs, io, path::PathBuf};

#[derive(Debug, Clone)]
pub struct Function<'a> {
    pub id: Identifier<'a>,
    pub commands: Vec<Command>,

    /// Always write this function's file, even if the content is empty.
    /// Defaults to true.
    pub always_write: bool,
}

impl<'a> Function<'a> {
    pub fn new(id: Identifier<'a>) -> Self {
        Self {
            id,
            commands: Vec::new(),
            always_write: true,
        }
    }

    pub fn write(self, out_dir: &PathBuf) -> io::Result<()> {
        if !self.always_write && self.commands.is_empty() {
            return Ok(());
        }

        let path = format!("{}/{}.mcfunction", self.id.namespace, self.id.path);
        let path = out_dir.join(path);
        let parent = path.parent().unwrap();

        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }

        let data = self
            .commands
            .into_iter()
            .map(|it| format!("{}{it}", if it.needs_macro() { "$" } else { "" }))
            .collect_vec()
            .join("\n");

        fs::write(path, data)?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ConcatLiteral(pub Vec<Literal>);

impl fmt::Display for ConcatLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.iter().map(|it| format!("{it}")).join(""))
    }
}

cmd_enums! {
    // ========== BASE ==========

    pub enum Literal {
        #[print = "{inner}"]
        Inline { inner: String },

        #[print = "$({inner})"]
        Macro { inner: String },

        #[print = "{inner}"]
        Concat { inner: ConcatLiteral },
    }

    #[enum_doc = "If this literal has a value, it will print a space before the value."]
    pub enum OptionLiteral {
        #[print = " {inner}"]
        Inline { inner: String },

        #[print = " $({inner})"]
        Macro { inner: String },

        #[print = ""]
        None {},
    }

    // ========== MAIN ==========

    #[enum_doc = "The main command enum, holds all commands."]
    pub enum Command {
        #[print = "execute {inner}"]
        Execute { inner: ExecuteCommand },

        #[print = "tellraw {selector} {message}"]
        Tellraw { selector: Literal, message: Literal },

        #[print = "data {inner}"]
        Data { inner: DataCommand },

        #[print = "scoreboard {inner}"]
        Scoreboard { inner: ScoreboardCommand },

        #[print = "return {inner}"]
        Return { inner: ReturnCommand },

        #[doc = "A custom command, not in the command struct. This should only be used for user-defined commands via facade functions."]
        #[print = "{inner}"]
        Custom { inner: Literal },

        #[print = "function {name}"]
        Function { name: Literal },

        #[print = "function {name} with {data}{path}"]
        FunctionWith {
            name: Literal,
            data: DataSource,
            path: OptionLiteral,
        },
    }

    // TODO: All the other commands lol

    // ========== RETURN ==========

    pub enum ReturnCommand {
        #[print = "{value}"]
        Value { value: Literal },

        #[print = "fail"]
        Fail {},

        #[print = "run {command}"]
        Run { command: Box<Command> },
    }

    // ========== SCOREBOARD ==========

    pub enum ScoreboardCommand {
        #[print = "objectives {inner}"]
        Objectives {
            inner: ScoreboardObjectivesCommand,
        },

        #[print = "players {inner}"]
        Players {
            inner: ScoreboardPlayersCommand,
        },
    }

    pub enum ScoreboardObjectivesCommand {
        #[print = "list"]
        List {},

        #[print = "add {objective} {criteria}{display_name}"]
        Add {
            objective: Literal,
            criteria: Literal,
            display_name: OptionLiteral,
        },

        #[print = "remove {objective}"]
        Remove {
            objective: Literal,
        },

        #[print = "setdisplay {slot} {objective}"]
        SetDisplay {
            slot: Literal,
            objective: Literal,
        },

        // TODO: modify [...]
    }

    pub enum ScoreboardPlayersCommand {
        #[print = "list {target}"]
        List {
            target: Literal,
        },

        #[print = "get {target} {objective}"]
        Get {
            target: Literal,
            objective: Literal,
        },

        #[print = "set {targets} {objective} {score}"]
        Set {
            targets: Literal,
            objective: Literal,
            score: Literal,
        },

        #[print = "add {targets} {objective} {score}"]
        Add {
            targets: Literal,
            objective: Literal,
            score: Literal,
        },

        #[print = "remove {targets} {objective} {score}"]
        Remove {
            targets: Literal,
            objective: Literal,
            score: Literal,
        },

        #[print = "reset {targets}{objective}"]
        Reset {
            targets: Literal,
            objective: OptionLiteral,
        },

        #[print = "enable {targets} {objective}"]
        Enable {
            targets: Literal,
            objective: Literal,
        },

        #[print = "operation {targets} {target_objective} {operation} {source} {source_objective}"]
        Operation {
            targets: Literal,
            target_objective: Literal,
            operation: Literal,
            source: Literal,
            source_objective: Literal,
        },

        // TODO: display [...]
    }

    // ========== DATA ==========

    pub enum DataCommand {
        #[print = "get {source} {path} {scale}"]
        Get {
            source: DataSource,
            path: Literal,
            scale: Literal,
        },

        #[print = "merge {source} {nbt}"]
        Merge {
            source: DataSource,
            nbt: Literal,
        },

        #[print = "remove {source} {path}"]
        Remove {
            source: DataSource,
            path: Literal,
        },

        #[print = "modify {source} {target_path} {action}"]
        Modify {
            source: DataSource,
            target_path: Literal,
            action: DataModifyAction,
        },
    }

    pub enum DataModifyAction {
        #[print = "append {inner}"]
        Append {
            inner: DataModifyArg,
        },

        #[print = "insert {index} {inner}"]
        Insert {
            index: Literal,
            inner: DataModifyArg,
        },

        #[print = "merge {inner}"]
        Merge {
            inner: DataModifyArg,
        },

        #[print = "prepend {inner}"]
        Prepend {
            inner: DataModifyArg,
        },

        #[print = "set {inner}"]
        Set {
            inner: DataModifyArg,
        },
    }

    pub enum DataModifyArg {
        #[print = "from {source} {source_path}"]
        From {
            source: DataSource,
            source_path: Literal,
        },

        #[print = "string {source}{source_path}{start}{end}"]
        String {
            source: DataSource,
            source_path: OptionLiteral,
            start: OptionLiteral,
            end: OptionLiteral,
        },

        #[print = "value {value}"]
        Value {
            value: Literal,
        }
    }

    pub enum DataSource {
        #[print = "block {pos}"]
        Block {
            pos: BlockPos,
        },

        #[print = "entity {target}"]
        Entity {
            target: Literal,
        },

        #[print = "storage {target}"]
        Storage {
            target: Literal,
        },
    }

    // ========== EXECUTE ==========

    pub enum ExecuteCommand {
        #[print = "if {condition} {action}"]
        If {
            condition: ExecuteIf,
            action: Box<ExecuteCommand>,
        },

        #[print = "unless {condition} {action}"]
        Unless {
            condition: ExecuteIf,
            action: Box<ExecuteCommand>,
        },

        #[print = "run {command}"]
        Run { command: Box<Command> },

        #[print = "store result {inner} {next}"]
        StoreResult {
            inner: ExecuteStore,
            next: Box<ExecuteCommand>,
        },

        #[print = "store success {inner} {next}"]
        StoreSuccess {
            inner: ExecuteStore,
            next: Box<ExecuteCommand>,
        },

        #[print = "align {axes} {next}"]
        Align {
            axes: Literal,
            next: Box<ExecuteCommand>,
        },

        #[print = "anchored {anchor} {next}"]
        Anchored {
            #[doc = "eyes | feet"]
            anchor: Literal,
            next: Box<ExecuteCommand>,
        },

        #[print = "as {targets} {next}"]
        As {
            targets: Literal,
            next: Box<ExecuteCommand>,
        },

        #[print = "at {targets} {next}"]
        At {
            targets: Literal,
            next: Box<ExecuteCommand>,
        },

        #[print = "facing {pos} {next}"]
        Facing {
            pos: BlockPos,
            next: Box<ExecuteCommand>,
        },

        #[print = "facing entity {targets} {anchor} {next}"]
        FacingEntity {
            targets: Literal,

            #[doc = "eyes | feet"]
            anchor: Literal,
            next: Box<ExecuteCommand>,
        },

        #[print = "in {dimension} {next}"]
        In {
            dimension: Literal,
            next: Box<ExecuteCommand>,
        },

        #[print = "on {relation} {next}"]
        On {
            #[doc = "attacker | controller | leasher | origin | owner | passengers | target | vehicle"]
            relation: Literal,
            next: Box<ExecuteCommand>,
        },

        #[print = "positioned {pos} {next}"]
        Positioned {
            pos: BlockPos,
            next: Box<ExecuteCommand>,
        },

        #[print = "positioned as {targets} {next}"]
        PositionedAs {
            targets: Literal,
            next: Box<ExecuteCommand>,
        },

        #[print = "positioned over {heightmap} {next}"]
        PositionedOver {
            #[doc = "world_surface | motion_blocking | motion_blocking_no_leaves | ocean_floor"]
            heightmap: Literal,
            next: Box<ExecuteCommand>,
        },

        #[print = "rotated {rot} {next}"]
        Rotated {
            rot: Rotation,
            next: Box<ExecuteCommand>,
        },

        #[print = "rotated as {targets} {next}"]
        RotatedAs {
            targets: Literal,
            next: Box<ExecuteCommand>,
        },

        #[print = "summon {entity} {next}"]
        Summon {
            entity: Literal,
            next: Box<ExecuteCommand>,
        },

        #[print = ""]
        None {},
    }

    pub enum Rotation {
        #[print = "{inner}"]
        Direct { inner: Literal },

        #[print = "{yaw} {pitch}"]
        Specific { yaw: Literal, pitch: Literal },
    }

    pub enum ExecuteStore {
        #[print = "block {target_pos} {path} {kind} {scale}"]
        Block {
            target_pos: BlockPos,
            path: Literal,
            kind: Literal,
            scale: Literal,
        },

        #[print = "bossbar {id} {op}"]
        Bossbar {
            id: Literal,

            #[doc = "value | max"]
            op: Literal,
        },

        #[print = "entity {target} {path} {kind} {scale}"]
        Entity {
            target: Literal,
            path: Literal,
            kind: Literal,
            scale: Literal,
        },

        #[print = "score {targets} {objective}"]
        Score {
            targets: Literal,
            objective: Literal,
        },

        #[print = "storage {target} {path} {kind} {scale}"]
        Storage {
            target: Literal,
            path: Literal,
            kind: Literal,
            scale: Literal,
        },
    }

    pub enum ExecuteIf {
        #[print = "biome {pos} {biome}"]
        Biome { pos: BlockPos, biome: Literal },

        #[print = "block {pos} {block}"]
        Block { pos: BlockPos, block: Literal },

        #[print = "data {inner}"]
        Data { inner: ExecuteIfData },

        #[print = "dimension {dimension}"]
        Dimension { dimension: Literal },

        #[print = "entity {target}"]
        Entity { target: Literal },

        #[print = "function {function}"]
        Function { function: Literal },

        #[print = "score {target} {target_objective} {operation} {source} {source_objective}"]
        Score {
            target: Literal,
            target_objective: Literal,
            operation: Literal,
            source: Literal,
            source_objective: Literal,
        },

        #[print = "score {target} {target_objective} matches {range}"]
        ScoreMatches {
            target: Literal,
            target_objective: Literal,
            range: Literal,
        },
    }

    pub enum ExecuteIfData {
        #[print = "block {pos} {data}"]
        Block { pos: BlockPos, data: Literal },

        #[print = "entity {target} {data}"]
        Entity { target: Literal, data: Literal },

        #[print = "storage {source} {data}"]
        Storage { source: Literal, data: Literal },
    }

    // ========== STRUCTS ==========

    #[print = "{x} {y} {z}"]
    pub struct BlockPos {
        x: Literal,
        y: Literal,
        z: Literal,
    }
}

#[cfg(test)]
pub mod tests {
    use crate::mc::{Command, ExecuteCommand, ExecuteStore, Literal};

    #[test]
    pub fn test_cmd() {
        let cmd = Command::Execute {
            inner: ExecuteCommand::StoreResult {
                inner: ExecuteStore::Score {
                    targets: Literal::Inline {
                        inner: "foo".into(),
                    },
                    objective: Literal::Inline {
                        inner: "bar".into(),
                    },
                },
                next: Box::new(ExecuteCommand::Run {
                    command: Box::new(Command::Tellraw {
                        selector: Literal::Inline {
                            inner: "foo".into(),
                        },
                        message: Literal::Inline {
                            inner: "bar".into(),
                        },
                    }),
                }),
            },
        };

        let expect = "execute store result score foo bar run tellraw foo bar";

        assert_eq!(format!("{cmd}"), expect);
    }
}
