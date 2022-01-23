use std::collections::HashSet;

use anyhow::{bail, Result};

use crate::ast::{Expr, FlagField, Op};

// This is kind of inneficent, because if we have
// flags Foo(u8) {
//     Expensive = ...
//     A = Expensive | 1
//     B = Expensive | 2
// }
// Both A and B will compute expensive, but I dont care

pub fn eval(e: &FlagField, fs: &[FlagField]) -> Result<u64> {
    let mut set = HashSet::new();
    eval_with(e, fs, &mut set)
}

fn eval_with<'a>(
    e: &'a FlagField,
    fs: &'a [FlagField],
    set: &mut HashSet<&'a FlagField>,
) -> Result<u64> {
    if set.contains(e) {
        bail!("Recursive definition for {}", e.name)
    }
    set.insert(e);
    eval_exp_with(&e.expr, fs, set)
}

fn eval_exp_with<'a>(
    expr: &Expr,
    fs: &'a [FlagField],
    set: &mut HashSet<&'a FlagField>,
) -> Result<u64> {
    match expr {
        Expr::Num(n) => Ok(*n),
        Expr::BNum(n) => Ok(*n),
        Expr::BinOp(l, o, r) => get_op(*o)(eval_exp_with(l, fs, set)?, eval_exp_with(r, fs, set)?),
        Expr::Ident(name) => eval_with(lookup(name, fs)?, fs, set),
    }
}

type OpFn = fn(u64, u64) -> Result<u64>;

fn lookup<'a>(name: &str, fs: &'a [FlagField]) -> Result<&'a FlagField> {
    for i in fs {
        if i.name == name {
            return Ok(i);
        }
    }
    bail!("No definition found for {}", name);
}

macro_rules! op {
    ($op:tt) => {
        |l, r| Ok(l $op r)
    };
}

fn get_op(o: Op) -> OpFn {
    match o {
        Op::LShift => op!(<<),
        Op::RShift => op!(>>),
        Op::BAnd => op!(&),
        Op::BOr => op!(|),
        Op::BXor => op!(^),
        Op::BClear => |l, r| Ok(l & !r),
    }
}
