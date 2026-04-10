---
name: stuart-profile
description: Stuart's role, projects, workflow preferences, and current toolchain for AI-assisted development
type: user
originSessionId: d0be7010-3858-4b92-8153-32045af7ab9b
---
Stuart builds multiple projects simultaneously using AI-assisted spec-driven development. New to the multi-agent systems field but rapidly adopting advanced patterns.

**Active projects:**
- **Toyo** (~/Code/toyo) -- AI Chief of Staff for founders. React/Cloudflare Workers/Postgres. 27 Claude skills. Most mature workflow.
- **Ironhog** (~/Code/ironhog) -- Supply chain security gate (Rust). Built almost entirely autonomously via "ralph loops" (bash scripts that iterate through OpenSpec tasks one at a time, build/test/commit per iteration, circuit breaker on stagnation). 348+ iterations.
- **Crucible** (~/Code/crucible) -- Personal AI assistant (Rust/Telegram). Mature phase flow: idea -> spec -> build -> verify -> ship -> merge.
- **Darkfactory** (~/Code/darkfactory) -- New project exploring lightweight multi-agent mission system.

**Toolchain:**
- Claude Code (Opus 4.6) for planning/speccing -- daily driver for team
- Codex (GPT-5.4 High) for implementation/coding tasks
- Claude/Codex for review/verification (cross-checks against each other)
- OpenSpec for spec-driven development (proposal -> design -> specs -> tasks)
- Custom skills, hooks, statusline in Claude Code
- SSH git, gh CLI, Linear for project management

**Key workflow patterns:**
- Ralph loops = primitive version of Factory's serial worker execution
- Already does creator-verifier (Claude plans, Codex codes, both review)
- Already does spec-before-code via OpenSpec validation contracts
- Wants to avoid vendor lock-in (why not using Factory directly)
