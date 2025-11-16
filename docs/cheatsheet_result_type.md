# Rust `Result<T, E>` Cheatsheet  
*A practical, visual guide from beginner to advanced*

---

## 1) What is `Result`?
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
Use `Result` for operations that can fail (file I/O, parsing, networking).  
- `Ok(T)` holds a success value.
- `Err(E)` holds an error.

> Rule of thumb: If a function can fail in a recoverable way, return `Result`.

---

## 2) Minimal starter: creating and matching
```rust
fn parse_port(s: &str) -> Result<u16, std::num::ParseIntError> {
    let n: u16 = s.parse()?; // use `?` once you know it; see §6
    Ok(n)
}

match parse_port("8080") {
    Ok(p) => println!("Port = {p}"),
    Err(e) => eprintln!("Invalid port: {e}"),
}
```

---

## 3) “When to use which `unwrap`?” — quick answers

| Method                  | Use when…                                                                 | Behavior on error |
|---                      |---                                                                        |---|
| `unwrap()`              | You **know** it cannot fail (test/prototyping)                            | Panics with generic msg |
| `expect("context")`     | It *shouldn’t* fail and you want a **clear panic message** if it does     | Panics with your message |
| `unwrap_or(default)`    | You have a **cheap default** value on error                               | Returns default |
| `unwrap_or_else(f)`     | You need to **compute** the default or log using the error                | Calls `f(err)` |
| `unwrap_or_default()`   | `T: Default` and default is fine                                          | Uses `T::default()` |

> Production code: prefer **`?`** or **propagate/handle** errors. Keep `unwrap/expect` for tests, quick tools, or verified invariants.

---

## 4) Idiomatic handling patterns

### Pattern match (full control)
```rust
match read_to_string(path) {
    Ok(text) => process(text),
    Err(e) if e.kind() == std::io::ErrorKind::NotFound => create_file_with_defaults(path),
    Err(e) => return Err(e.into()),
}
```

### Map success or error
```rust
let len: Result<usize, io::Error> =
    fs::read_to_string(path).map(|s| s.len());

let display_err: Result<String, String> =
    fs::read_to_string(path).map_err(|e| format!("I/O failed: {e}"));
```

### Fallbacks
```rust
let config = load_config().unwrap_or_default(); // OK if default is acceptable
let config = load_config().unwrap_or_else(|e| {
    eprintln!("Using defaults due to: {e}");
    Config::default()
});
```

---

## 5) Combinators you’ll use a lot

| Combinator             | Purpose                                                     | Example |
|---                     |---                                                          |---|
| `map`                  | Transform `Ok(T) -> Ok(U)`                                  | `res.map(|x| x + 1)` |
| `map_err`              | Transform error type                                        | `res.map_err(MyError::from)` |
| `and_then`             | Chain `Result`-producing ops                                | `parse(s).and_then(open_port)` |
| `or_else`              | Provide alternate computation on error                      | `res.or_else(|_| try_default())` |
| `inspect` / `inspect_err` (1.62+) | Peek for logging                              | `res.inspect(|v| println!("{v}"))` |
| `ok()`                 | `Result<T,E> -> Option<T>` (drop error)                     | `res.ok()` |
| `err()`                | `Result<T,E> -> Option<E>` (drop ok)                        | `res.err()` |
| `transpose()`          | `Option<Result<T,E>> -> Result<Option<T>,E>`                | see §9 |
| `as_deref()`/`as_ref()`| Work with references without moving                         | `res.as_ref().map(|s| s.len())` |

---

## 6) The `?` operator: ergonomic propagation
Use `?` inside functions that return `Result` (or `Option`).

```rust
fn total_len(paths: &[PathBuf]) -> Result<usize, io::Error> {
    let mut sum = 0;
    for p in paths {
        let s = fs::read_to_string(p)?; // if Err, returns early with that Err
        sum += s.len();
    }
    Ok(sum)
}
```

### Adding context with `expect` vs structured error
- For one-off invariants: `expect("config path must exist")`.
- For real error paths: convert/augment the error and **propagate**.

```rust
fn read_cfg(path: &Path) -> Result<String, io::Error> {
    let s = fs::read_to_string(path) // no `expect` here
        .map_err(|e| io::Error::new(e.kind(), format!("reading {}: {e}", path.display())))?;
    Ok(s)
}
```

---

## 7) Designing function signatures

### Return concrete, specific errors when possible
```rust
fn read_user() -> Result<User, io::Error> { /* ... */ }
fn parse_user(s: &str) -> Result<User, serde_json::Error> { /* ... */ }
```

### Or define a domain error enum (scales well)
```rust
#[derive(Debug)]
enum AppError {
    Io(std::io::Error),
    BadConfig(String),
    Parse(serde_json::Error),
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self { AppError::Io(e) }
}
impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self { AppError::Parse(e) }
}

type AppResult<T> = Result<T, AppError>;
```

Now you can `?` from I/O and parsing seamlessly:
```rust
fn load_user(path: &Path) -> AppResult<User> {
    let raw = fs::read_to_string(path)?;     // AppError::Io
    let user = serde_json::from_str(&raw)?;  // AppError::Parse
    Ok(user)
}
```

---

