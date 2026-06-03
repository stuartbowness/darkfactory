//! Read-only shadow comparison for `darkfactory-render` (design D8).
//!
//! [`compare`] line-diffs a freshly rendered contract against an existing,
//! hand-authored one and reports whether they match. It is the safety
//! mechanism that lets the renderer be trusted before it is relied upon: an
//! operator runs `darkfactory-render shadow` against a real mission and inspects
//! the diff WITHOUT replacing anything. Cutover (wiring render into the pipeline,
//! deleting hand-authoring) is a later, gated change; shadow never auto-cutovers.
//!
//! The diff is line-based and diagnostic only, implemented in-house with no
//! external dependency. It is pure (no file IO): the CLI reads both documents
//! and hands their text here.

/// The result of a shadow comparison between a rendered contract and an existing
/// one.
///
/// [`Identical`](Diff::Identical) means the two documents are byte-for-byte
/// equal. [`Different`](Diff::Different) carries a readable line diff: lines only
/// in `existing` are prefixed `- `, lines only in `rendered` are prefixed `+ `,
/// in the order they appear. When the bytes differ but the line content is
/// identical (a trailing-newline or LF-vs-CRLF difference), `Different` instead
/// carries the explicit [`NEWLINE_ONLY_MESSAGE`] so the diff is never empty.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Diff {
    /// The rendered and existing contracts match exactly.
    Identical,
    /// The contracts differ; the `String` is a readable line diff.
    Different(String),
}

/// The explicit message carried by [`Diff::Different`] when the two documents
/// have identical line content and differ only in a trailing newline or in line
/// endings (LF vs CRLF).
///
/// The line diff is computed with [`str::lines`], which strips the trailing
/// newline and `\r`, so a newline-only difference would otherwise yield an empty
/// diff. `compare` substitutes this message instead, so the operator NEVER sees
/// a blank diff.
pub const NEWLINE_ONLY_MESSAGE: &str =
    "contents are identical; the files differ only in trailing newline or line endings";

/// Compare a freshly rendered contract against an existing one, line by line.
///
/// Returns [`Diff::Identical`] when the two texts are equal, else
/// [`Diff::Different`] carrying a readable line diff. The comparison is pure and
/// does no IO. The diff is diagnostic only: it never modifies either input and
/// implies no cutover.
pub fn compare(rendered: &str, existing: &str) -> Diff {
    if rendered == existing {
        return Diff::Identical;
    }
    let diff = line_diff(existing, rendered);
    // The bytes differ but the line diff is empty: the only difference is a
    // trailing newline or line endings (`lines()` strips both). Report it
    // explicitly so the operator never sees a blank `Different` diff.
    if diff.is_empty() {
        return Diff::Different(NEWLINE_ONLY_MESSAGE.to_string());
    }
    Diff::Different(diff)
}

