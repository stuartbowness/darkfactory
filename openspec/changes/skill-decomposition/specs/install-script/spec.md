## ADDED Requirements

### Requirement: install.sh exists at repo root
A POSIX-compatible shell script named `install.sh` SHALL exist at `~/.darkfactory/install.sh` and be executable.

#### Scenario: Script is present and executable
- **WHEN** a developer clones the Darkfactory repo
- **THEN** `~/.darkfactory/install.sh` exists
- **AND** the file is executable (`test -x ~/.darkfactory/install.sh` returns 0)
- **AND** the script's shebang is `#!/bin/sh` (not `#!/bin/bash`)

### Requirement: install.sh creates symlinks for all 12 skills
Running `install.sh` SHALL create a symlink in `~/.claude/skills/<name>` pointing to `~/.darkfactory/skills/<name>` for each of the 12 skills.

#### Scenario: All 12 skill symlinks created
- **WHEN** `~/.darkfactory/install.sh` is run on a clean machine (no existing symlinks)
- **THEN** the following symlinks are created: `~/.claude/skills/mission`, `~/.claude/skills/grill`, `~/.claude/skills/language`, `~/.claude/skills/prd`, `~/.claude/skills/adversarial`, `~/.claude/skills/dispatch`, `~/.claude/skills/tdd`, `~/.claude/skills/test-quality`, `~/.claude/skills/security`, `~/.claude/skills/mission-resume`, `~/.claude/skills/mission-status`, `~/.claude/skills/mission-init`
- **AND** each symlink resolves to its corresponding `~/.darkfactory/skills/<name>` directory

### Requirement: install.sh is idempotent
Running `install.sh` a second time SHALL produce the same result without errors.

#### Scenario: Re-run on existing symlinks
- **WHEN** `~/.darkfactory/install.sh` is run after it has already been run
- **THEN** the script exits with code 0
- **AND** all 12 symlinks still resolve correctly
- **AND** no existing non-Darkfactory skills in `~/.claude/skills/` are modified or removed

### Requirement: install.sh works on macOS and Linux
The script SHALL use only POSIX-portable constructs and work on both macOS (BSD coreutils) and Debian/Ubuntu (GNU coreutils).

#### Scenario: Script uses no GNU-only flags
- **WHEN** `install.sh` is reviewed
- **THEN** it contains no `sed -i`, `date -d`, `stat -c`, `mktemp --suffix`, `readlink -f`, or `grep -P` calls
- **AND** `ln -sf` is used to create symlinks (handles both creating and updating)

### Requirement: install.sh reports what it did
After running, the script SHALL print a summary of what was linked.

#### Scenario: Successful install output
- **WHEN** `~/.darkfactory/install.sh` runs successfully
- **THEN** it prints one line per skill symlink created or already-present
- **AND** the final line reports the total number of skills installed
- **AND** it exits with code 0

### Requirement: README reflects the new install process and skill set
The README.md SHALL be updated to match the post-refactor structure.

#### Scenario: Setup section replaced with install.sh
- **WHEN** README.md is inspected
- **THEN** the "First-time install" section shows `~/.darkfactory/install.sh` as the single install command
- **AND** it contains no manual `ln -sf` commands for individual skills

#### Scenario: Skills table uses current names
- **WHEN** the README.md skills table is inspected
- **THEN** it lists skills under their current names (e.g. `/mission-init`, `/mission-resume`, `/mission-status`)
- **AND** it contains no legacy names (`/darkfactory-init`)

#### Scenario: Required external skills section removed or updated
- **WHEN** README.md is inspected
- **THEN** the skills previously listed as "required external" from `layercodedev/skills` (`/grill-me`, `/ubiquitous-language`, `/write-a-prd`, `/tdd`, `/security-review`, `/test-quality-review`) are no longer described as external
- **AND** they are either listed in the owned skills table or described as bundled
- **AND** the only skills still marked as optional/project-specific are `/test-app` and `/demo-video`
