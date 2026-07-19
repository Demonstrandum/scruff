use ruff_formatter::prelude::format_with;
use ruff_formatter::format_element::TextWidth;
use ruff_formatter::{FormatContext, FormatOptions};
use ruff_python_ast::{AnyNodeRef, Expr, ExprList, Number};
use ruff_source_file::LineRanges;
use ruff_text_size::{Ranged, TextRange, TextSlice};

use super::expr_number_literal::{normalize_floating_number, normalize_integer};
use crate::expression::parentheses::{
    NeedsParentheses, OptionalParentheses, empty_parenthesized, parenthesized,
};
use crate::prelude::*;

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
            && let Some(column_widths) = aligned_column_widths(elts, f.context())
        {
            return format_aligned_sequence(SequenceKind::List, elts, &column_widths, f);
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

fn aligned_column_widths(
    elements: &[Expr],
    context: &PyFormatContext,
) -> Option<Vec<u32>> {
    if elements.len() < 2 {
        return None;
    }

    let first_shape = nested_sequence_shape(&elements[0], context)?;

    if !first_shape.first().is_some_and(|columns| *columns >= 2)
        || !elements[1..]
            .iter()
            .all(|element| nested_sequence_shape(element, context).as_ref() == Some(&first_shape))
    {
        return None;
    }

    let mut rows = Vec::new();
    for element in elements {
        collect_leaf_rows(element, context, &mut rows)?;
    }

    if !rows.iter().any(|row| row_has_alignment_intent(row, context)) {
        return None;
    }

    let mut column_widths = vec![0; rows.first()?.len()];
    for row in rows {
        for (column, expression) in row.iter().enumerate() {
            column_widths[column] =
                column_widths[column].max(formatted_scalar_width(expression, context)?);
        }
    }
    Some(column_widths)
}

fn nested_sequence_shape(expression: &Expr, context: &PyFormatContext) -> Option<Vec<usize>> {
    if context
        .comments()
        .leading_dangling_trailing(expression)
        .into_iter()
        .next()
        .is_some()
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

fn collect_leaf_rows<'a>(
    expression: &'a Expr,
    context: &PyFormatContext,
    rows: &mut Vec<&'a [Expr]>,
) -> Option<()> {
    let (_, elements) = sequence_elements(expression)?;

    if elements
        .iter()
        .all(|element| formatted_scalar_width(element, context).is_some())
    {
        rows.push(elements);
        return Some(());
    }

    for element in elements {
        collect_leaf_rows(element, context, rows)?;
    }
    Some(())
}

fn row_has_alignment_intent(row: &[Expr], context: &PyFormatContext) -> bool {
    row.windows(2).any(|pair| {
        let gap = context
            .source()
            .slice(TextRange::new(pair[0].end(), pair[1].start()));
        let Some((_, after_comma)) = gap.rsplit_once(',') else {
            return false;
        };
        !after_comma.contains(['\n', '\r'])
            && (after_comma.bytes().filter(|byte| *byte == b' ').count() >= 2
                || after_comma.contains('\t'))
    })
}

fn formatted_scalar_width(expression: &Expr, context: &PyFormatContext) -> Option<u32> {
    let width = match expression {
        Expr::NumberLiteral(number) => {
            let source = context.source().slice(number);
            match number.value {
                Number::Int(_) => text_width(&normalize_integer(source), context)?,
                Number::Float(_) => text_width(&normalize_floating_number(source), context)?,
                Number::Complex { .. } => {
                    let normalized =
                        normalize_floating_number(source.trim_end_matches(['j', 'J']));
                    text_width(&std::format!("{normalized}j"), context)?
                }
            }
        }
        Expr::BooleanLiteral(boolean) => {
            if boolean.value { 4 } else { 5 }
        }
        Expr::NoneLiteral(_) => 4,
        Expr::Name(name) => text_width(name.id.as_str(), context)?,
        _ => return None,
    };
    Some(width)
}

fn text_width(text: &str, context: &PyFormatContext) -> Option<u32> {
    TextWidth::from_text(text, context.options().indent_width())
        .width()
        .map(|width| width.value())
}

#[derive(Copy, Clone)]
enum SequenceKind {
    List,
    Tuple,
}

impl SequenceKind {
    const fn delimiters(self) -> (&'static str, &'static str) {
        match self {
            Self::List => ("[", "]"),
            Self::Tuple => ("(", ")"),
        }
    }
}

fn sequence_elements(expression: &Expr) -> Option<(SequenceKind, &[Expr])> {
    match expression {
        Expr::List(list) => Some((SequenceKind::List, &list.elts)),
        Expr::Tuple(tuple) => Some((SequenceKind::Tuple, &tuple.elts)),
        _ => None,
    }
}

fn format_aligned_sequence(
    kind: SequenceKind,
    elements: &[Expr],
    column_widths: &[u32],
    f: &mut PyFormatter,
) -> FormatResult<()> {
    let (open, close) = kind.delimiters();
    let is_leaf_row = elements
        .iter()
        .all(|element| formatted_scalar_width(element, f.context()).is_some());

    token(open).fmt(f)?;
    if is_leaf_row {
        for (index, element) in elements.iter().enumerate() {
            let width = formatted_scalar_width(element, f.context())
                .expect("Aligned scalar width should have been validated");
            element.format().fmt(f)?;

            if index + 1 < elements.len() {
                token(",").fmt(f)?;
                let padding = column_widths[index] - width + 1;
                let padding = " ".repeat(padding as usize);
                text(&padding).fmt(f)?;
            }
        }
    } else {
        indent(&format_with(|f| {
            hard_line_break().fmt(f)?;
            for (index, element) in elements.iter().enumerate() {
                if index > 0 {
                    hard_line_break().fmt(f)?;
                }
                let (nested_kind, nested_elements) = sequence_elements(element)
                    .expect("Aligned nested sequence should have been validated");
                format_aligned_sequence(nested_kind, nested_elements, column_widths, f)?;
                token(",").fmt(f)?;
            }
            Ok(())
        }))
        .fmt(f)?;
        hard_line_break().fmt(f)?;
    }
    token(close).fmt(f)
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
