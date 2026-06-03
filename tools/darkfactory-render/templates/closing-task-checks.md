# Closing-task checks

The standing operational checks for the mandatory closing tasks. These trace to
no spec scenario; the contract renderer joins each `Closing: <kind>` task to the
checks under its kind here. Each check is a `- [<method>] <statement>` bullet;
the method tag is one of `execution`, `output`, or `inspection`.

### smoke-test
- [execution] an end-to-end test runs the project's primary user flow against real dependencies (no internal mocks) and passes

### cleanup
- [execution] the linter and build are clean
- [inspection] no dead code, unused dependencies, leftover scaffolding, or temporary artifacts remain

### module-shape
- [output] a record exists comparing the design's module inventory and type catalog to the committed code, noting any drift or none

### claude-arch
- [inspection] CLAUDE.md and ARCH.md exist and reflect the built codebase (created if absent; a pre-existing one is left unmodified)

### documentation
- [inspection] documentation for what was built exists
- [output] any commands shown in the documentation were run against the built artifacts with their real output pasted
