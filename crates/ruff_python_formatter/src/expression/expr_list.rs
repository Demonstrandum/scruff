use ruff_formatter::format_element::TextWidth;
use ruff_formatter::prelude::format_with;
use ruff_formatter::{FormatContext, FormatOptions};
use ruff_python_ast::{AnyNodeRef, Expr, ExprList, UnaryOp};
use ruff_source_file::LineRanges;
use ruff_text_size::{Ranged, TextRange, TextSlice};

use crate::comments::trailing_comments;
use crate::context::NodeLevel;
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
            && let Some(column_layouts) = aligned_column_layouts(elts, f.context())
        {
            return format_aligned_sequence(SequenceKind::List, elts, &column_layouts, f);
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

pub(super) fn aligned_column_layouts(
    elements: &[Expr],
    context: &PyFormatContext,
) -> Option<Vec<ColumnLayout>> {
    if elements.len() < 2 {
        return None;
    }

    let first_shape = nested_sequence_shape(&elements[0])?;

    if first_shape.first().is_none_or(|columns| *columns < 2)
        || !elements[1..]
            .iter()
            .all(|element| nested_sequence_shape(element).as_ref() == Some(&first_shape))
    {
        return None;
    }

    let mut rows = Vec::new();
    for element in elements {
        collect_leaf_rows(element, 1, context, &mut rows)?;
    }

    if !rows
        .iter()
        .any(|row| row_has_alignment_intent(row, context))
    {
        return None;
    }

    let mut column_layouts = Vec::with_capacity(rows.first()?.elements.len());
    for column in 0..rows[0].elements.len() {
        let mut width = 0;
        let mut left_width = 0;
        let mut right_width = 0;
        let mut all_numeric = true;

        for row in &rows {
            let metrics = formatted_scalar_metrics(&row.elements[column], context)?;
            width = width.max(metrics.width);
            if let Some(decimal) = metrics.decimal {
                left_width = left_width.max(decimal.left_width);
                right_width = right_width.max(decimal.right_width);
            } else {
                all_numeric = false;
            }
        }

        column_layouts.push(if all_numeric {
            ColumnLayout::Decimal {
                left_width,
                right_width,
            }
        } else {
            ColumnLayout::Left { width }
        });
    }
    if !aligned_rows_fit(&rows, &column_layouts, context) {
        return None;
    }
    Some(column_layouts)
}

fn nested_sequence_shape(expression: &Expr) -> Option<Vec<usize>> {
    let elements = match expression {
        Expr::List(list) => &list.elts,
        Expr::Tuple(tuple) => &tuple.elts,
        _ => return Some(Vec::new()),
    };

    let first = elements.first()?;
    let child_shape = nested_sequence_shape(first)?;
    if !elements
        .iter()
        .skip(1)
        .all(|element| nested_sequence_shape(element).as_ref() == Some(&child_shape))
    {
        return None;
    }

    let mut shape = Vec::with_capacity(child_shape.len() + 1);
    shape.push(elements.len());
    shape.extend(child_shape);
    Some(shape)
}

struct LeafRow<'a> {
    expression: &'a Expr,
    elements: &'a [Expr],
    depth: u32,
}

fn collect_leaf_rows<'a>(
    expression: &'a Expr,
    depth: u32,
    context: &PyFormatContext,
    rows: &mut Vec<LeafRow<'a>>,
) -> Option<()> {
    let (_, elements) = sequence_elements(expression)?;

    if elements
        .iter()
        .all(|element| formatted_scalar_metrics(element, context).is_some())
    {
        let comments = context.comments().leading_dangling_trailing(expression);
        if !comments.leading.is_empty()
            || !comments.dangling.is_empty()
            || comments
                .trailing
                .iter()
                .any(|comment| comment.line_position().is_own_line())
        {
            return None;
        }
        rows.push(LeafRow {
            expression,
            elements,
            depth,
        });
        return Some(());
    }

    if context.comments().has(expression) {
        return None;
    }

    for element in elements {
        collect_leaf_rows(element, depth.saturating_add(1), context, rows)?;
    }
    Some(())
}

