pub mod macros;

use crate::{cmd_enums, util::Identifier};

pub struct Function {
    pub id: Identifier,
    pub commands: Vec<Command>,
}

cmd_enums! {
    pub enum Literal {
        #[print = "{inner}"]
        Inline { inner: String },

        #[print = "$({inner})"]
        Macro { inner: String },
    }

    #[enum_doc = "The main command enum, holds all commands."]
    pub enum Command {
        #[print = "execute {inner}"]
        Execute { inner: ExecuteCommand },

        #[print = "tellraw {selector} {message}"]
        Tellraw { selector: Literal, message: Literal },

        #[print = "data {inner}"]
        Data { inner: DataCommand },
    }

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

        // TODO: modify, remove
    }

    // TODO: Scoreboard
    // TODO: All the other commands lol

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

        #[print = "store result {inner}"]
        StoreResult { inner: ExecuteStore },

        #[print = "store success {inner}"]
        StoreSuccess { inner: ExecuteStore },

        #[print = "align {axes}"]
        Align { axes: Literal },

        #[print = "anchored {anchor}"]
        Anchored {
            #[doc = "eyes | feet"]
            anchor: Literal,
        },

        #[print = "as {targets}"]
        As { targets: Literal },

        #[print = "at {targets}"]
        At { targets: Literal },

        #[print = "facing {pos}"]
        Facing { pos: BlockPos },

        #[print = "facing entity {targets} {anchor}"]
        FacingEntity {
            targets: Literal,

            #[doc = "eyes | feet"]
            anchor: Literal,
        },

        #[print = "in {dimension}"]
        In { dimension: Literal },

        #[print = "on {relation}"]
        On {
            #[doc = "attacker | controller | leasher | origin | owner | passengers | target | vehicle"]
            relation: Literal,
        },

        #[print = "positioned {pos}"]
        Positioned { pos: BlockPos },

        #[print = "positioned as {targets}"]
        PositionedAs { targets: Literal },

        #[print = "positioned over {heightmap}"]
        PositionedOver {
            #[doc = "world_surface | motion_blocking | motion_blocking_no_leaves | ocean_floor"]
            heightmap: Literal,
        },

        #[print = "rotated {rot}"]
        Rotated { rot: Rotation },

        #[print = "rotated as {targets}"]
        RotatedAs { targets: Literal },

        #[print = "summon {entity}"]
        Summon { entity: Literal },
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
            op: Literal, // `value | max`
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

    #[print = "{x} {y} {z}"]
    pub struct BlockPos {
        x: Literal,
        y: Literal,
        z: Literal,
    }
}
