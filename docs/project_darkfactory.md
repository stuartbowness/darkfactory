---
name: darkfactory-project-vision
description: Darkfactory project goal -- build a lightweight multi-agent mission system using Claude Code primitives, inspired by Factory's Missions architecture
type: project
originSessionId: d0be7010-3858-4b92-8153-32045af7ab9b
---
Darkfactory is an exploration of building a lightweight version of Factory's "Missions" multi-agent system, but using Claude Code primitives (skills, subagents, commands, references) instead of a custom IDE/app.

**Why:** Stuart attended a talk by a Factory engineer (AI Engineer Europe, April 2026) describing their Missions architecture: orchestrators plan and write validation contracts, workers implement features serially with clean context, validators verify adversarially. The system runs autonomously for hours/days. Stuart wants to see if similar patterns can be composed from Claude Code's existing building blocks.

**How to apply:** All design decisions should favor composing existing Claude Code primitives (Agent tool, skills, CLAUDE.md files, hooks, git) over building custom infrastructure. Keep it lightweight and iterative.
