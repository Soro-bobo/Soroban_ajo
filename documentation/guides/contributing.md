# Contributing to Ajo Platform

## Development Workflow

1. Fork the repository and create a feature branch:
   ```bash
   git checkout -b feat/your-feature-name
   ```

2. Make your changes, following the conventions below.

3. Ensure all tests pass:
   ```bash
   cargo test                    # backend + contract
   pnpm --filter frontend lint   # frontend
   ```

4. Submit a pull request against `main`.

## Conventions

### Rust (Backend & Contracts)
- Use `?` for error propagation — no `.unwrap()` outside tests.
- Add `#[tracing::instrument]` to every public service method.
- All SQL via `sqlx` prepared statements — no string interpolation.
- Keep modules single-responsibility; controllers validate, services contain logic.

### TypeScript (Frontend)
- Strict mode — no `any`. Use `unknown` if the type is truly unknown.
- All props typed with interfaces, not `type` aliases (unless union types).
- No default exports except for Next.js pages (`export default function PageName`).
- Hooks return stable references (use `useCallback` / `useMemo` where needed).

### Git
- Commit messages: `type(scope): short description`
  - Types: `feat`, `fix`, `refactor`, `test`, `docs`, `chore`
  - Example: `feat(groups): add payout rotation endpoint`
- One logical change per commit.

### SQL Migrations
- Filename: `NNN_description.sql` (three-digit padded).
- Always include `CREATE INDEX` for foreign keys and queried columns.
- Never drop columns — mark as deprecated in a comment first.

## Testing

### Backend
```bash
cd backend
cargo test          # unit tests in each module
cargo test -- --ignored   # integration tests (require live DB)
```

### Contracts
```bash
cd contracts/ajo
cargo test          # runs all #[test] in tests/integration_test.rs
```

### Frontend
```bash
pnpm --filter frontend type-check   # TypeScript
pnpm --filter frontend lint         # ESLint
```

## Opening Issues

Before opening an issue:
- Search existing issues to avoid duplicates.
- Include: environment (OS, Rust version, Node version), reproduction steps, expected vs actual behavior.
