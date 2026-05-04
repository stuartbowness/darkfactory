# Worker Brief: Task 4 - PATTERNS.md and TESTING.md

## Your Task

Write two reference files in `skills/frontend/`:
1. `PATTERNS.md` — concrete code snippets illustrating the deep-component rules
2. `TESTING.md` — frontend behavioral testing guidance

## PATTERNS.md

Write PATTERNS.md with the following structure. Use the pre-extracted code snippets below. Each snippet is 5-10 lines. Label each with the rule it illustrates.

### Deep Component Examples (at least 2)

**Example 1: FilesDrawer — small prop interface owning a workflow**
Source: ~/Code/toyo (file management drawer)
Rule: Small Semantic Interfaces + Collapse Coordination

```typescript
interface FilesDrawerProps {
  open: boolean
  onClose: () => void
  mode?: "drawer" | "page"
}
```

Three props, two semantic events. The component owns file upload, delete, directory navigation, drag-drop, and folder creation internally. The parent only knows "open/close" and "drawer or page mode."

**Example 2: BrowserPane — derives state from props**
Source: ~/Code/toyo (browser preview pane)
Rule: Derive Don't Sync

```typescript
const iframeUrl = useMemo(
  () => buildLiveViewUrl(browserSession.liveViewUrl, readOnlyMode),
  [browserSession.liveViewUrl, readOnlyMode],
)
```

The iframe URL is derived during render via useMemo, not stored in state and synced with useEffect. The component has ~8 props but they describe one concept (browser session display).

### Allowed-Shallow Examples (at least 2)

**Example 1: Button — design system leaf**
Source: ~/Code/toyo (UI primitives)
Rule: Allowed-shallow exception — design system leaf

```tsx
function Button({
  className,
  variant = "default",
  size = "default",
  asChild = false,
  ...props
}: ButtonProps) {
  const Comp = asChild ? Slot.Root : "button"
  return (
    <Comp
      data-slot="button"
      className={cn(buttonVariants({ variant, size, className }))}
      {...props}
    />
  )
}
```

Zero logic. CVA for variants, Slot for polymorphism, rest spread. This is intentionally shallow — its job is visual consistency, not workflow ownership.

**Example 2: Card + CardHeader — compound component pattern**
Source: ~/Code/toyo (UI primitives)
Rule: Composition over Configuration (design system side)

```tsx
function Card({ className, ...props }: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="card"
      className={cn("bg-card text-card-foreground flex flex-col gap-6 rounded-xl border py-6 shadow-sm", className)}
      {...props}
    />
  )
}

function CardHeader({ className, ...props }: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="card-header"
      className={cn("@container/card-header grid auto-rows-min ...", className)}
      {...props}
    />
  )
}
```

Each sub-component is a thin wrapper. Callers compose: `<Card><CardHeader>...</CardHeader><CardContent>...</CardContent></Card>`. No config objects.

### Hook Pattern — Derive Don't Sync (at least 1)

**Example: useChatScroll — pure function derives from DOM**
Source: ~/Code/toyo (chat scroll behavior)
Rule: Derive Don't Sync

```typescript
export function isAtBottomPosition(
  element: Pick<HTMLDivElement, "scrollHeight" | "scrollTop" | "clientHeight">
): boolean {
  return element.scrollHeight - element.scrollTop - element.clientHeight < AT_BOTTOM_THRESHOLD
}

// Inside useChatScroll:
function onScroll() {
  const el = scrollRef.current
  if (!el) return
  const atBottom = isAtBottomPosition(el)
  isBottomLockedRef.current = getNextBottomLockState({
    atBottom,
    nextScrollTop: el.scrollTop,
    previousScrollTop: previousScrollTopRef.current,
    wasLocked: isBottomLockedRef.current,
    isUserScrollUp: recentUserScrollIntentRef.current === "up",
  })
  setIsAtBottom(atBottom)
}
```

Pure function computes `isAtBottom` from DOM measurements. Lock state lives in a ref (not useState). Display state (`isAtBottom`) is the only useState — it's the minimal state needed for rendering.

