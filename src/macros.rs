#[macro_export]
macro_rules! check_token {
    ($cursor: ident => $var: ident[$n: expr] == $tkn: ident) => {{
        let it = $var.get($n);

        if let Some((token, span)) = it {
            if token.clone() != $crate::Token::$tkn {
                return Err($crate::LexerError {
                    src: $cursor.source(),
                    at: span.clone(),
                    err: format!(
                        "Unexpected token: {} (expected: {})",
                        token,
                        $crate::Token::$tkn
                    ),
                }
                .into());
            }
        }

        it
    }};

    (data $cursor: ident => $var: ident[$n: expr] == $tkn: ident) => {{
        let it = $var.get($n);

        if let Some((token, span)) = it {
            if let $crate::Token::$tkn(_) = token {
            } else {
                return Err($crate::LexerError {
                    src: $cursor.source(),
                    at: span.clone(),
                    err: format!("Unexpected token: {}", token),
                }
                .into());
            }
        }

        it
    }};

    (remove $cursor: ident => $var: ident[$n: expr] == $tkn: ident) => {{
        let it = $var.get($n);

        if let Some((token, span)) = it {
            if token.clone() != $crate::Token::$tkn {
                return Err($crate::LexerError {
                    src: $cursor.source(),
                    at: span.clone(),
                    err: format!(
                        "Unexpected token: {} (expected: {})",
                        token,
                        $crate::Token::$tkn
                    ),
                }
                .into());
            }
        }

        $var.remove($n)
    }};

    ($cursor: ident => $tkn: ident == $expected: ident) => {{
        if $tkn.0 != $crate::Token::$expected {
            return Err($crate::LexerError {
                src: $cursor.source(),
                at: $tkn.1.clone(),
                err: format!(
                    "Unexpected token: {} (expected: {})",
                    $tkn.0,
                    $crate::Token::$expected
                ),
            }
            .into());
        }
    }};

    (data $cursor: ident == $tkn: ident) => {{
        let it = $cursor.peek();

        if let Some((token, span)) = it {
            if let $crate::Token::$tkn(_) = token {
            } else {
                return Err($crate::LexerError {
                    src: $cursor.source(),
                    at: span.clone(),
                    err: format!("Unexpected token: {}", token),
                }
                .into());
            }
        }

        it
    }};

    ($cursor: ident == $tkn: ident) => {{
        let it = $cursor.peek();

        if let Some((token, span)) = it.clone() {
            if token != $crate::Token::$tkn {
                return Err($crate::LexerError {
                    src: $cursor.source(),
                    at: span.clone(),
                    err: format!(
                        "Unexpected token: {} (expected: {})",
                        token,
                        $crate::Token::$tkn
                    ),
                }
                .into());
            }
        }

        it
    }};

    (remove $cursor: ident == $tkn: ident) => {{
        let it = $cursor.next();

        if let Some((token, span)) = it.clone() {
            if token != $crate::Token::$tkn {
                return Err($crate::LexerError {
                    src: $cursor.source(),
                    at: span.clone(),
                    err: format!(
                        "Unexpected token: {} (expected: {})",
                        token,
                        $crate::Token::$tkn
                    ),
                }
                .into());
            }
        }

        it
    }};

    (data $cursor: ident => $tkn: ident == $expected: ident) => {{
        if let $crate::Token::$expected(_) = $tkn.0 {
        } else {
            return Err($crate::LexerError {
                src: $cursor.source(),
                at: $tkn.1.clone(),
                err: format!("Unexpected token: {}", $tkn.0),
            }
            .into());
        }
    }};
}

