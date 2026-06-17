# Darkfactory simplification notes

Working notes from an exhaustive pass over the whole suite, looking for what to remove, merge, or thin to tighten the core product. Reviewer perspective: senior architect seeing the repo fresh. Date: 2026-06-16.

The core product is the autonomous mission loop: orchestrator dispatches workers, validators check them against a contract behind information walls, a draft PR ships. Everything is measured against whether it serves that loop.

## The one-line diagnosis

Darkfactory works, but it has accreted three kinds of weight:
1. **Skill sprawl** — 23 skills, but only ~9 are load-bearing in a mission. The rest are standalone accessories, maintainer meta-tools, or large reference dumps.
2. **Planning ceremony** — the planning half (proposal -> spec -> design -> tasks -> coverage-audit -> contract) carries 7 serial AI review gates and several drift audits before a single line of code is written. This is the bulk of time-to-first-commit.
3. **Tool gold-plating** — the Rust supervisor is ~30-40% incidental complexity (an async lifecycle store with one broken consumer, a security-hole abort channel with no consumer, dead test-only code).

Plus doc triplication: the same architecture is explained in README (27K), ARCH.md (8K), and UBIQUITOUS_LANGUAGE.md (29K).

---

## Skill inventory: load-bearing vs accessory

Load-bearing in a mission (keep, untouched):
- `mission` (+ `basic`), `mission-init`, `mission-resume` — the loop and its lifecycle
- `grill` (Phase 1), `language` (Phase 2), `artifact-review` (review gates) — planning
- `tdd`, `frontend` — worker methodologies (routed by task layer)
- `adversarial` — Phase 9 final gate
- `security`, `test-quality` — scrutiny validator subagents (conditional gates)
- `lint-audit` — mission-init setup check

Accessory / standalone (not invoked by the mission loop):
- `prd` — **dead** after proposal.md consolidation (commit 88dda38). Mission never calls it.
- `skill-review`, `write-a-skill` — maintainer meta-tools, not user-facing
- `domain-model`, `module-design`, `arch-review` — richer standalone alternatives, "not yet wired into missions"
- `qa` — standalone post-mission bug filing
- `mission-status` — read-only dashboard that duplicates mission-resume's parsing
- `ui-system` (307 lines), `react-perf` (4003 lines) — optional per-task frontend constraint reference

---

## Candidates by area

### A. Skills — removal

- **Remove `prd`** (75 lines, 1 skill). Dead since proposal consolidation; duplicates grill+language interview and module-design's module sketch; its GitHub-issue output is orphaned. Risk: very low. Update README + mission-init checklist.
- **Remove `skill-review` + `write-a-skill`** (~335 lines, 2 skills); move their content to `docs/`. These are Darkfactory-development tooling, not mission tooling. They inflate the skill list new users see. Risk: low.
- **Merge `mission-status` into `mission-resume`** (~110 net lines, 1 skill). Resume already parses every file status reads. Add a `--status` read-only early-exit. Risk: low; preserve the read-only guarantee.

Note on grill/language/domain-model: agent B concluded the grill (raw interview) + language (post-hoc glossary) split is **intentional**, not redundant, and domain-model is a deliberately-unwired richer alternative. Not a merge target. Only real dedup here: `arch-review/INTERFACE-DESIGN.md` re-implements `module-design`'s parallel-agent algorithm — collapse to one.

### B. Skills — large reference content

- **`react-perf` (4003 lines, single file)** violates the suite's own progressive-disclosure rule (<500 lines). It's verbatim Vercel guidance, optional per-task constraint. Options: split into companion files (keep in repo, lighter mental load) or extract to a separate guide repo with a thin router skill. Recommend split first.
- **`ui-system` (307 lines)** — verbatim shadcn guidance, optional constraint. Fine as-is; keep separate from frontend.
- **`lint-audit` (1151 lines, 6 files)** — large but justified (load-bearing at mission-init). Set a hard scope boundary: oxlint + tsconfig strictness only, no ESLint/biome/prettier creep.

### C. Mission orchestrator — ceremony (the big planning-time win)