### Anti-Pattern with Fix (at least 1)

**Anti-pattern: useEffect syncing derived state**

```typescript
// BAD: syncing a pure derivation via useEffect
const [isAtBottom, setIsAtBottom] = useState(false)

useEffect(() => {
  setIsAtBottom(
    scrollRef.current.scrollHeight -
    scrollRef.current.scrollTop -
    scrollRef.current.clientHeight < 50
  )
}, [scrollRef.current?.scrollHeight])
```

**Why it's wrong:** Derives state in an effect with incomplete dependency tracking. Creates stale closures, triggers unnecessary renders, and races with scroll events.

**Fix:** Extract the pure function, call it in the scroll handler:

```typescript
// GOOD: pure function called on demand
const atBottom = isAtBottomPosition(scrollRef.current)
setIsAtBottom(atBottom)
```

## TESTING.md

Write TESTING.md covering frontend behavioral testing. This is NOT a copy of /tdd's testing philosophy — it's React-specific.

### Structure

1. **Core principle:** Test what users see and do, not how the component works internally.

2. **How to test components:**
   - Render the component with realistic props
   - Query by role, label, visible text (use `getByRole`, `getByLabelText`, `getByText`)
   - Fire user events with `userEvent` (click, type, tab — not `fireEvent`)
   - Assert visible outcomes (text appears, element disappears, callback fires)

3. **What NOT to test:**
   - Prop plumbing (parent passes X, child receives X)
   - Hook calls (useEffect fired, useState set)
   - Internal state names or values
   - Class names or Tailwind utility classes
   - Snapshot tests for layout (use visual verification instead)

4. **Hook logic boundary:**
   - Complex hook logic (validation, state machines, data transformation) is tested in separate `/tdd` tasks
   - Component tests should NOT re-test hook logic — test the component's visible behavior that results from the hook

5. **Test survival rule:**
   - A good test survives refactoring that preserves behavior
   - If renaming internal state, extracting a sub-component, or switching from local state to a hook breaks the test, the test was coupled to implementation

## Project Context

These files complete the `/frontend` skill's reference library. SKILL.md (task 3) already links to both files.

## Architecture

Both files go in `skills/frontend/` alongside SKILL.md, DEEP-COMPONENTS.md, and STACK.md.

## Key Terms

- **Deep component**: Small prop interface, owns one workflow end-to-end
- **Allowed-shallow exception**: Intentionally thin (design system leaf, route glue, adapter, headless primitive)
- **Derive don't sync**: Compute during render, not in useEffect
- **Behavioral test**: Tests visible behavior through rendered output, not implementation details

## Domain Types

None for this task.

## What Previous Tasks Built

### Task 1: Foundation vocabulary files
- skills/frontend/DEEP-COMPONENTS.md: four rules, allowed-shallow exceptions, deletion test
- skills/frontend/STACK.md: React 19 + Vite + Tailwind 4 + ShadCN + Vitest stack

### Task 2: Scrutiny validator gate 7
- Gate 7 (Frontend depth review) inserted, gates renumbered 1-15, gate 1 methodology-aware

### Task 3: Core /frontend SKILL.md
- Workflow: Scan → Contract → Build (visual verify) → Test → Deep-component audit
- Per-component loop, checklist before commit
- Links to DEEP-COMPONENTS.md, STACK.md, PATTERNS.md, TESTING.md

## Downstream Consumers

- SKILL.md links to both files (links already exist, files need to be created)
- Workers following /frontend will reference PATTERNS.md for examples and TESTING.md for testing guidance

## Procedures

- Use `/tdd` to implement this task
- Commit all changes in a single commit with message: "[task-4] Add PATTERNS.md and TESTING.md"
- Write your handoff report to openspec/changes/frontend-architecture-skill/mission/handoffs/task-4.md

## Handoff Report Format

- Task: Task 4 - PATTERNS.md and TESTING.md
- Commit SHA: <the full SHA>
- Approach: <brief explanation>
- Commands run: <any commands and their exit codes>
- Concerns: <anything uncertain>