fn aligned_rows_fit(
    rows: &[LeafRow],
    column_layouts: &[ColumnLayout],
    context: &PyFormatContext,
) -> bool {
    let columns_width = column_layouts
        .iter()
        .map(|layout| layout.width())
        .fold(0u32, u32::saturating_add);
    let separators_width = u32::try_from(column_layouts.len().saturating_sub(1))
        .unwrap_or(u32::MAX)
        .saturating_mul(2);
    // Account for both delimiters and the trailing comma on each row.
    let row_width = columns_width
        .saturating_add(separators_width)
        .saturating_add(3);
    let indent_width = context.options().indent_width().value();
    let statement_indent = u32::from(
        context
            .indent_level()
            .to_ascii_spaces(context.options().indent_width()),
    );
    let line_width = u32::from(context.options().line_width().value());

    rows.iter().all(|row| {
        statement_indent
            .saturating_add(row.depth.saturating_mul(indent_width))
            .saturating_add(row_width)
            <= line_width
    })
}

fn row_has_alignment_intent(row: &LeafRow, context: &PyFormatContext) -> bool {
    row.elements.windows(2).any(|pair| {
        let gap = context
            .source()
            .slice(TextRange::new(pair[0].end(), pair[1].start()));
        let Some((before_comma, after_comma)) = gap.rsplit_once(',') else {
            return false;
        };
        !after_comma.contains(['\n', '\r'])
            && (before_comma.ends_with([' ', '\t'])
                || after_comma.bytes().filter(|byte| *byte == b' ').count() >= 2
                || after_comma.contains('\t'))
    }) || row_comment_has_alignment_intent(row.expression, context)
}

fn row_comment_has_alignment_intent(row: &Expr, context: &PyFormatContext) -> bool {
    let Some(comment) = context.comments().trailing(row).first() else {
        return false;
    };
    let gap = context
        .source()
        .slice(TextRange::new(row.end(), comment.start()));
    let after_comma = gap.rsplit_once(',').map_or(gap, |(_, after)| after);
    !after_comma.contains(['\n', '\r'])
        && (after_comma.bytes().filter(|byte| *byte == b' ').count() > 3
            || after_comma.contains('\t'))
}

#[derive(Copy, Clone)]
struct ScalarMetrics {
    width: u32,
    decimal: Option<DecimalMetrics>,
}

#[derive(Copy, Clone)]
struct DecimalMetrics {
    left_width: u32,
    right_width: u32,
}

fn formatted_scalar_metrics(expression: &Expr, context: &PyFormatContext) -> Option<ScalarMetrics> {
    if matches!(expression, Expr::List(_) | Expr::Tuple(_))
        || context.source().contains_line_break(expression.range())
        || context.comments().contains_comments(expression.into())
    {
        return None;
    }

    let formatted = formatted_scalar(expression, context)?;
    if is_numeric_scalar(expression) {
        numeric_metrics(&formatted, context)
    } else {
        Some(ScalarMetrics {
            width: text_width(&formatted, context)?,
            decimal: None,
        })
    }
}

fn formatted_scalar(expression: &Expr, context: &PyFormatContext) -> Option<String> {
    let mut isolated_context = PyFormatContext::new(
        context.options().clone(),
        context.source(),
        context.comments().clone(),
        context.trivia(),
        context.tokens(),
    );
    isolated_context.set_node_level(NodeLevel::ParenthesizedExpression);

    let formatted = ruff_formatter::format!(isolated_context, [expression.format()]).ok()?;
    let printed = formatted.print().ok()?;
    let code = printed.as_code();
    (!code.contains(['\n', '\r'])).then(|| code.to_owned())
}

