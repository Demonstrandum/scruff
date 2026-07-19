use ruff_formatter::prelude::format_with;
use ruff_python_ast::{AnyNodeRef, Expr, ExprList};
use ruff_text_size::Ranged;

use crate::expression::parentheses::{
    NeedsParentheses, OptionalParentheses, empty_parenthesized, parenthesized,
};
use crate::prelude::*;
use crate::verbatim::verbatim_text;

#[derive(Default)]
pub struct FormatExprList;

impl FormatNodeRule<ExprList> for FormatExprList {
    fn fmt_fields(&self, item: &ExprList, f: &mut PyFormatter) -> FormatResult<()> {
        let ExprList {
            range: _,
            node_index: _,
            elts,
            ctx: _,
        } = item;

        let comments = f.context().comments().clone();
        let dangling = comments.dangling(item);

        if elts.is_empty() {
            return empty_parenthesized("[", dangling, "]").fmt(f);
        }

        if f.options().is_tali_mode()
            && dangling.is_empty()
            && f.context().source().contains_line_break(item.range())
            && is_rectangular_nested_list(elts, f.context())
        {
            return verbatim_text(item).fmt(f);
        }

        let items = format_with(|f| {
            f.join_comma_separated(item.end())
                .nodes(elts.iter())
                .finish()
        });

        parenthesized("[", &items, "]")
            .with_dangling_comments(dangling)
            .fmt(f)
    }
}

fn is_rectangular_nested_list(elements: &[Expr], context: &PyFormatContext) -> bool {
    if elements.len() < 2 {
        return false;
    }

    let Some(first_shape) = nested_sequence_shape(&elements[0], context) else {
        return false;
    };

    first_shape.first().is_some_and(|columns| *columns >= 2)
        && elements[1..]
            .iter()
            .all(|element| nested_sequence_shape(element, context).as_ref() == Some(&first_shape))
}

fn nested_sequence_shape(expression: &Expr, context: &PyFormatContext) -> Option<Vec<usize>> {
    if !context
        .comments()
        .leading_dangling_trailing(expression)
        .is_empty()
    {
        return None;
    }

    let elements = match expression {
        Expr::List(list) => &list.elts,
        Expr::Tuple(tuple) => &tuple.elts,
        _ => return Some(Vec::new()),
    };

    let first = elements.first()?;
    let child_shape = nested_sequence_shape(first, context)?;
    if !elements
        .iter()
        .skip(1)
        .all(|element| nested_sequence_shape(element, context).as_ref() == Some(&child_shape))
    {
        return None;
    }

    let mut shape = Vec::with_capacity(child_shape.len() + 1);
    shape.push(elements.len());
    shape.extend(child_shape);
    Some(shape)
}

impl NeedsParentheses for ExprList {
    fn needs_parentheses(
        &self,
        _parent: AnyNodeRef,
        _context: &PyFormatContext,
    ) -> OptionalParentheses {
        OptionalParentheses::Never
    }
}