/// Build a readable line diff between `existing` (old) and `rendered` (new).
///
/// Uses a longest-common-subsequence (LCS) of lines so unchanged lines align
/// and only genuinely changed lines are marked: a line present only in
/// `existing` is emitted `- <line>`, a line present only in `rendered` is emitted
/// `+ <line>`. Lines are joined with `\n`. This is an in-house diff (no external
/// crate); inputs are short contract documents, so the quadratic LCS table is
/// fine.
fn line_diff(existing: &str, rendered: &str) -> String {
    let old: Vec<&str> = existing.lines().collect();
    let new: Vec<&str> = rendered.lines().collect();

    // LCS length table: lcs[i][j] = length of the LCS of old[i..] and new[j..].
    let mut lcs = vec![vec![0usize; new.len() + 1]; old.len() + 1];
    for i in (0..old.len()).rev() {
        for j in (0..new.len()).rev() {
            lcs[i][j] = if old[i] == new[j] {
                lcs[i + 1][j + 1] + 1
            } else {
                lcs[i + 1][j].max(lcs[i][j + 1])
            };
        }
    }

    // Walk the table, emitting only the lines that fall outside the common
    // subsequence. Removed (old-only) lines lead so a changed line reads
    // `- old` then `+ new`.
    let mut out: Vec<String> = Vec::new();
    let (mut i, mut j) = (0usize, 0usize);
    while i < old.len() && j < new.len() {
        if old[i] == new[j] {
            i += 1;
            j += 1;
        } else if lcs[i + 1][j] >= lcs[i][j + 1] {
            out.push(format!("- {}", old[i]));
            i += 1;
        } else {
            out.push(format!("+ {}", new[j]));
            j += 1;
        }
    }
    while i < old.len() {
        out.push(format!("- {}", old[i]));
        i += 1;
    }
    while j < new.len() {
        out.push(format!("+ {}", new[j]));
        j += 1;
    }

    out.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_inputs_compare_identical() {
        let text = "# Contract\n\n## Task 1: Greet\n- [ ] [execution] greets\n";
        assert_eq!(compare(text, text), Diff::Identical);
    }

    #[test]
    fn a_changed_line_is_reported_as_removed_then_added() {
        let existing = "# Contract\n## Task 1: Greet\n- [ ] [execution] greets\n";
        let rendered = "# Contract\n## Task 1: Greet\n- [ ] [execution] greets loudly\n";
        let Diff::Different(diff) = compare(rendered, existing) else {
            panic!("changed inputs must compare Different");
        };
        assert!(
            diff.contains("- - [ ] [execution] greets"),
            "the existing-only line must be marked with `- `, got:\n{diff}"
        );
        assert!(
            diff.contains("+ - [ ] [execution] greets loudly"),
            "the rendered-only line must be marked with `+ `, got:\n{diff}"
        );
        // The shared lines must NOT appear in the diff.
        assert!(
            !diff.contains("# Contract"),
            "unchanged lines must not appear in the diff, got:\n{diff}"
        );
    }

    #[test]
    fn a_purely_added_line_is_marked_added_only() {
        let existing = "# Contract\n## Task 1: Greet\n";
        let rendered = "# Contract\n## Task 1: Greet\n- [ ] [execution] new check\n";
        assert_eq!(
            compare(rendered, existing),
            Diff::Different("+ - [ ] [execution] new check".to_string()),
        );
    }

    #[test]
    fn a_purely_removed_line_is_marked_removed_only() {
        let existing = "# Contract\n## Task 1: Greet\n- [ ] [execution] old check\n";
        let rendered = "# Contract\n## Task 1: Greet\n";
        assert_eq!(
            compare(rendered, existing),
            Diff::Different("- - [ ] [execution] old check".to_string()),
        );
    }

    #[test]
    fn a_trailing_newline_only_difference_is_reported_explicitly_not_empty() {
        // Line content is byte-identical; the only difference is the missing
        // final newline on `existing`. `lines()` strips it, so the line diff is
        // empty. `compare` must NEVER hand back `Different("")`.
        let rendered = "# Contract\n## Task 1: Greet\n- [ ] [execution] greets\n";
        let existing = "# Contract\n## Task 1: Greet\n- [ ] [execution] greets";
        let Diff::Different(diff) = compare(rendered, existing) else {
            panic!("a trailing-newline-only difference must compare Different");
        };
        assert!(
            !diff.is_empty(),
            "the diff must never be empty for differing inputs"
        );
        assert_eq!(diff, NEWLINE_ONLY_MESSAGE);
    }

    #[test]
    fn a_crlf_vs_lf_only_difference_is_reported_explicitly_not_empty() {
        // Same line content, different line endings (CRLF vs LF). `lines()`
        // strips the `\r`, so the line diff is empty. Must be explicit, not "".
        let rendered = "# Contract\nline two\n";
        let existing = "# Contract\r\nline two\r\n";
        assert_eq!(
            compare(rendered, existing),
            Diff::Different(NEWLINE_ONLY_MESSAGE.to_string()),
        );
    }

    #[test]
    fn unchanged_surrounding_lines_are_omitted_from_the_diff() {
        // Only the middle line changes; the LCS keeps the bookends aligned so
        // the diff names exactly two lines.
        let existing = "alpha\nbravo\ncharlie\n";
        let rendered = "alpha\nBRAVO\ncharlie\n";
        assert_eq!(
            compare(rendered, existing),
            Diff::Different("- bravo\n+ BRAVO".to_string()),
        );
    }
}