- **Cut artifact-review from 7 gates to ~3.** Today every planning artifact gets a Codex review + checkpoint + human gate (glossary, proposal, each spec, design, tasks, coverage-audit, contract). Keep the reviews on proposal, contract, and the coverage audit (Phase 4.5) by default; make glossary, specs, design, and tasks opt-in via the standalone skill. The coverage audit stays reviewed because it's the orchestrator self-checking planning completeness — an independent pass earns its keep there more than on docs a human reads easily. Biggest wall-clock win. Risk: medium; do as a configurable default, not a hard cut.
- **Keep the coverage audit as its own phase (renamed 4.5), do NOT collapse it into the contract.** An earlier draft proposed folding it into Phase 5; on review they do different jobs — the audit checks completeness (proposal -> task), the contract checks verifiability (spec -> check). The audit is self-review of the orchestrator's own planning, which is exactly where the independent gate matters. The cleanup here is the renumber (4a -> 4.5), not removal.
- **Trim design.md from 6 mandatory sections to ~4.** Integration Points is derivable from Module Inventory + task topology; Test Strategy overlaps validators. Protect Type Catalog + Module Inventory (worker briefs and audits depend on them). Risk: moderate.
- **Drop the foundation audit (task-1-only drift check); keep the end-of-mission module-shape re-check.** Per-task scrutiny Gate 6 already cross-checks design every task, so the task-1 special case is the weakest of three overlapping checks. Risk: low-moderate (loses early warning).
- **Remove the per-task demo-video step** (non-blocking MP4 recording). Not a validation; user-test validator already drives the browser. Risk: low.
- **mission-init: cut the optional-skill enumeration** (~40 lines) to a one-line count; it never affects readiness.
- Do NOT merge basic and full mode — the duplication is shallow (flag docs) and they serve different rigor levels by design.

### D. Mission orchestrator — validation gates (lower confidence)

- **Consolidate the 15 scrutiny gates to ~10.** Fold silent-skip into test-quality, code-standards into code-review; group the deterministic grep gates under one heading. Risk: moderate — each gate catches a real class and the validator is the only wall before merge. Consolidate presentation before logic.

### E. darkfactory-dispatch (Rust) — make it synchronous-only

src is 2,990 LOC (not 4,141; that includes tests). Keep the genuinely hard core: process-group isolation + group signaling, two-phase SIGTERM/SIGKILL timeout while draining NDJSON, parent-death detection. Cut the incidental ~1,000 LOC:

- **Drop the async-dispatch subsystem** (`--async`, `status`/`result`/`wait`, the `~/.darkfactory/dispatches` store, ~300 LOC). Exactly one consumer (`adversarial`) and per findings.md it uses it incorrectly (ID never escapes the background shell; a passing review is reported as failed). Claude Code's own `run_in_background` + `BashOutput` is the real concurrency primitive. Fix adversarial to use a backgrounded synchronous `--json` dispatch.
- **Delete the control-file abort channel** (`control.json` watchdog). No consumer; findings.md flags it as a local-privilege security hole (world-known /tmp path lets any local user SIGKILL dispatches).
- **Collapse sync-mode status.json writing** once async is gone (its only readers were the removed subcommands). Live progress already flows as NDJSON to the log.
- **Remove unused flags `--grace` and `--heartbeat`** (no skill sets them; `--grace` enables the `--timeout 0` instant-SIGTERM foot-gun). Keep behavior as fixed internals.
- **Delete dead test-only APIs** (`graceful_stop`, `CancellationToken`/`heartbeat_ticker`, `StreamMode::Silent/Verbose`, `StdinMode::Piped`, `RunHandle::pid`) — flagged with `#[expect(dead_code)]`, do first, risk-free.

Target: ~1,800-2,000 LOC, a third gone, all incidental. Keep it Rust; a shell rewrite would lose the load-bearing robustness. Separate from this cleanup: findings.md 1.2/1.3 (sync error-path orphans a worker; unbounded stderr await can hang) are real bugs in the core to fix regardless.

### F. Docs & infra

- **README (27K) does too much** — setup + tutorial + reference + architecture. Triplicates ARCH.md (three-role model, info walls, dir layout) and the skills table. Split: thin README (setup + quick start), GETTING_STARTED.md (hands-on), fold the skills table + architecture into ARCH.md as the single source.
- **UBIQUITOUS_LANGUAGE.md (29K, 80+ terms)** has become a reference encyclopedia. Prune to ~15 core ambiguity-prone terms; move skill-specific vocab (test-health, frontend, dispatch, determinism) into the skills that own them.
- **ARCH.md** — remove the hardcoded "23 skills total" count (goes stale; git history shows it drifting) and the lint-audit paragraph (belongs in design-decisions).
- **Archive 2 completed openspec changes** (frontend-architecture-skill 10/10 tasks, lint-audit-skill 7/7) and **delete the abandoned `resume-phase-recovery`** (proposal-only, no tasks/mission dir). Note: `skill-decomposition` has 3 unchecked tasks (11/14) so it is NOT done — finish or formally close it, don't archive yet. `deterministic-planning` is 10/10 but has no PR. Keeps `changes/` scannable for real in-flight work.
- **Delete `migrate.sh`** — one-time pre-decomposition migration, dead since April.
- **Delete or populate empty `.claudeignore`.**
- Consolidate the two talk-notes files; keep provenance.md as the traceability source.

