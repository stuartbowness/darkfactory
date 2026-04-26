# Worker Brief: Task 1 - Parser table tests

## Your Task
Add table-driven unit tests for the `parse_codex_event` function in `src/engine.rs`. Currently only 2 of ~10 event types have direct tests. Add inline `#[cfg(test)]` tests covering every event type the parser handles.

### Event types to test

For each event type, provide a realistic NDJSON line (based on real codex output) and assert the correct `EventKind` and extracted fields.

1. **thread.started** -> EventKind::SessionStart, session_id extracted
   ```json
   {"type":"thread.started","thread_id":"test-abc-123"}
   ```

2. **item.started (tool_use)** -> EventKind::ToolStart, tool name extracted
   ```json
   {"type":"item.started","item":{"type":"tool_use","name":"shell"}}
   ```

3. **item.started (agent_message)** -> EventKind::Response or appropriate kind
   ```json
   {"type":"item.started","item":{"type":"agent_message"}}
   ```

4. **item.completed (agent_message)** -> EventKind::Response, text extracted
   ```json
   {"type":"item.completed","item":{"type":"agent_message","text":"I created the file."}}
   ```

5. **item.completed (tool_use)** -> EventKind::ToolEnd, tool name extracted
   ```json
   {"type":"item.completed","item":{"type":"tool_use","name":"shell"}}
   ```

6. **item.completed with file write** -> EventKind::FileWrite, path extracted
   ```json
   {"type":"item.completed","item":{"type":"tool_use","name":"write","file_path":"src/main.rs"}}
   ```

7. **item.completed with file read** -> EventKind::FileRead, path extracted
   ```json
   {"type":"item.completed","item":{"type":"tool_use","name":"read","file_path":"Cargo.toml"}}
   ```

8. **turn.completed** -> EventKind::TurnComplete, tokens extracted
   ```json
   {"type":"turn.completed","usage":{"input_tokens":100,"output_tokens":50,"reasoning_tokens":20}}
   ```

9. **turn.failed** -> EventKind::TurnFailed
   ```json
   {"type":"turn.failed","error":{"message":"rate limited"}}
   ```

10. **error** -> EventKind::Error
    ```json
    {"type":"error","error":{"message":"connection reset"}}
    ```

11. **Non-JSON line** -> EventKind::RawPassthrough
    ```
    This is not JSON at all
    ```

12. **Unknown JSON type** -> None (parser returns None for unrecognized types)
    ```json
    {"type":"unknown.future.event","data":"something"}
    ```

13. **Empty line** -> None (parser returns None)

### Test pattern

Use a table-driven approach:
```rust
#[cfg(test)]
mod parser_tests {
    use super::*;

    #[test]
    fn parse_codex_event_table() {
        let cases = vec![
            ("thread.started", r#"{"type":"thread.started","thread_id":"abc"}"#, Some(EventKind::SessionStart)),
            // ... more cases
        ];
        for (name, input, expected_kind) in cases {
            let result = parse_codex_event(input);
            match expected_kind {
                Some(kind) => {
                    let event = result.expect(&format!("{name}: should parse"));
                    assert_eq!(event.kind, kind, "{name}: wrong EventKind");
                }
                None => assert!(result.is_none(), "{name}: should return None"),
            }
        }
    }
}
```

Also add individual tests for field extraction where important (session_id, token counts, file paths).

### Code standards
- Read `tools/darkfactory-dispatch/CLAUDE.md`
- Inline `#[cfg(test)]` in src/engine.rs
- Look at the existing `parse_codex_event` function to understand its exact behavior and match types
- Verify `cargo test` passes

## What Previous Tasks Built
The parser function `parse_codex_event` already exists in src/engine.rs. Read it first to understand the exact event mapping before writing tests.

## Procedures
- Read src/engine.rs to find parse_codex_event and understand its exact behavior
- Add the table-driven tests as inline #[cfg(test)]
- Verify `cargo test` passes
- Commit: "[test-1] Add parser table tests for all codex NDJSON event types"
- Write handoff to openspec/changes/dispatch-test-coverage/mission/handoffs/task-1.md
