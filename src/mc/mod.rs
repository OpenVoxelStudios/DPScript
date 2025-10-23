pub mod macros;

use crate::{cmd_enums, util::Identifier};

pub struct Function {
    pub id: Identifier,
    pub commands: Vec<Command>,
}

cmd_enums! {
    // ========== BASE ==========

    pub enum Literal {
        #[print = "{inner}"]
        Inline { inner: String },

        #[print = "$({inner})"]
        Macro { inner: String },
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
    }

    // TODO: Scoreboard
    // TODO: All the other commands lol

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
        #[print = "from {source}{source_path}"]
        From {
            source: DataSource,
            source_path: OptionLiteral,
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
        #[print = "block {pos} {path}"]
        Block { pos: BlockPos, path: Literal },

        #[print = "entity {target} {path}"]
        Entity { target: Literal, path: Literal },

        #[print = "storage {source} {path}"]
        Storage { source: Literal, path: Literal },
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
