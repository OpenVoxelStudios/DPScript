// deno-lint-ignore-file no-unused-vars

/**
 * @file A fast, easy, and convenient Minecraft datapack programming language.
 * @author RedstoneWizard08
 * @license MIT
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

/**
 * @param {RuleOrLiteral} rule
 * @param {RuleOrLiteral} sep
 * @returns {ChoiceRule}
 */
const sepRepeat = (rule, sep) => optional(seq(rule, repeat(seq(sep, rule))));

export default grammar({
    name: "dpscript",

    extras: ($) => [
        /\s/,
        $.comment,
    ],

    rules: {
        // TODO: add the actual grammar rules

        source_file: ($) => repeat($.def),

        def: ($) => choice($.function_def, $.objective, $.constant),
        expr: ($) => choice($.literal, $.array, $.range_expr, $.unary_expr, $.bin_expr, $.call),
        body: ($) => choice($.variable, $.constant, $.bin_expr_assign, seq($.call, ";")),

        function_def: ($) =>
            seq(
                field("attrs", repeat($.attr)),
                field("modifiers", repeat($.fn_mod)),
                "fn",
                field("name", $.ident),
                "(",
                field("args", optional(sepRepeat($.fn_arg, ","))),
                ")",
                field("return_type", optional(seq("-", ">", $.type))),
                choice(
                    field("body", $.block),
                    ";",
                ),
            ),

        objective: ($) =>
            seq(
                field("attrs", repeat($.attr)),
                field("modifiers", optional(choice("pub"))),
                "objective",
                field("name", $.ident),
                ":",
                field("criteria", $.ident),
                "=",
                field("name", $.str),
                ";",
            ),

        variable: ($) =>
            seq(
                "let",
                field("name", $.ident),
                field("type", optional(seq(":", $.type))),
                field("value", optional(seq("=", $.expr))),
                ";",
            ),

        constant: ($) =>
            seq(
                "const",
                field("name", $.ident),
                optional(seq(":", field("type", $.type))),
                "=",
                field("value", $.expr),
                ";",
            ),

        call: ($) => seq(field("target", $.expr), "(", field("args", sepRepeat($.expr, ",")), ")"),

        range_expr: ($) =>
            prec.left(choice(
                seq(field("start", $.expr), "..", field("end", $.expr)),
                seq(field("start", $.expr), ".."),
                seq("..", field("end", $.expr)),
            )),

        unary_expr: ($) =>
            prec.left(choice(
                seq("(", $.expr, ")"),
                seq("!", $.expr),
                seq("-", $.expr),
            )),

        bin_expr: ($) =>
            choice(
                prec.left(11, seq($.expr, ".", $.expr)),
                prec.left(10, seq($.expr, "*", $.expr)),
                prec.left(10, seq($.expr, "/", $.expr)),
                prec.left(9, seq($.expr, "+", $.expr)),
                prec.left(9, seq($.expr, "-", $.expr)),
                prec.left(8, seq($.expr, "<<", $.expr)),
                prec.left(8, seq($.expr, ">>", $.expr)),
                prec.left(7, seq($.expr, "&", $.expr)),
                prec.left(6, seq($.expr, "^", $.expr)),
                prec.left(5, seq($.expr, "|", $.expr)),
                prec.left(4, seq($.expr, "==", $.expr)),
                prec.left(4, seq($.expr, "!=", $.expr)),
                prec.left(4, seq($.expr, "<", $.expr)),
                prec.left(4, seq($.expr, ">", $.expr)),
                prec.left(4, seq($.expr, "<=", $.expr)),
                prec.left(4, seq($.expr, ">=", $.expr)),
                prec.left(3, seq($.expr, "&&", $.expr)),
                prec.left(2, seq($.expr, "||", $.expr)),
            ),

        bin_expr_assign: ($) =>
            choice(
                prec.left(10, seq($.expr, "=", $.expr)),
                prec.left(10, seq($.expr, "+=", $.expr)),
                prec.left(10, seq($.expr, "-=", $.expr)),
                prec.left(10, seq($.expr, "*=", $.expr)),
                prec.left(10, seq($.expr, "/=", $.expr)),
                prec.left(10, seq($.expr, "%=", $.expr)),
                prec.left(10, seq($.expr, "&=", $.expr)),
                prec.left(10, seq($.expr, "|=", $.expr)),
                prec.left(10, seq($.expr, "^=", $.expr)),
                prec.left(10, seq($.expr, "<<=", $.expr)),
                prec.left(10, seq($.expr, ">>=", $.expr)),
            ),

        array: ($) => seq("[", sepRepeat($.expr, ","), "]"),

        fn_arg: ($) => seq(field("attrs", repeat($.attr)), field("name", $.ident), ":", field("type", $.type)),
        fn_mod: ($) => choice("pub", "facade", "inline", "instance", "compiler"),

        block: ($) => seq("{", repeat($.body), "}"),

        attr: ($) =>
            seq(
                "#",
                "[",
                choice(
                    seq(field("name", $.ident), "=", field("value", $.literal)),
                    seq(field("name", $.ident), "(", sepRepeat(field("value", $.literal), ","), ")"),
                    field("name", $.ident),
                ),
                "]",
            ),

        type: ($) => /[A-Za-z_][A-Za-z0-9_]*/, // Right now there are no generics.
        literal: ($) => choice($.number, $.bool, $.str, $.ident),
        ident: ($) => /[A-Za-z_][A-Za-z0-9_]*/,
        number: ($) => /\d+\s*(\.\s*\d+)?[f|d|b]?/,
        bool: ($) => choice("true", "false"),
        str: ($) => /"[^"]*"/,

        comment: ($) => token(choice(seq("//", /.*/), seq("/*", /[^*]*\*+([^/*][^*]*\*+)*/, "/"))),
    },
});