---

## The steering tension

Two different goals point at different cuts:
- **Cut maintenance surface, behavior unchanged**: remove prd / skill-review / write-a-skill, merge mission-status, dead-code the Rust tool, archive openspec, prune docs. Pure win, low risk.
- **Cut planning ceremony for speed** (accept more human judgment, less rigor): fewer artifact-review gates, collapse coverage-audit, trim design sections, fewer scrutiny gates. Real tradeoff — faster missions, less automated safety.

The first set is mechanical cleanup. The second is a product decision about how much ceremony the mission should impose.

## Conservative achievable total (low risk, behavior preserved)
Remove 4 skills (prd, skill-review, write-a-skill, mission-status->resume), ~1,000 LOC from the Rust tool, ~50K of redundant docs, 4 stale/dead openspec+infra files. No loss to the build/validate/fix loop.

---

## Ranked candidates

Risk = chance of breaking something / losing safety. Value = how much gets removed (S/M/L). DX = improvement to the human experience of using the suite.

### Tier 1 — high value or DX, low risk
- Drop Rust async subsystem + control channel + sync-status (synchronous-only) — Risk: Low, Value: L (~400 LOC, closes a security hole + bugs), DX: high
- Split/extract `react-perf` (4003 lines, one file) — Risk: Low, Value: L, DX: high
- Prune UBIQUITOUS_LANGUAGE.md 29K -> ~5K core terms — Risk: Low, Value: L (~24K prose), DX: high
- Split README (setup vs tutorial vs architecture) — Risk: Low, Value: M, DX: high
- Remove `skill-review` + `write-a-skill` (move to docs/) — Risk: Low, Value: M (~335 lines, 2 skills), DX: high
- Delete dead test-only Rust APIs — Risk: None, Value: M (~120 LOC), DX: medium
- Remove `prd` skill (dead since proposal consolidation) — Risk: Very low, Value: S (75 lines, 1 skill), DX: medium
- Archive 2 completed openspec changes (frontend-architecture-skill, lint-audit-skill) + delete abandoned resume-phase-recovery — Risk: Low, Value: L (~150K out of active), DX: medium
- Delete `migrate.sh` (dead since April) — Risk: None, Value: S, DX: neutral

### Tier 2 — solid wins, small caveats
- Merge `mission-status` into `mission-resume --status` — Risk: Low, Value: M (~110 lines, 1 skill), DX: neutral (lose muscle memory; can alias)
- Remove Rust `--grace`/`--heartbeat` flags — Risk: Low, Value: S (closes `--timeout 0` foot-gun), DX: medium
- mission-init: trim optional-skill listing — Risk: Very low, Value: S (~40 lines), DX: medium
- ARCH.md: drop hardcoded skill count + lint-audit paragraph — Risk: Low, Value: S, DX: medium
- Dedup `arch-review/INTERFACE-DESIGN.md` vs `module-design` — Risk: Low, Value: S, DX: neutral
- Delete/populate empty `.claudeignore`; merge talk notes — Risk: Low, Value: S, DX: neutral

### Tier 3 — biggest mission-speed lever, real rigor tradeoff
- Cut artifact-review gates 7 -> ~3 (keep proposal, contract, coverage audit; opt-in for glossary/specs/design/tasks) — Risk: Med, Value: M (~half of planning wall-clock), DX: high
- Keep coverage audit as Phase 4.5 (renamed from 4a), do NOT collapse into contract — different jobs; self-review worth an independent gate. DONE: renumbered.
- Drop task-1 foundation audit (gate 6 already covers it) — Risk: Low-med, Value: S (~16 lines), DX: medium
- Trim design.md 6 -> 4 sections (keep Type Catalog + Module Inventory) — Risk: Med, Value: S (~15 lines), DX: medium
- Remove per-task demo-video step — Risk: Low, Value: S (~13 lines), DX: neutral

### Tier 4 — lower confidence, handle with care
- Consolidate scrutiny gates 15 -> ~10 — Risk: Med, Value: M (~50 lines, faster validation), DX: medium
- Demote mission-resume Phase 3a marker table — Risk: Low-med, Value: S, DX: neutral
- Cut fallback.log ceremony (keep the fallback) — Risk: Med, Value: S, DX: neutral

Non-removal: set a hard scope boundary on `lint-audit` (oxlint + tsconfig only, no ESLint/biome creep) — it is the skill most likely to balloon next.

Deliberately NOT candidates (intentional design, not bloat): the grill/language/domain-model split; keeping basic mode separate from full mode.
