# Full Audit Report

## Scope and commands run
- Read `README.md`.
- Checked for `CONTRIBUTING.md` (none found).
- Reviewed project directory layout.
- Ran:
  - `cargo test --workspace`
  - `cargo check --workspace`
  - `cargo clippy --workspace --all-targets`
  - `rg -n "TODO|FIXME|HACK"`
  - `cargo tree -d`
  - `cargo outdated -R` (not installed)
  - `cargo audit` (not installed)

## Findings (sorted by safest/easiest first)

### 1) Typo breaks builds (safe to fix)
- **File:** `crates/parser/src/syntax_kind/generated.rs:12`
- **Issue:** `#[doc(hiddent)]` is misspelled; should be `hidden`.
- **Severity:** **significant** (hard build blocker)
- **Safe, non-controversial fix?** **Yes**
- **Impact:** Causes `cargo check`, `cargo test`, and `cargo clippy` to fail before most of the workspace is compiled.

### 2) Deprecated Cargo config filename (safe to fix)
- **File:** `.cargo/config:1`
- **Issue:** Cargo warns that `.cargo/config` is deprecated in favor of `.cargo/config.toml`.
- **Severity:** **trivial**
- **Safe, non-controversial fix?** **Yes**
- **Impact:** Tooling warning on every Cargo invocation.

### 3) Stale TODO/FIXME/HACK comments (mostly safe to address incrementally)
- **File:** `crates/parser/src/event.rs:5`
  - `TODO: add node error event`
  - Severity: **moderate**
  - Safe fix: **Yes**, if scoped to parser error-node model.
- **File:** `crates/parser/src/grammar/statements.rs:49`
  - `TODO: need to allow global assignment also`
  - Severity: **significant** (language correctness gap)
  - Safe fix: **No** (behavioral/parser changes need design agreement).
- **File:** `crates/lua-analyzer/src/reload.rs:29`
  - `TODO: register dynamic cap`
  - Severity: **moderate**
  - Safe fix: **No** (protocol behavior change).
- **File:** `crates/text_edit/src/lib.rs:114`
  - `FIXME: mutate text in-place or reuse memory`
  - Severity: **trivial** (perf)
  - Safe fix: **Yes**, if micro-optimized without API changes.
- **File:** `crates/text_edit/src/lib.rs:120`
  - `FIXME: can be done without allocating intermediate vector`
  - Severity: **trivial** (perf)
  - Safe fix: **Yes**.
- **File:** `crates/lua-analyzer/src/diagnostics.rs:8`
  - `FIXME: should be FxHashMap<FileId, Vec<ra_id::Diagnostic>>`
  - Severity: **moderate**
  - Safe fix: **No** (type/model refactor).
- **File:** `crates/lua-analyzer/src/diagnostics.rs:10`
  - `FIXME: should be Vec<flycheck::Diagnostic>`
  - Severity: **moderate**
  - Safe fix: **No** (pipeline refactor).
- **File:** `crates/syntax/src/lib.rs:125`
  - `FIXME: validation errors are not handled here`
  - Severity: **significant** (may hide incremental-parse validation issues)
  - Safe fix: **No** (parsing behavior risk).

### 4) Lint warnings in proc-macro crate (safe to fix)
- **File:** `lib/binding_powers_impl/src/lib.rs:34`
  - Clippy: `from_over_into` (`impl Into` should be `impl From`).
  - Severity: **trivial**
  - Safe fix: **Yes**.
- **File:** `lib/binding_powers_impl/src/lib.rs:40`
  - Clippy: `non_canonical_partial_ord_impl`.
  - Severity: **trivial**
  - Safe fix: **Yes**.

### 5) Additional parser warnings (mostly safe, but lower priority than build blocker)
- **File:** `crates/parser/src/grammar/expressions.rs:197`
  - Clippy: needless `return`.
  - Severity: **trivial**
  - Safe fix: **Yes**.
- **File:** `crates/parser/src/grammar/expressions.rs:214`
  - Clippy: needless `return`.
  - Severity: **trivial**
  - Safe fix: **Yes**.
- **File:** `crates/parser/src/parser.rs:34`
  - Rust warning: confusing lifetime elision in return type (`Parser<'_>` explicit suggestion).
  - Severity: **trivial**
  - Safe fix: **Yes**.
- **File:** `crates/parser/src/parser.rs:8,110,163,189,218,219,257,271,280` and `crates/parser/src/grammar.rs:14,25`
  - Rust warnings: dead code / unused items.
  - Severity: **trivial** to **moderate** depending on intended roadmap.
  - Safe fix: **Mixed** (may represent planned parser paths).

### 6) Test failures, skipped tests, and low-coverage risk
- **Failing tests/build:** `cargo test --workspace` fails at compile time due to `#[doc(hiddent)]` typo.
  - Primary file: `crates/parser/src/syntax_kind/generated.rs:12`.
  - Severity: **significant**.
  - Safe fix: **Yes**.
- **Skipped tests:** No explicit `#[ignore]`/skip output was observed before failure; test execution is blocked by compile error.
  - Severity: **moderate** (unknown runtime status).
  - Safe fix: **Yes** (fix compile blocker first).
- **Low-coverage areas (structural):** many crates have no in-crate tests (`base_db`, `hir`, `ide`, `parser`, `stdx`, `text_edit`, `vfs`, `accept`, `binding_powers_impl`, `xtask` etc.).
  - Severity: **moderate**.
  - Safe fix: **Yes** (add focused unit tests is low risk).

### 7) Dependency and vulnerability audit limitations
- **Outdated dependencies:** could not run `cargo outdated` because command is not installed; installing it failed due network 403 while fetching crates.io index.
- **Vulnerabilities:** could not run `cargo audit` because command is not installed; installing it failed due the same network 403.
- **Duplicate dependency pressure:** `cargo tree -d` reports no duplicates in current resolution.
- **Severity:** **moderate** (visibility gap, not proven vulnerability)
- **Safe, non-controversial fix?** **Yes** (run `cargo outdated`/`cargo audit` in CI or a network-enabled dev environment).

### 8) Documentation gaps
- **Missing `CONTRIBUTING.md`:** repository has no contributing guide.
  - Severity: **trivial**.
  - Safe fix: **Yes**.
- **README appears stale/minimal for current workspace complexity:**
  - States parser is only partially implemented and “Please don't use this yet,” while repository includes multiple analysis/database crates and LSP plumbing.
  - No contributor workflow for tests/linting/xtask commands.
  - Severity: **moderate**.
  - Safe fix: **Yes** (expand README with status, architecture, and contributor commands).

### 9) Additional typos/wording issues
- **File:** `crates/parser/src/grammar.rs:84`
  - Message says “expected a name referencer” (awkward wording; likely should be “reference”).
  - Severity: **trivial**.
  - Safe fix: **Yes**.
