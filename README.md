# DPScript

A fast, easy, and convenient Minecraft datapack programming language.

# Features

- Extremely comfortable syntax
- A friendly and familiar syntax
- An IR form to allow other syntaxes to grow on top of DPScript's framework
- Advanced conditionals
- Scoreboard & store manipulation
- A batteries-included standard library (WIP)
- Full array support
- Support for loops
- Compile-time constants
- Easy text components and NBT manipulation
- Functions & subroutines
- Tags
- Init & Tick blocks
- Modules
- Imports & Exports
- A language server for auto-completion (Coming Soon!)
- Support for your favorite IDEs (WIP)
- A fully-fledged package registry and build system (Coming Soon!)
- And much more!

DPScript features a fully custom tokenizer, lexer, validator, and AST to provide an extremely comfortable
syntax while being very stable. By merging all imported files into a single AST it can provide the best
error detection and validation possible. Don't worry anymore about strange issues when making datapacks!

## TODO List

### Core

- [x] Tokenizer
- [x] AST
- [x] Lexer
- [x] AST Validator
- [x] IR (DPIR)
- [x] Codegen
- [x] Build System

### Language Features

- [x] Functions
- [x] Variables
- [x] Compile-time constants
- [x] Imports
- [x] Exports
- [x] If/else statements
- [x] Subroutines & goto
- [x] `for entity in selector` loops
- [ ] `for ... in ...` loops
- [ ] `for ... in [0..3]` loop unrolling

### QoL

- [x] Syntax Highlighting
- [x] IR Syntax Highlighting
- [ ] More CLI commands and options
- [ ] Language server
- [ ] Code formatter

### Future Plans

- [ ] Package Registry
- [ ] Build system support for the package registry
- [ ] IR-level optimizer

## Credits

- Created by @KodeurKubik and @RedstoneWizard08
- Most floating-point arithmetic is done using @gibbsly's amazing [gm](https://github.com/gibbsly/gm) datapack, ported to DPScript
