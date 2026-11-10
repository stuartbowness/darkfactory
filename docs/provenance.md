# Provenance

Darkfactory was developed by Stuart Bowness with AI coding assistance. It combines
an agent-operated development workflow with a Rust process supervisor.

## Upstream influences and adapted material

- Factory's mission architecture informed the orchestrator/worker/reviewer
  roles and validation-contract approach. Darkfactory implements its own
  Markdown workflow; it is not Factory's implementation or an affiliated product.
- [Matt Pocock's skills](https://github.com/mattpocock/skills) informed and supplied
  adapted interview, TDD, and software-design material.
- [buildoak/agent-mux](https://github.com/buildoak/agent-mux) was a reference for
  process supervision, event parsing, and timeout design. Darkfactory's dispatcher
  is implemented in Rust and deliberately supports one worker CLI.
- [Vercel's agent skills](https://github.com/vercel-labs/agent-skills),
  [Web Interface Guidelines](https://github.com/vercel-labs/web-interface-guidelines),
  and [shadcn/ui](https://github.com/shadcn-ui/ui) supply adapted frontend guidance.
- [OpenSpec](https://github.com/Fission-AI/OpenSpec) supplies the planning-artifact
  convention and external CLI integration.

The goal is to make the adaptation and engineering decisions inspectable, not to
claim invention of multi-agent review, TDD, or process supervision.
[THIRD_PARTY_NOTICES.md](../THIRD_PARTY_NOTICES.md) maps bundled material to notices.

## What this repository contributes

The mission and recovery instructions connect clarification, scoped worker briefs,
acceptance criteria, validation, and bounded repair attempts. The Rust dispatcher
provides a synchronous execution boundary with process-group signalling,
structured output, event parsing, and fixture-based regression tests.

Separating worker and reviewer prompt context is intentional, but does not
prevent either role from reading files in the shared repository.

The initial talk-reference notes are retained as historical notes, not verified
transcripts or current documentation of the referenced products. No comparative
benchmark or reliability advantage over those products is claimed.
