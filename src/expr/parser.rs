#![allow(dead_code)]

use nom::{IResult, types::CompleteStr};
use expr::{Expr, ExprOps};

fn value<'a>(input: CompleteStr<'a>, ops: &dyn ExprOps) -> IResult<CompleteStr<'a>, CompleteStr<'a>> {
    for (i, _) in input.char_indices() {
        if (i + ops.lp().len() <= input.len() && &input[i..i + ops.lp().len()] == ops.lp()) ||
            (i + ops.rp().len() <= input.len() && &input[i..i + ops.rp().len()] == ops.rp()) ||
            (i + ops.or().len() <= input.len() && &input[i..i + ops.or().len()] == ops.or()) ||
            (i + ops.and().len() <= input.len() && &input[i..i + ops.and().len()] == ops.and()) ||
            (i + ops.not().len() <= input.len() && &input[i..i + ops.not().len()] == ops.not())
            {
                return Ok((CompleteStr(&input[i..]), CompleteStr(input[0..i].trim())));
            }
    }
    Ok((CompleteStr(""), CompleteStr(input.trim())))
}

named_args!(parens<'a>(ops: &dyn ExprOps) <CompleteStr<'a>, Expr>,
    ws!(delimited!(tag!(ops.lp()), call!(parse_expr, ops), tag!(ops.rp())))
);

named_args!(factor<'a>(ops: &dyn ExprOps) <CompleteStr<'a>, Expr>,
    alt!(
        call!(parens, ops) |
        ws!(preceded!(tag!(ops.not()), call!(parse_expr, ops))) => {|exp: Expr| Expr::Not(Box::new(exp))} |
        ws!(call!(value, ops)) => {|val: CompleteStr| Expr::Value(val.0.to_string())}
    )
);

named_args!(term<'a>(ops: &dyn ExprOps) <CompleteStr<'a>, Expr>,
    do_parse!(
        init: call!(factor, ops) >>
        res:  fold_many0!(
            pair!(tag!(ops.and()), call!(factor, ops)),
            init,
            |acc: Expr, (_, val): (CompleteStr, Expr)| {
                Expr::And(Box::new(acc), Box::new(val))
            }
        ) >>
        (res)
    )
);

named_args!(pub parse_expr<'a>(ops: &dyn ExprOps) <CompleteStr<'a>, Expr>,
    do_parse!(
        init: call!(term, ops) >>
        res: fold_many0!(
            pair!(tag!(ops.or()), call!(term, ops)),
            init,
            |acc: Expr, (_, val): (CompleteStr, Expr)| {
                Expr::Or(Box::new(acc), Box::new(val))
            }
        ) >>
        (res)
    )
);