fn is_numeric_scalar(expression: &Expr) -> bool {
    match expression {
        Expr::NumberLiteral(_) => true,
        Expr::UnaryOp(unary) if matches!(unary.op, UnaryOp::UAdd | UnaryOp::USub) => {
            matches!(unary.operand.as_ref(), Expr::NumberLiteral(_))
        }
        _ => false,
    }
}

fn numeric_metrics(text: &str, context: &PyFormatContext) -> Option<ScalarMetrics> {
    let width = text_width(text, context)?;
    let (left, right) = text
        .find('.')
        .map_or((text, ""), |decimal| (&text[..decimal], &text[decimal..]));
    Some(ScalarMetrics {
        width,
        decimal: Some(DecimalMetrics {
            left_width: text_width(left, context)?,
            right_width: text_width(right, context)?,
        }),
    })
}

#[derive(Copy, Clone)]
pub(super) enum ColumnLayout {
    Left { width: u32 },
    Decimal { left_width: u32, right_width: u32 },
}

impl ColumnLayout {
    const fn width(self) -> u32 {
        match self {
            Self::Left { width } => width,
            Self::Decimal {
                left_width,
                right_width,
            } => left_width.saturating_add(right_width),
        }
    }

    const fn leading_padding(self, metrics: ScalarMetrics) -> u32 {
        match (self, metrics.decimal) {
            (
                Self::Decimal { left_width, .. },
                Some(DecimalMetrics {
                    left_width: cell_left,
                    ..
                }),
            ) => left_width.saturating_sub(cell_left),
            (Self::Left { .. } | Self::Decimal { .. }, _) => 0,
        }
    }

    const fn trailing_padding(self, metrics: ScalarMetrics) -> u32 {
        match (self, metrics.decimal) {
            (Self::Left { width }, _) => width.saturating_sub(metrics.width),
            (
                Self::Decimal { right_width, .. },
                Some(DecimalMetrics {
                    right_width: cell_right,
                    ..
                }),
            ) => right_width.saturating_sub(cell_right),
            (Self::Decimal { .. }, None) => 0,
        }
    }
}

fn text_width(text: &str, context: &PyFormatContext) -> Option<u32> {
    TextWidth::from_text(text, context.options().indent_width())
        .width()
        .map(ruff_formatter::format_element::Width::value)
}

#[derive(Copy, Clone)]
pub(super) enum SequenceKind {
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

pub(super) fn format_aligned_sequence(
    kind: SequenceKind,
    elements: &[Expr],
    column_layouts: &[ColumnLayout],
    f: &mut PyFormatter,
) -> FormatResult<()> {
    let (open, close) = kind.delimiters();
    let is_leaf_row = elements
        .iter()
        .all(|element| formatted_scalar_metrics(element, f.context()).is_some());

    token(open).fmt(f)?;
    if is_leaf_row {
        for (index, element) in elements.iter().enumerate() {
            let metrics = formatted_scalar_metrics(element, f.context())
                .expect("Aligned scalar width should have been validated");
            let layout = column_layouts[index];
            let leading_padding = " ".repeat(layout.leading_padding(metrics) as usize);
            text(&leading_padding).fmt(f)?;
            element.format().fmt(f)?;
            let trailing_padding = " ".repeat(layout.trailing_padding(metrics) as usize);
            text(&trailing_padding).fmt(f)?;

            if index + 1 < elements.len() {
                token(",").fmt(f)?;
                space().fmt(f)?;
            }
        }
    } else {
        indent(&format_with(|f: &mut PyFormatter| {
            let comments = f.context().comments().clone();
            hard_line_break().fmt(f)?;
            for (index, element) in elements.iter().enumerate() {
                if index > 0 {
                    hard_line_break().fmt(f)?;
                }
                let (nested_kind, nested_elements) = sequence_elements(element)
                    .expect("Aligned nested sequence should have been validated");
                format_aligned_sequence(nested_kind, nested_elements, column_layouts, f)?;
                token(",").fmt(f)?;
                trailing_comments(comments.trailing(element)).fmt(f)?;
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
