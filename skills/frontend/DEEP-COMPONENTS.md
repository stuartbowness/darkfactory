# Deep Components

A deep component is a React component with a small semantic interface that hides meaningful state, effects, and rendering logic. It owns one user-facing workflow end-to-end. The caller should understand the domain concept and the final semantic events, not every intermediate step the component takes to render or coordinate that workflow.

Depth is not about file length or raw prop count. It is about how much complexity the component removes from its consumers.

## Four Rules

### 1. Small Semantic Interfaces

A component's prop interface should represent domain concepts, not implementation knobs. Count concepts across props, context providers, URL params, and cache dependencies. Do not count only raw props.

A component with 6 props describing one concept can still be deep. A component with 1 prop bag containing 4 unrelated concerns is shallow because the caller must understand too many separate things.

As a guideline, components requiring the caller to understand more than 7 independent semantic concepts (counting props, consumed context providers, URL params, and cache dependencies) are likely too wide unless they are an allowed-shallow exception.

Diagnostic: "How many independent domain concepts must a caller understand to use this component?"

### 2. Derive Don't Sync

Compute values during render with inline computation or `useMemo` instead of syncing state with `useEffect`. Effects that transform data without I/O are a depth violation because they split one fact across two sources.

Use effects for I/O, subscriptions, imperative browser APIs, and external systems. Do not use effects just to mirror props, filter arrays into state, or calculate display values. The canonical source is the React team's "You Might Not Need an Effect."

Diagnostic: "Can this value be computed from existing state/props during render instead of stored separately?"

### 3. Composition Over Configuration

Use compound component slots and children instead of config objects with many options. Callers should compose behavior without learning the child's internal option schema.

A component accepting `items: [{ label, icon, onClick, disabled, variant }]` is shallower than one accepting `<Menu><MenuItem>...</MenuItem></Menu>` because the config shape turns component behavior into data plumbing. Prefer children and named slots when callers need to provide UI or behavior. As a guideline, option objects with more than 5 configuration entries are a signal to consider compound components instead.

Diagnostic: "Would this be clearer as compound components with children/slots instead of a config prop?"

### 4. Collapse Coordination

A deep component owns one user-facing workflow end-to-end. It exposes semantic events such as `onSubmit` and `onCancel`, not coordination knobs such as `onValidate`, `onStep1Complete`, or `onReset`.

Parents choose what happens when the workflow finishes or is cancelled. They should not drive every intermediate state inside the child. If the parent must choreograph validation, step transitions, reset behavior, and loading display, the component is not owning the workflow.

Diagnostic: "Does the parent coordinate state that belongs inside this component?"

## Allowed-Shallow Exceptions

### Design System Leaves

`Button`, `Badge`, `Card` sub-components, and similar UI leaves are thin by design. They should be small CVA wrappers with `data-slot` attributes. Their job is visual consistency and accessibility hooks, not workflow ownership.

### Route/Layout Glue

`App.tsx`, route modules, and page wrappers may read route params, load route-level data, and hand that data to a deep component. This is acceptable if they stay thin and do not become workflow owners.

### Adapter Components

Components such as an auth callback page or OAuth redirect handler adapt external flows into the app. They are boundary concerns. They are allowed to be shallow because their purpose is integration, not behavior ownership.

### Headless Behavior Primitives

Radix Dialog, Combobox, Popover wrappers, and similar headless behavior primitives provide infrastructure without business rules. They may own accessibility and interaction mechanics, but they should not know domain logic.

## Deletion Test

Use this diagnostic before keeping a component:

"If you deleted this component, would complexity vanish (it was a pass-through) or reappear across every consumer (it was earning its keep)?"

If deletion removes only a layer of prop forwarding, the component was shallow. If deletion forces every caller to recreate state, rendering branches, validation, and workflow steps, the component is deep.
