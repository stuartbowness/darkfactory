//! Domain vocabulary for `darkfactory-render` plus the fail-loud error type.
//!
//! Every other `core` module shares these types. The core never prints and
//! never falls back silently: parsing and rendering return
//! `Result<_, RenderError>`; only the future CLI translates errors to stderr
//! and exit codes.
//!
//! Some types and fields are not yet read in this task; later parse/render
//! tasks consume them. Because the whole catalog is `pub` from the library
//! root, the compiler treats it as reachable public API, so no `dead_code`
//! suppression is needed here.

/// How a validator checks an acceptance entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerificationMethod {
    /// Run a command.
    Execution,
    /// Inspect output, a file, or a response.
    Output,
    /// Read code.
    Inspection,
}

impl VerificationMethod {
    /// Parse the bare method token (`execution`, `output`, `inspection`).
    ///
    /// Bracket / `[...]` handling belongs to the line parser; this parses the
    /// inner word only. An unrecognized token is
    /// [`RenderError::UnknownVerificationMethod`].
    pub fn parse(token: &str) -> Result<Self, RenderError> {
        match token {
            "execution" => Ok(Self::Execution),
            "output" => Ok(Self::Output),
            "inspection" => Ok(Self::Inspection),
            other => Err(RenderError::UnknownVerificationMethod(other.to_string())),
        }
    }
}

/// A scenario status. Stage 1 only reads `Deviated` to mark `(DEVIATED)`.
///
/// An omitted status is modeled as `Option<ScenarioStatus> == None`, which
/// means non-deviated. See [`ScenarioStatus::parse_optional`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScenarioStatus {
    Proposed,
    Approved,
    Built,
    Verified,
    Deviated,
}

impl ScenarioStatus {
    /// Parse a present status token into a variant.
    ///
    /// An unrecognized token is [`RenderError::UnknownStatus`].
    pub fn parse(token: &str) -> Result<Self, RenderError> {
        match token {
            "proposed" => Ok(Self::Proposed),
            "approved" => Ok(Self::Approved),
            "built" => Ok(Self::Built),
            "verified" => Ok(Self::Verified),
            "deviated" => Ok(Self::Deviated),
            other => Err(RenderError::UnknownStatus(other.to_string())),
        }
    }

    /// Parse an optional status token.
    ///
    /// `None` (status omitted) reads as non-deviated and yields `Ok(None)`. A
    /// present but unrecognized value is [`RenderError::UnknownStatus`]; an
    /// omitted status is never an error.
    pub fn parse_optional(token: Option<&str>) -> Result<Option<Self>, RenderError> {
        match token {
            None => Ok(None),
            Some(token) => Self::parse(token).map(Some),
        }
    }

    /// Whether this status records an approved divergence.
    pub fn is_deviated(self) -> bool {
        matches!(self, Self::Deviated)
    }
}

/// A `capability/N` scenario identifier (e.g. `add-todo/1`) tracing a check
/// back to a formal spec scenario.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecReference {
    /// The capability name (the part before `/`).
    pub capability: String,
    /// The scenario number N (the part after `/`).
    pub number: u32,
}

/// One acceptance entry: a verbatim check statement, how to verify it, and the
/// task it points at.
///
/// The Spec Reference is inherited from the entry's scenario, so it is not
/// stored on the entry itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcceptanceEntry {
    /// The verbatim falsifiable check statement.
    pub statement: String,
    /// How a validator checks the statement.
    pub method: VerificationMethod,
    /// The integer task number this entry links to.
    pub task_link: u32,
}

/// A parsed spec scenario: its identifier, status, and acceptance entries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequirementScenario {
    /// The `capability/N` identifier of this scenario.
    pub spec_ref: SpecReference,
    /// The scenario status; `None` means omitted (non-deviated).
    pub status: Option<ScenarioStatus>,
    /// The acceptance entries declared under this scenario.
    pub acceptance: Vec<AcceptanceEntry>,
}

/// The standing authored source of mandatory operational checks that trace to
/// no spec scenario (smoke-test, cleanup, module-shape, claude-arch,
/// documentation).
///
/// Models the closing-task kinds mapped to their checks.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ClosingTaskFragment {
    /// Each closing-task kind paired with the operational checks it contributes.
    pub kinds: Vec<ClosingTaskKind>,
}

/// One closing-task kind and the checks it contributes to the contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClosingTaskKind {
    /// The kind name (e.g. `smoke-test`, `cleanup`).
    pub name: String,
    /// The verbatim operational check statements for this kind.
    pub checks: Vec<String>,
}

/// One task from tasks.md: a feature task or a closing task.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    /// The ordinal task number.
    pub number: u32,
    /// The task name.
    pub name: String,
    /// The closing-task kind when this is a closing task; `None` for a feature
    /// task.
    pub closing_kind: Option<String>,
}

/// The ordered tasks from tasks.md.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TaskList {
    /// The tasks in document order.
    pub tasks: Vec<Task>,
}

