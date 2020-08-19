//
// parser - snowflake's parser
//
// copyright (c) 2020 the snowflake authors <whiskerdev@protonmail.com>
// this source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at http://mozilla.org/MPL/2.0/.
//

use num_bigint::BigInt;

// "top level" statements that are not an expression
// while anything can be a statement, I think a goal should be that
// anything can return a value
// - @bree
#[derive(Debug, PartialEq)]
pub enum Statement {
    FnDecl {
        name: String,
        args: Vec<String>,
        body: Vec<Box<Expression>>,
    },
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    OpCall { op: OpSymbol, args: Vec<Box<Expression>> },
    FnCall { name: String, args: Vec<Expression> },
    Integer(BigInt),
    Identifier(String),
}

// named OpSymbol so it has some "genericness" for future use
// in something like macros
// - @bree
#[derive(Debug, PartialEq)]
pub enum OpSymbol {
    Plus,
    Minus,
    Star,
    ForwardSlash,
}