## 8) Clean error types with `thiserror` (ergonomic, readable)
```toml
# Cargo.toml
[dependencies]
thiserror = "1"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

```rust
use thiserror::Error;

#[derive(Debug, Error)]
enum AppError {
    #[error("I/O: {0}")]
    Io(#[from] std::io::Error),

    #[error("invalid config: {0}")]
    BadConfig(String),

    #[error("JSON parse: {0}")]
    Parse(#[from] serde_json::Error),
}

type AppResult<T> = Result<T, AppError>;
```

- `#[from]` auto-implements `From<...>` so `?` just works.
- `#[error(...)]` controls display messages.

---

## 9) `Result` ↔ `Option` and collection tricks

### Convert between them
```rust
let maybe_num: Option<i32> = Some(5);
let must_parse: Result<i32, _> = maybe_num.ok_or_else(|| "missing number".to_string());

let maybe_err: Option<&str> = Err("oof").err(); // Some("oof")
```

### `transpose` when nesting `Option` + `Result`
```rust
fn maybe_load(flag: bool) -> Option<Result<String, io::Error>> {
    if flag { Some(fs::read_to_string("file.txt")) } else { None }
}

let res: Result<Option<String>, io::Error> = maybe_load(true).transpose();
```

### Collecting many `Result`s
```rust
let inputs = vec!["1", "2", "x"];
let parsed: Result<Vec<i32>, _> = inputs.into_iter().map(str::parse).collect();
// Err at the first invalid item ("x")
```

### Short-circuiting iterator helpers
```rust
files.into_iter()
    .try_for_each(|p| -> io::Result<()> {
        let s = fs::read_to_string(&p)?;
        process(&s)?;
        Ok(())
    })?;
```

---

## 10) Choosing the right strategy (decision guide)

1. **Can you recover here?**  
   - Yes ➜ handle `Err` locally (retry, fallback, default).  
   - No  ➜ **propagate** with `?`.

2. **Is failure a programming bug, not an environment issue?**  
   - Yes ➜ `expect("explain invariant")` or refactor to avoid fallible call.  
   - No  ➜ return/propagate a meaningful error.

3. **Will callers need details?**  
   - Yes ➜ keep rich error types (`enum`, `thiserror`).  
   - No  ➜ collapse to simpler errors (string or `anyhow::Error` in apps).

---

## 11) Testing with `Result`
```rust
#[test]
fn parses_ok() -> Result<(), Box<dyn std::error::Error>> {
    let n: u32 = "42".parse()?;
    assert_eq!(n, 42);
    Ok(())
}
```

---

## 12) Common pitfalls and fixes

- **Overusing `unwrap()` in library code**  
  → Replace with `?` or return `Result`.

- **Losing error context**  
  → Convert and annotate (`map_err`, custom enums, `thiserror` messages).

- **Huge error enums everywhere**  
  → Keep domain-local enums; convert at module boundaries.

- **Catching and logging too early**  
  → Prefer bubbling up; log once at the top-level with full context.

---

## 13) Application-level ergonomic errors (`anyhow`)
```toml
[dependencies]
anyhow = "1"
```

```rust
use anyhow::{Context, Result};

fn run() -> Result<()> {
    let raw = std::fs::read_to_string("config.json")
        .context("reading config.json")?;
    let cfg: Config = serde_json::from_str(&raw)
        .context("parsing config.json")?;
    do_work(cfg).context("running pipeline")?;
    Ok(())
}
```

> Prefer `thiserror` in libraries; `anyhow` in binaries.

---

## 14) Mini cookbook

### Retry transient errors
```rust
fn fetch_with_retry(url: &str, attempts: usize) -> Result<String, reqwest::Error> {
    let mut last = None;
    for _ in 0..attempts {
        match reqwest::blocking::get(url)?.error_for_status() {
            Ok(resp) => return resp.text(),
            Err(e) => last = Some(e),
        }
    }
    Err(last.unwrap())
}
```

### Validate then convert
```rust
fn non_empty(s: String) -> Result<String, &'static str> {
    if s.trim().is_empty() { Err("empty string") } else { Ok(s) }
}
```

### Custom display for user-facing messages
```rust
#[derive(Debug)]
struct HumanErr(&'static str);

impl std::fmt::Display for HumanErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::error::Error for HumanErr {}
```

---

## 15) TL;DR quick reference

- **Prefer**: return `Result`, use `?`, add context, choose good error types.
- **Use `unwrap/expect`** only for invariants, tests, or quick experiments.
- **Ecosystem**: `thiserror` (typed), `anyhow` (app-level), `eyre` (alt).

---

### Pocket examples (copy/paste)

```rust
// 1) Propagate with context (std)
fs::read_to_string(path)
    .map_err(|e| io::Error::new(e.kind(), format!("{}: {e}", path.display())))?;

// 2) Domain error + From conversions
#[derive(Debug, thiserror::Error)]
enum E {
    #[error("I/O: {0}")] Io(#[from] io::Error),
    #[error("Parse: {0}")] Parse(#[from] serde_json::Error),
}

// 3) Collect results
let values: Vec<i32> = lines.iter().map(|s| s.parse()).collect::<Result<_, _>>()?;

// 4) Choose unwrap
let port = std::env::var("PORT").expect("PORT env var must be set");
```

---
