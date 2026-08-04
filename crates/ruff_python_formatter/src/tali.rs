use std::collections::HashMap;
use std::mem::Discriminant;

use ruff_python_ast::{Stmt, Suite};
use ruff_text_size::{Ranged, TextRange, TextSize, TextSlice};

use crate::comments::{Comments, SourceComment};

pub(crate) struct HorizontalLayout {
    pub(crate) assignments: HashMap<TextSize, u16>,
    pub(crate) annotations: HashMap<TextSize, u16>,
    pub(crate) comments: HashMap<TextSize, u16>,
}

pub(crate) fn horizontal_layout(
    statements: &Suite,
    source: &str,
    comments: &Comments,
) -> HorizontalLayout {
    let mut layout = HorizontalLayout {
        assignments: HashMap::new(),
        annotations: HashMap::new(),
        comments: HashMap::new(),
    };

    align_operators(statements, source, &mut layout);
    align_comments(statements, source, comments, &mut layout);
    layout
}

#[derive(Clone, Copy)]
struct OperatorCandidate {
    statement_start: TextSize,
    line_start: TextSize,
    line_end: TextSize,
    operator_column: u32,
    adjacent_column: u32,
    source_spaces: u32,
}

fn align_operators(statements: &Suite, source: &str, layout: &mut HorizontalLayout) {
    let mut assignments = Vec::new();
    let mut annotations = Vec::new();

    for statement in statements {
        let (assignment, annotation) = operator_candidates(statement, source);
        push_operator_candidate(assignment, &mut assignments, &mut layout.assignments);
        push_operator_candidate(annotation, &mut annotations, &mut layout.annotations);
    }

    finish_operator_run(&mut assignments, &mut layout.assignments);
    finish_operator_run(&mut annotations, &mut layout.annotations);
}

fn push_operator_candidate(
    candidate: Option<OperatorCandidate>,
    run: &mut Vec<OperatorCandidate>,
    output: &mut HashMap<TextSize, u16>,
) {
    if let Some(candidate) = candidate
        && run
            .last()
            .is_none_or(|previous| candidate.line_start == previous.line_end)
    {
        run.push(candidate);
    } else {
        finish_operator_run(run, output);
        if let Some(candidate) = candidate {
            run.push(candidate);
        }
    }
}

fn finish_operator_run(run: &mut Vec<OperatorCandidate>, output: &mut HashMap<TextSize, u16>) {
    if run.len() >= 2 && run.iter().any(|candidate| candidate.source_spaces >= 2) {
        let column = run
            .iter()
            .map(|candidate| candidate.operator_column)
            .max()
            .unwrap_or_default();

        for candidate in run.drain(..) {
            let spaces = column.saturating_sub(candidate.adjacent_column).max(1);
            output.insert(
                candidate.statement_start,
                u16::try_from(spaces).unwrap_or(u16::MAX),
            );
        }
    } else {
        run.clear();
    }
}

fn operator_candidates(
    statement: &Stmt,
    source: &str,
) -> (Option<OperatorCandidate>, Option<OperatorCandidate>) {
    if source
        .slice(TextRange::new(statement.start(), statement.end()))
        .contains(['\n', '\r'])
    {
        return (None, None);
    }
    let Some(line_end) = next_line_start(statement.end(), source) else {
        return (None, None);
    };
    let line_start = source_line_start(statement.start(), source);

    match statement {
        Stmt::Assign(assign) => {
            if assign.targets.len() != 1 {
                return (None, None);
            }
            let Some(target) = assign.targets.last() else {
                return (None, None);
            };
            let Some(operator) = find_operator(target.end(), assign.value.start(), "=", source)
            else {
                return (None, None);
            };
            (
                Some(OperatorCandidate {
                    statement_start: statement.start(),
                    line_start,
                    line_end,
                    operator_column: source_column(operator, source),
                    adjacent_column: source_column(target.end(), source),
                    source_spaces: source_column(operator, source)
                        .saturating_sub(source_column(target.end(), source)),
                }),
                None,
            )
        }
        Stmt::AnnAssign(assign) => {
            let Some(colon) =
                find_operator(assign.target.end(), assign.annotation.start(), ":", source)
            else {
                return (None, None);
            };
            let annotation = OperatorCandidate {
                statement_start: statement.start(),
                line_start,
                line_end,
                operator_column: source_column(assign.annotation.start(), source),
                adjacent_column: source_column(colon + TextSize::from(1), source),
                source_spaces: source_column(assign.annotation.start(), source)
                    .saturating_sub(source_column(colon + TextSize::from(1), source)),
            };
            let assignment = assign.value.as_ref().and_then(|value| {
                let operator = find_operator(assign.annotation.end(), value.start(), "=", source)?;
                Some(OperatorCandidate {
                    statement_start: statement.start(),
                    line_start,
                    line_end,
                    operator_column: source_column(operator, source),
                    adjacent_column: source_column(assign.annotation.end(), source),
                    source_spaces: source_column(operator, source)
                        .saturating_sub(source_column(assign.annotation.end(), source)),
                })
            });
            (assignment, Some(annotation))
        }
        _ => (None, None),
    }
}

fn find_operator(start: TextSize, end: TextSize, operator: &str, source: &str) -> Option<TextSize> {
    let range = TextRange::new(start, end);
    let offset = source.slice(range).find(operator)?;
    Some(start + TextSize::try_from(offset).ok()?)
}

fn next_line_start(offset: TextSize, source: &str) -> Option<TextSize> {
    let rest = source.get(offset.to_usize()..)?;
    let next = rest
        .find('\n')
        .map_or(source.len(), |newline| offset.to_usize() + newline + 1);
    TextSize::try_from(next).ok()
}

