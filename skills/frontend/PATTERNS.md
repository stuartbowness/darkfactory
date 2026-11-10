# Frontend Patterns

Use these examples as reference points when applying the deep-component rules from [DEEP-COMPONENTS.md](DEEP-COMPONENTS.md).

## Deep Component Examples

### FilesDrawer - small prop interface owning a workflow

Rule: Small Semantic Interfaces + Collapse Coordination

Source: the internal Toyo application’s file management drawer

```typescript
interface FilesDrawerProps {
  open: boolean
  onClose: () => void
  mode?: "drawer" | "page"
}
```

Three props, two semantic events. The component owns file upload, delete, directory navigation, drag-drop, and folder creation internally. The parent only knows "open/close" and "drawer or page mode."

### BrowserPane - derives state from props

Rule: Derive Don't Sync

Source: the internal Toyo application’s browser preview pane

```typescript
const iframeUrl = useMemo(
  () => buildLiveViewUrl(browserSession.liveViewUrl, readOnlyMode),
  [browserSession.liveViewUrl, readOnlyMode],
)
```

The iframe URL is derived during render via `useMemo`, not stored in state and synced with `useEffect`. The component has about 8 props, but they describe one concept: browser session display.

## Allowed-Shallow Examples

### Button - design system leaf

Rule: Allowed-shallow exception - design system leaf

Source: the internal Toyo application’s UI primitives

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

Zero logic. CVA handles variants, Slot handles polymorphism, and the rest spread passes button attributes through. This is intentionally shallow. Its job is visual consistency, not workflow ownership.

### Card + CardHeader - compound component pattern

Rule: Composition over Configuration, design system side

Source: the internal Toyo application’s UI primitives

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

## Hook Pattern - Derive Don't Sync

### useChatScroll - pure function derives from DOM

Rule: Derive Don't Sync

Source: the internal Toyo application’s chat scroll behavior

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

Pure function computes `isAtBottom` from DOM measurements. Lock state lives in a ref, not `useState`. Display state, `isAtBottom`, is the only `useState`; it is the minimal state needed for rendering.

## Anti-Pattern with Fix

### Anti-pattern: useEffect syncing derived state

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

Why it is wrong: this derives state in an effect with incomplete dependency tracking. It creates stale closures, triggers unnecessary renders, and races with scroll events.

Fix: extract the pure function and call it in the scroll handler.

```typescript
// GOOD: pure function called on demand
const atBottom = isAtBottomPosition(scrollRef.current)
setIsAtBottom(atBottom)
```