#[macro_export]
macro_rules! check_ir_token {
    ($cursor: ident => $var: ident[$n: expr] == $tkn: ident) => {{
        let it = $var.get($n);

        if let Some((token, span)) = it {
            if token.clone() != $crate::IRToken::$tkn {
                return Err($crate::IRLexerError {
                    src: $cursor.source(),
                    at: span.clone(),
                    err: format!(
                        "Unexpected token: {} (expected: {})",
                        token,
                        $crate::IRToken::$tkn
                    ),
                }
                .into());
            }
        }

        it
    }};

    (data $cursor: ident => $var: ident[$n: expr] == $tkn: ident) => {{
        let it = $var.get($n);

        if let Some((token, span)) = it {
            if let $crate::IRToken::$tkn(_) = token {
            } else {
                return Err($crate::IRLexerError {
                    src: $cursor.source(),
                    at: span.clone(),
                    err: format!("Unexpected token: {}", token),
                }
                .into());
            }
        }

        it
    }};

    (remove $cursor: ident => $var: ident[$n: expr] == $tkn: ident) => {{
        let it = $var.get($n);

        if let Some((token, span)) = it {
            if token.clone() != $crate::IRToken::$tkn {
                return Err($crate::IRLexerError {
                    src: $cursor.source(),
                    at: span.clone(),
                    err: format!(
                        "Unexpected token: {} (expected: {})",
                        token,
                        $crate::IRToken::$tkn
                    ),
                }
                .into());
            }
        }

        $var.remove($n)
    }};

    ($cursor: ident => $tkn: ident == $expected: ident) => {{
        if $tkn.0 != $crate::IRToken::$expected {
            return Err($crate::IRLexerError {
                src: $cursor.source(),
                at: $tkn.1.clone(),
                err: format!(
                    "Unexpected token: {} (expected: {})",
                    $tkn.0,
                    $crate::IRToken::$expected
                ),
            }
            .into());
        }
    }};

    (data $cursor: ident == $tkn: ident) => {{
        let it = $cursor.peek();

        if let Some((token, span)) = it {
            if let $crate::IRToken::$tkn(_) = token {
            } else {
                return Err($crate::IRLexerError {
                    src: $cursor.source(),
                    at: span.clone(),
                    err: format!("Unexpected token: {}", token),
                }
                .into());
            }
        }

        it
    }};

    ($cursor: ident == $tkn: ident) => {{
        let it = $cursor.peek();

        if let Some((token, span)) = it.clone() {
            if token != $crate::IRToken::$tkn {
                return Err($crate::IRLexerError {
                    src: $cursor.source(),
                    at: span.clone(),
                    err: format!(
                        "Unexpected token: {} (expected: {})",
                        token,
                        $crate::IRToken::$tkn
                    ),
                }
                .into());
            }
        }

        it
    }};

    (remove $cursor: ident == $tkn: ident) => {{
        let it = $cursor.next();

        if let Some((token, span)) = it.clone() {
            if token != $crate::IRToken::$tkn {
                return Err($crate::UnnamedLexerError {
                    src: $cursor.source(),
                    at: span.clone(),
                    err: format!(
                        "Unexpected token: {} (expected: {})",
                        token,
                        $crate::IRToken::$tkn
                    ),
                }
                .into());
            }
        }

        it
    }};

    (data $cursor: ident => $tkn: ident == $expected: ident) => {{
        if let $crate::IRToken::$expected(_) = $tkn.0 {
        } else {
            return Err($crate::IRLexerError {
                src: $cursor.source(),
                at: $tkn.1.clone(),
                err: format!("Unexpected token: {}", $tkn.0),
            }
            .into());
        }
    }};
}

#[macro_export]
macro_rules! add_return {
    ($arr: ident += $variant: ident($val: ident)) => {{
        let node = $crate::Node::$variant($val);
        $arr.push(node.clone());
        return Ok(Some(node));
    }};
}

#[macro_export]
macro_rules! add_ir_return {
    ($arr: ident += $variant: ident($val: ident)) => {{
        let node = $crate::IRNode::$variant($val);
        $arr.push(node.clone());
        return Ok(Some(node));
    }};
}

#[macro_export]
macro_rules! module_top_level_getter {
    ($fn: ident -> $ty: ident) => {
        impl $crate::Module {
            pub fn $fn(&self) -> Vec<$crate::$ty> {
                let mut items = Vec::new();

                if let Some(nodes) = &self.top_level {
                    for node in nodes {
                        if let $crate::TopLevelNode::$ty(item) = node {
                            items.push(item.clone());
                        }
                    }
                }

                items
            }
        }
    };
}

#[macro_export]
macro_rules! module_indexer_add {
    ($id: ident += ($name: ident, $module: ident)) => {
        if let Some(it) = $id.get_mut(&$name) {
            it.extend($module.$id());
        } else {
            $id.insert($name.clone(), $module.$id());
        }
    };

    ($id: ident += ($name: ident, $module: ident, &$cx: ident)) => {
        if let Some(it) = $id.get_mut(&$name) {
            it.extend($module.$id());
        } else {
            $id.insert($name.clone(), $module.$id(&$cx));
        }
    };
}

#[macro_export]
macro_rules! dump_ast_part {
    ($ast: ident.$id: ident => $dir: ident) => {
        if let Some(it) = &$ast.$id {
            let path = $dir.join(format!("{}.ron", stringify!($id)));

            fs::write(path, ron::ser::to_string_pretty(it, PrettyConfig::new())?)?;
        }
    };
}

#[macro_export]
macro_rules! command {
    ($($part: expr),*) => {
        $crate::IRCommand {
            cmd: vec![$($crate::IRNode::Literal($crate::IRLiteral::String($part.into()))),*],
        }
    };
}

#[macro_export]
macro_rules! take_until {
    ($cursor: ident, $output: ident, $span: ident => $token: ident) => {
        while let Some((tkn, sp)) = $cursor.next() {
            let mut opens = [
                0, // Brackets
                0, // Braces
                0, // Parenthesis
            ];

            if tkn == $crate::Token::$token && opens.iter().all(|it| *it == 0) {
                $span = $span.add(sp);
                break;
            }

            match tkn {
                $crate::Token::LeftBracket => opens[0] += 1,
                $crate::Token::LeftBrace => opens[1] += 1,
                $crate::Token::LeftParen => opens[2] += 1,
                $crate::Token::RightBracket => opens[0] -= 1,
                $crate::Token::RightBrace => opens[1] -= 1,
                $crate::Token::RightParen => opens[2] -= 1,
                _ => $output.push((tkn.clone(), sp)),
            };

            if opens.iter().any(|it| *it < 0) {
                return Err($crate::LexerError {
                    src: $cursor.source(),
                    at: sp,
                    err: format!("Unexpected token: {}", tkn),
                }
                .into());
            }
        }
    };
}
