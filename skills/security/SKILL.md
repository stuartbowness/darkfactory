---
name: security
user-invokable: true
description: Complete a security review of the pending changes on the current branch
---

# Security Review
Work through each section. Report findings using the output format at the end.

## Authorization (IDOR)
Check every endpoint that takes an ID parameter. Verify the user can access that specific resource.
**Pattern to flag** - auth check without ownership check:
```typescript
const post = await db.posts.findUnique({ where: { id: req.params.id } })
return res.json(post) // Any user can access any post
// Fixed
const post = await db.posts.findUnique({ where: { id: req.params.id } })
if (post.authorId !== req.user.id) return res.status(403)
```
Check for:
- Endpoints that fetch by ID without filtering by user/org
- Update/delete operations missing ownership verification
- Multi-tenant data leaking across organizations
- API keys granting access beyond their owner's resources

## Input Validation
Check that user input is validated at API boundaries using Zod or similar.
**Pattern to flag** - using req.body directly:
```typescript
await db.posts.create({ data: req.body }) // Unvalidated
// Fixed
const data = CreatePostSchema.parse(req.body)
await db.posts.create({ data })
```
Also validate URL params and query strings, not just request bodies.

## Environment Variables
Check for:
- Hardcoded secrets in source code
- Secrets in client-exposed variables (`NEXT_PUBLIC_*`, `VITE_*`, etc.)
- Missing `.env` in `.gitignore`
- No startup validation of required env vars

## Injection
**SQL/NoSQL** - flag string interpolation in queries:
```typescript
db.query(`SELECT * FROM users WHERE id = ${userId}`) // Vulnerable
db.query('SELECT * FROM users WHERE id = $1', [userId]) // Safe
```
**XSS** - flag these patterns:
- `dangerouslySetInnerHTML` with user content
- `element.innerHTML = userInput`
- `href={userInput}` (javascript: URLs execute)
**Command injection** - flag `exec()` with interpolation:
```typescript
exec(`convert ${filename} output.png`) // Vulnerable
execFile('convert', [filename, 'output.png']) // Safe
```
**Template injection** - flag user input in template engines:
```typescript
const html = ejs.render(userTemplate); // Vulnerable
const html = ejs.render(staticTemplate, { data: userData }); // Safe
```
**Unvalidated redirects** - flag redirects using user input:
```typescript
res.redirect(req.query.next); // Vulnerable - open redirect
const allowed = ['/', '/dashboard', '/profile'];
if (allowed.includes(req.query.next)) res.redirect(req.query.next); // Safe
```
**File uploads**:
- Missing file type validation (check magic bytes, not just extension)
- No file size limits
- Uploaded files served from same domain (enables XSS)
- Path traversal in filenames (`../../etc/passwd`)

## Secrets
Flag:
- API keys, passwords, tokens in source code
- `console.log(req.body)` that may log credentials
- Secrets in error messages returned to clients
- Tokens without expiration

## Data Protection
Check for:
- Plaintext storage of sensitive data (PII, credentials)
- Weak encryption algorithms (DES, RC4, ECB mode)
- Hardcoded encryption keys
- Full credit card/SSN in API responses (mask to last 4 digits)
- Verbose error messages exposing stack traces or DB schema
- PII logged without redaction
- PII in URLs or query strings

## Authentication & Sessions
Check for:
- Missing auth middleware on protected routes
- `Math.random()` for tokens (use `crypto.randomBytes` or `crypto.randomUUID`)
- Cookies missing `httpOnly`, `secure`, or `sameSite` flags
- JWT tokens without expiration
- No rate limiting on login/password reset endpoints
- Passwords stored with weak hashing (MD5, SHA1) -- use bcrypt/argon2
- Session ID not regenerated after login (session fixation)
- No account lockout after failed attempts

## Race Conditions
Flag check-then-act patterns in financial/inventory operations:
```typescript
// Vulnerable - race between check and deduct
const balance = await getBalance(userId)
if (balance >= amount) await deductBalance(userId, amount)
// Safe - atomic transaction
await db.transaction(async (tx) => {
  const balance = await tx.getBalance(userId, { forUpdate: true })
  if (balance < amount) throw new Error('Insufficient funds')
  await tx.deductBalance(userId, amount)
})
```

## Security Headers
Check that the app configures:
- `Strict-Transport-Security` (HSTS)
- `Content-Security-Policy`
- `X-Content-Type-Options: nosniff`
- `X-Frame-Options: DENY` or CSP `frame-ancestors`
- `Referrer-Policy`
- `Permissions-Policy`

## CSRF Protection
Check for:
- Missing CSRF tokens on state-changing operations
- CSRF tokens not validated server-side
For SPAs: Use `SameSite=Strict` cookies + custom headers (e.g., `X-Requested-With`).

## CORS Configuration
Flag overly permissive CORS:
```typescript
app.use(cors({ origin: '*', credentials: true })); // Vulnerable
app.use(cors({
  origin: ['https://app.example.com'],
  credentials: true,
  methods: ['GET', 'POST'],
})); // Safe
```
Check for `Access-Control-Allow-Origin: *` with credentials.

## API Security
Flag:
- Stack traces in production error responses
- Database error messages exposed to clients
- Sensitive operations on GET requests
- GraphQL introspection enabled in production
- Swagger/OpenAPI docs exposed without auth
- Admin endpoints without additional auth layer
- Batch endpoints without limits

## Client-Side Security
Flag:
- Unsafe DOM manipulation (`innerHTML`, `document.write` with user input)
- Missing origin validation on `postMessage`
- Sensitive data in `localStorage`/`sessionStorage`
- External scripts without subresource integrity (SRI) hashes
- iframes from untrusted sources without `sandbox` attribute
- Webhook endpoints without signature verification

## Infrastructure & Configuration
Check for:
- Default credentials on any service
- Debug mode enabled in production
- Source maps deployed to production
- `.git` directory exposed in web root
- TLS versions below 1.2
- `NODE_TLS_REJECT_UNAUTHORIZED = '0'` in production
- Test accounts or backdoors in production code

## Dependencies
If reviewing changes to package manifests or lock files:
- Run `npm audit --audit-level=high` (or equivalent)
- Flag outdated packages with known CVEs
- Check for dependency confusion risk (private names conflicting with public registry)
- Verify lockfile is committed with integrity hashes

## Post-Review Checks
After completing the review, run:
```bash
# Dependency audit
npm audit --audit-level=high 2>/dev/null || pnpm audit 2>/dev/null || true
# Secret scanning
grep -rE "(api[_-]?key|secret|token|password)\s*[:=]\s*['\"][^'\"]+['\"]" --include="*.ts" --include="*.js" --include="*.py" --include="*.go" || true
grep -rE "AKIA[0-9A-Z]{16}" --include="*.ts" --include="*.js" --include="*.py" --include="*.go" || true
```

## Output Format
```
## Critical
- `src/api/posts.ts:23` - IDOR: delete endpoint missing ownership check
- `src/lib/db.ts:45` - SQL injection via string interpolation
## High
- `src/api/auth.ts:12` - No rate limiting on login endpoint
- `.env` - Not in .gitignore
## Medium
- `src/helpers.ts:23` - Unpinned package versions
- `src/ws.ts:112` - Session token in URL query params
```
Use `file:line` format. State the issue concisely.
