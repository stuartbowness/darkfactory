# Frontend Testing

## Core Principle

Test what users see and do, not how the component works internally.

Frontend tests should read like behavior specs. A good test says a user can open a menu, submit a form, see an error, or trigger a semantic callback. It should not care whether the component uses local state, a hook, a reducer, or extracted child components.

## How to Test Components

Render the component with realistic props.

```tsx
render(<FilesDrawer open onClose={onClose} mode="drawer" />)
```

Query by role, label, or visible text.

```tsx
screen.getByRole("button", { name: /new folder/i })
screen.getByLabelText(/file name/i)
screen.getByText(/uploads/i)
```

Use `userEvent` for real interaction. Prefer `click`, `type`, and `tab` over `fireEvent`.

```tsx
const user = userEvent.setup()

await user.click(screen.getByRole("button", { name: /new folder/i }))
await user.type(screen.getByLabelText(/folder name/i), "Reports")
await user.click(screen.getByRole("button", { name: /create/i }))
```

Assert visible outcomes and semantic events.

```tsx
expect(screen.getByText("Reports")).toBeVisible()
expect(onClose).not.toHaveBeenCalled()
```

## What Not to Test

Do not test prop plumbing. A test that only proves a parent passes `foo` to a child will break when the component is deepened without proving behavior changed.

Do not test hook calls. Avoid assertions such as "useEffect fired" or "useState was set." Test the rendered result or semantic callback instead.

Do not test internal state names or values. State is an implementation detail.

Do not test class names or Tailwind utility classes. Use visual verification for layout and styling behavior.

Do not use snapshot tests for layout. Snapshots are noisy for frontend structure and weak at proving what users can see or do.

## Hook Logic Boundary

Complex hook logic belongs in separate `/tdd` tasks when it has validation, state machines, data transformation, or other behavior that can be tested through a non-rendering interface.

Component tests should not re-test hook internals. They should test the visible behavior that results from the hook.

For example, a hook test can prove the state machine chooses the next chat scroll lock state. A component test should prove the scroll control appears, disappears, or responds to user action in the rendered UI.

## Test Survival Rule

A good test survives refactoring that preserves behavior.

If renaming internal state, extracting a sub-component, or switching from local state to a hook breaks the test, the test was coupled to implementation.

Before keeping a component test, ask:

- Would this still pass if the component had different private state names?
- Would this still pass if part of the UI moved into a child component?
- Would this still pass if state moved from `useState` to a tested hook?
- Does this fail when a user-visible behavior actually breaks?