fn source_column(offset: TextSize, source: &str) -> u32 {
    let prefix = &source[..offset.to_usize()];
    let line = prefix.rsplit_once('\n').map_or(prefix, |(_, line)| line);
    u32::try_from(line.len()).unwrap_or(u32::MAX)
}

fn source_line_start(offset: TextSize, source: &str) -> TextSize {
    let start = source[..offset.to_usize()]
        .rfind('\n')
        .map_or(0, |offset| offset + 1);
    TextSize::try_from(start).unwrap_or(TextSize::new(u32::MAX))
}

#[derive(Clone, Copy)]
struct CommentCandidate {
    family: CommentFamily,
    line_start: TextSize,
    comment_start: TextSize,
    line_end: TextSize,
    comment_column: u32,
    code_column: u32,
    source_spaces: u32,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum CommentFamily {
    Assignment,
    Other(Discriminant<Stmt>),
}

fn align_comments(
    statements: &Suite,
    source: &str,
    comments: &Comments,
    layout: &mut HorizontalLayout,
) {
    let mut run = Vec::new();

    for statement in statements {
        let candidate = comment_candidate(statement, source, comments, layout);
        if let Some(candidate) = candidate
            && run.last().is_none_or(|previous: &CommentCandidate| {
                candidate.family == previous.family && candidate.line_start == previous.line_end
            })
        {
            run.push(candidate);
        } else {
            finish_comment_run(&mut run, &mut layout.comments);
            if let Some(candidate) = candidate {
                run.push(candidate);
            }
        }
    }
    finish_comment_run(&mut run, &mut layout.comments);
}

fn comment_candidate(
    statement: &Stmt,
    source: &str,
    comments: &Comments,
    layout: &HorizontalLayout,
) -> Option<CommentCandidate> {
    let comment = comments
        .trailing(statement)
        .iter()
        .find(|comment| comment.line_position().is_end_of_line())?;
    if source
        .slice(TextRange::new(statement.start(), statement.end()))
        .contains(['\n', '\r'])
    {
        return None;
    }

    let comment_column = source_column(comment.start(), source);
    let code_column = formatted_statement_end_column(statement, source, layout);
    Some(CommentCandidate {
        family: match statement {
            Stmt::Assign(_) | Stmt::AnnAssign(_) | Stmt::AugAssign(_) => CommentFamily::Assignment,
            _ => CommentFamily::Other(std::mem::discriminant(statement)),
        },
        line_start: source_line_start(statement.start(), source),
        comment_start: comment.start(),
        line_end: next_line_start(comment.end(), source)?,
        comment_column,
        code_column,
        source_spaces: comment_column.saturating_sub(code_column),
    })
}

fn formatted_statement_end_column(
    statement: &Stmt,
    source: &str,
    layout: &HorizontalLayout,
) -> u32 {
    let start = source_column(statement.start(), source);
    match statement {
        Stmt::Assign(assign) if assign.targets.len() == 1 => {
            let target = &assign.targets[0];
            let spaces = layout
                .assignments
                .get(&statement.start())
                .copied()
                .unwrap_or(1);
            start
                .saturating_add(u32::from(target.end() - target.start()))
                .saturating_add(u32::from(spaces))
                .saturating_add(2)
                .saturating_add(u32::from(statement.end() - assign.value.start()))
        }
        Stmt::AnnAssign(assign) => {
            let annotation_spaces = layout
                .annotations
                .get(&statement.start())
                .copied()
                .unwrap_or(1);
            let mut end = start
                .saturating_add(u32::from(assign.target.end() - assign.target.start()))
                .saturating_add(1)
                .saturating_add(u32::from(annotation_spaces))
                .saturating_add(u32::from(
                    assign.annotation.end() - assign.annotation.start(),
                ));
            if let Some(value) = &assign.value {
                let assignment_spaces = layout
                    .assignments
                    .get(&statement.start())
                    .copied()
                    .unwrap_or(1);
                end = end
                    .saturating_add(u32::from(assignment_spaces))
                    .saturating_add(2)
                    .saturating_add(u32::from(value.end() - value.start()));
            }
            end
        }
        _ => source_column(statement.end(), source),
    }
}

fn finish_comment_run(run: &mut Vec<CommentCandidate>, output: &mut HashMap<TextSize, u16>) {
    if run.len() >= 2 && run.iter().any(|candidate| candidate.source_spaces >= 4) {
        let column = run
            .iter()
            .map(|candidate| {
                candidate
                    .comment_column
                    .max(candidate.code_column.saturating_add(2))
            })
            .max()
            .unwrap_or_default();
        for candidate in run.drain(..) {
            let spaces = column.saturating_sub(candidate.code_column).max(2);
            output.insert(
                candidate.comment_start,
                u16::try_from(spaces).unwrap_or(u16::MAX),
            );
        }
    } else {
        run.clear();
    }
}

pub(crate) fn comment_exceeds_line_width(
    comment: &SourceComment,
    source: &str,
    width: u16,
) -> bool {
    let Some(line_end) = source[comment.end().to_usize()..]
        .find('\n')
        .and_then(|offset| comment.end().to_usize().checked_add(offset))
    else {
        return source[source[..comment.start().to_usize()]
            .rfind('\n')
            .map_or(0, |offset| offset + 1)..]
            .trim_end()
            .len()
            > usize::from(width);
    };
    let line_start = source[..comment.start().to_usize()]
        .rfind('\n')
        .map_or(0, |offset| offset + 1);
    source[line_start..line_end].trim_end().len() > usize::from(width)
}