/// The typed output contract document model produced by the render step.
///
/// `body` is the rendered `contract.md`; `warnings` carries non-fatal advisories
/// (currently the inspection-cap breach) that the CLI emits to stderr without
/// failing the render.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RenderedContract {
    /// The fully rendered `contract.md` body.
    pub body: String,
    /// Non-fatal advisories produced during render (e.g. the inspection-cap
    /// breach). An empty list means a clean render.
    pub warnings: Vec<String>,
}

impl RenderedContract {
    /// The rendered `contract.md` body.
    pub fn to_markdown(&self) -> String {
        self.body.clone()
    }

    /// The non-fatal render advisories, in the order they were produced.
    pub fn warnings(&self) -> &[String] {
        &self.warnings
    }
}

/// The fail-loud error surface for the whole crate.
///
/// `core` returns `Result<_, RenderError>` and never prints; only the future
/// CLI maps these to stderr and exit codes. Only the first two variants are
/// exercised in this task; later tasks use the rest.
#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum RenderError {
    #[error("unknown verification method: {0}")]
    UnknownVerificationMethod(String),
    #[error("acceptance statement is empty for scenario {0}")]
    EmptyStatement(String),
    #[error("scenario {0} has no acceptance entries")]
    EmptyAcceptanceList(String),
    #[error("acceptance entry is missing a spec reference")]
    MissingSpecRef,
    #[error("duplicate spec reference: {0}")]
    DuplicateSpecReference(String),
    #[error("unknown scenario status: {0}")]
    UnknownStatus(String),
    #[error("acceptance entry links to task {0}, which does not exist")]
    DanglingTaskLink(u32),
    #[error("acceptance entry links to closing task {0}")]
    AcceptanceLinksClosingTask(u32),
    #[error("feature task {0} has no acceptance checks")]
    FeatureTaskNoChecks(u32),
    #[error("unknown closing-task kind: {0}")]
    UnknownClosingKind(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verification_method_parses_execution_token() {
        assert_eq!(
            VerificationMethod::parse("execution"),
            Ok(VerificationMethod::Execution)
        );
    }

    #[test]
    fn verification_method_parses_output_and_inspection_tokens() {
        assert_eq!(
            VerificationMethod::parse("output"),
            Ok(VerificationMethod::Output)
        );
        assert_eq!(
            VerificationMethod::parse("inspection"),
            Ok(VerificationMethod::Inspection)
        );
    }

    #[test]
    fn verification_method_rejects_unknown_token() {
        assert_eq!(
            VerificationMethod::parse("bogus"),
            Err(RenderError::UnknownVerificationMethod("bogus".to_string()))
        );
    }

    #[test]
    fn scenario_status_parses_deviated_token() {
        assert_eq!(
            ScenarioStatus::parse("deviated"),
            Ok(ScenarioStatus::Deviated)
        );
    }

    #[test]
    fn scenario_status_parses_all_present_tokens() {
        assert_eq!(
            ScenarioStatus::parse("proposed"),
            Ok(ScenarioStatus::Proposed)
        );
        assert_eq!(
            ScenarioStatus::parse("approved"),
            Ok(ScenarioStatus::Approved)
        );
        assert_eq!(ScenarioStatus::parse("built"), Ok(ScenarioStatus::Built));
        assert_eq!(
            ScenarioStatus::parse("verified"),
            Ok(ScenarioStatus::Verified)
        );
    }

    #[test]
    fn scenario_status_rejects_unknown_token() {
        assert_eq!(
            ScenarioStatus::parse("bogus"),
            Err(RenderError::UnknownStatus("bogus".to_string()))
        );
    }

    #[test]
    fn omitted_status_reads_as_non_deviated() {
        let parsed = ScenarioStatus::parse_optional(None).expect("omitted status is not an error");
        assert_eq!(parsed, None);
        assert!(
            !parsed.map(ScenarioStatus::is_deviated).unwrap_or(false),
            "omitted status must read as non-deviated"
        );
    }

    #[test]
    fn present_status_parses_through_optional() {
        assert_eq!(
            ScenarioStatus::parse_optional(Some("deviated")),
            Ok(Some(ScenarioStatus::Deviated))
        );
    }

    #[test]
    fn present_unknown_status_through_optional_is_error() {
        assert_eq!(
            ScenarioStatus::parse_optional(Some("bogus")),
            Err(RenderError::UnknownStatus("bogus".to_string()))
        );
    }

    #[test]
    fn deviated_status_is_deviated_others_are_not() {
        assert!(ScenarioStatus::Deviated.is_deviated());
        assert!(!ScenarioStatus::Proposed.is_deviated());
        assert!(!ScenarioStatus::Verified.is_deviated());
    }

    #[test]
    fn render_error_messages_are_stable() {
        assert_eq!(
            RenderError::UnknownVerificationMethod("foo".to_string()).to_string(),
            "unknown verification method: foo"
        );
        assert_eq!(
            RenderError::UnknownStatus("bar".to_string()).to_string(),
            "unknown scenario status: bar"
        );
    }
}
