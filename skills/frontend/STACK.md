# Frontend Stack

This reference defines the default React frontend stack and conventions for Darkfactory frontend work.

## Stack

- React 19
- Vite
- TailwindCSS 4
- Radix UI / ShadCN
- Vitest
- React Testing Library
- React Router

## Import Discipline

Use direct imports only. Do not import UI components from barrel files.

```ts
import { Button } from '@/components/ui/button'
import { cn } from '@/lib/utils'
```

Do not use imports like this:

```ts
import { Button } from '@/components/ui'
```

Use `cn()` from `@/lib/utils` for conditional class composition.

## File Organization

- `components/ui/` contains design system leaves such as buttons, badges, cards, fields, dialogs, and wrappers around Radix primitives.
- `components/<domain>/` contains deep components for a product or workflow area, such as `components/chat/` or `components/toyo/`.
- `hooks/` contains stateful logic extraction when the state belongs outside a single component or must be tested separately.
- `lib/` contains pure utilities with no React state and no rendering.

## Semantic Color Tokens

Use semantic color tokens such as `bg-card`, `text-card-foreground`, `text-muted-foreground`, `border-border`, and `bg-background`.

Do not use raw Tailwind colors such as `bg-blue-500`, `text-zinc-600`, or `border-slate-200` in app components. Do not add manual `dark:` overrides. Theme behavior should come from tokens.

## Data Slot Attributes

Follow the ShadCN convention: each design system component renders a `data-slot` attribute for styling hooks.

```tsx
function Card({ className, ...props }: React.ComponentProps<'div'>) {
  return (
    <div
      data-slot="card"
      className={cn('bg-card text-card-foreground rounded-lg border', className)}
      {...props}
    />
  )
}
```

## Testing

Use Vitest with React Testing Library. Test components through rendered behavior: roles, labels, visible text, user interaction, and final visible outcomes.

Do not test implementation details such as hook calls, prop plumbing, internal state names, or class composition internals.

## Routing

Use React Router for client-side routing. Route modules should stay thin: read route params, load route-level data, and pass domain facts into deep feature components.
