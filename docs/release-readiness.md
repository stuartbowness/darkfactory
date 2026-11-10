# MIT release preparation

Prepared on 2026-11-10 against main commit `940d77d`.

## Changes and evidence

- Added MIT licensing and upstream attribution notices.
- Reworked the introduction, architecture, reviewer reading path, and limitations.
- Added a credential-free demo using the real dispatcher and fixture subprocesses.
- Replaced destructive installer/migration behaviour with destination preflight,
  dry-run support, and eight disposable-directory tests.
- Fixed invalid log destinations starting an unsupervised worker. The regression
  failed before the fix and passed after it.
- Local macOS verification passed Rust formatting, strict Clippy, 79 Rust test
  executions, installer tests, both lint-audit shell suites, and the offline demo.
- Added Linux/macOS CI for those checks. Hosted results are recorded on the PR.
- Gitleaks scanned all local Git refs: 200 commits, no detected secrets. This is
  automated pattern detection, not proof that all historical material is public-safe.
  The cleaned source tree also passed a separate scan.

## Publication boundary

The MIT source release was made public on 2026-11-10 after Linux/macOS CI passed.
Generated mission records and
OpenSpec-generated commands were removed from the current tree and backed up
outside the repository. History, other branches, tags, releases, issues, and PRs
were preserved. Their attachments and discussions have not received an exhaustive
publication review. Publication intentionally preserves that history; cleanup
was scoped to the current repository files.

The Linux descendant-cleanup issue remains open; see [known limitations](known-limitations.md).
No live model mission or model-quality benchmark was run as part of this cleanup.
