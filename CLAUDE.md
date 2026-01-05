# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Tasker is a desktop task management application built with **Tauri v2 + SvelteKit + SQLite**. It's a cross-platform native app that combines a Rust backend with a modern web-based frontend.

- [Project 문서](./PROJECT.md) 참고
- [History 문서](./history) 참고

## 규칙

- 변경사항이 있을시 history 디렉토리에 오늘 날짜 파일에 작업 로그를 남긴다.
  예를들어, 20260105 작업 내용은 `history/20260105.md` 에 작성한다.

## Development Commands

### Setup
```bash
npm install                    # Install frontend dependencies
```

### Development
```bash
npm run tauri dev             # Start dev server + launch Tauri app
                              # Frontend runs on http://localhost:1420 (strict port)
                              # Backend auto-reloads on Rust changes
                              # Frontend has HMR on ws://localhost:1421

npm run dev                   # Frontend only (for UI work)
npm run check                 # TypeScript + Svelte type checking
```

### Build
```bash
npm run tauri build           # Production build (creates platform-specific installer)
npm run build                 # Frontend build only (outputs to ./build)
```

### Environment Setup
Create `.env` file in project root for Google OAuth:
```env
GOOGLE_CLIENT_ID=your_client_id_here
GOOGLE_CLIENT_SECRET=your_client_secret_here
```

## Architecture Overview

### Frontend-Backend Communication (Tauri IPC)

**Frontend (TypeScript):**
```typescript
import { invoke } from '@tauri-apps/api/core';
const tasks = await invoke('get_tasks');
await invoke('add_task', { task: newTask });
```

**Backend (Rust):**
```rust
#[tauri::command]
fn get_tasks(state: State<AppState>) -> Result<Vec<Task>, String> {
    // Implementation
}
```

All commands are registered in `src-tauri/src/lib.rs` via `invoke_handler![]`.

### Database Architecture

**Location:** Platform-specific app data directory (e.g., `~/.local/share/com.cskim.tasker/`)

**Schema:**
- `tasks` table: id (UUID), title, completed, priority, category, due_date, **position** (for drag-drop order)
- `settings` table: key-value store for OAuth tokens and configuration

**Access Pattern:**
- All database operations go through `db.rs::Database` struct
- Uses `Mutex<Option<Database>>` for thread-safe access
- Each command opens a new SQLite connection (simple pooling)
- `update_task_order()` uses transactions for atomic batch updates

### State Management

**Backend State (Rust):**
```rust
struct AppState {
    db: Mutex<Option<Database>>  // Thread-safe DB access
}
```

**Frontend State (Svelte 5):**
- Uses Svelte 5 runes: `$state`, `$derived`
- No global state management library
- Component-local state with reactive computations
- Example: `let tasks = $state<Task[]>([]); let filtered = $derived(() => tasks.filter(...))`

### Google OAuth Flow

1. Frontend: `get_google_auth_url()` → Opens browser via `@tauri-apps/plugin-opener`
2. User authorizes → Redirected to `http://localhost:14123` with auth code
3. Frontend: `finish_google_auth(code)` → Exchanges code for tokens
4. Tokens stored in `settings` table (access_token, refresh_token)
5. Subsequent API calls use stored access token

**Scopes:** tasks, calendar, userinfo.email, userinfo.profile

## Key Implementation Details

### Drag-and-Drop Task Reordering

**Uses mouse events (not HTML5 DnD API)** due to Tauri compatibility:
```typescript
onmousedown={(e) => handleMouseDown(task.id, e)}
onmouseenter={() => handleMouseEnter(task.id)}
onmouseup={handleMouseUp}
```

When dropped, calls `update_task_order(ordered_ids: Vec<String>)` which:
1. Opens SQLite transaction
2. Updates `position` column for all tasks
3. Commits atomically

### SvelteKit Configuration

**Critical:** SSR must be disabled for Tauri:
```typescript
// src/routes/+layout.ts
export const ssr = false;
```

**Adapter:** `adapter-static` with SPA fallback mode

**Dev Port:** Must be `1420` (hardcoded in `tauri.conf.json`)

### Routing Structure

- `/` - Main task list with filters (All/Active/Completed)
- `/task/[id]` - Task detail editor (title, priority, category, due_date)
- `/calendar` - Month view with tasks on due dates
- `/settings` - OAuth connections and integrations

### Error Handling Pattern

**Backend:** All commands return `Result<T, String>`
```rust
.map_err(|e| format!("Database error: {}", e))?
```

**Frontend:** Try-catch with console.error or alert
```typescript
try {
    await invoke('add_task', { task });
} catch (e) {
    console.error('Failed to add task:', e);
}
```

## Common Patterns

### Adding a New Tauri Command

1. **Define command in Rust:**
```rust
// src-tauri/src/lib.rs or db.rs
#[tauri::command]
fn my_command(arg: String, state: State<AppState>) -> Result<MyType, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    // ... implementation
    Ok(result)
}
```

2. **Register in invoke_handler:**
```rust
.invoke_handler(tauri::generate_handler![
    existing_commands,
    my_command  // Add here
])
```

3. **Call from frontend:**
```typescript
const result = await invoke('my_command', { arg: 'value' });
```

### Adding Database Fields

1. **Modify table schema in `db.rs::Database::new()`:**
```rust
conn.execute(
    "ALTER TABLE tasks ADD COLUMN new_field TEXT",
    [],
)?;
```

2. **Update `Task` struct:**
```rust
pub struct Task {
    // existing fields...
    pub new_field: Option<String>,
}
```

3. **Update TypeScript interface:**
```typescript
interface Task {
    // existing fields...
    new_field?: string | null;
}
```

4. **Update SQL queries** in all relevant methods (get_tasks, add_task, update_task)

### Working with Svelte 5 Runes

**State:**
```typescript
let items = $state<Item[]>([]);  // Reactive state
items.push(newItem);              // Direct mutation
```

**Derived State:**
```typescript
let filtered = $derived(() => {
    return items.filter(i => i.active);
});
// Access as: filtered() or just filtered depending on context
```

**Effects:**
```typescript
$effect(() => {
    console.log('items changed:', items);
});
```

## Testing

Currently no automated tests. To test:
1. Run `npm run tauri dev`
2. Open DevTools (right-click → Inspect or Cmd+Option+I on macOS)
3. Check console for debug logs
4. Test features manually

## Known Issues & Workarounds

### Drag-and-Drop Not Working
- HTML5 DnD API doesn't work reliably in Tauri
- **Solution:** Use mouse events (`onmousedown`/`onmouseenter`/`onmouseup`)

### OAuth Redirect
- Redirect URI is `http://localhost:14123` (hardcoded)
- Must manually copy auth code from URL if redirect fails
- No auto-refresh for expired tokens (manual re-auth required)

### Port Conflicts
- Vite dev server MUST run on port 1420
- If blocked, kill process: `lsof -ti:1420 | xargs kill -9`

## File Locations

### Backend (Rust)
- `src-tauri/src/lib.rs` - Tauri setup, command registration, app state
- `src-tauri/src/db.rs` - Database layer (SQLite operations)
- `src-tauri/src/google.rs` - Google OAuth and Tasks API integration
- `src-tauri/tauri.conf.json` - Tauri configuration (window, build, permissions)

### Frontend (SvelteKit)
- `src/routes/+layout.svelte` - Sidebar navigation shell
- `src/routes/+page.svelte` - Main task list with drag-drop
- `src/routes/task/[id]/+page.svelte` - Task detail editor
- `src/routes/settings/+page.svelte` - OAuth connections
- `src/app.css` - Tailwind CSS imports

### Configuration
- `vite.config.js` - Vite bundler config (dev port: 1420)
- `svelte.config.js` - SvelteKit adapter (static SPA)
- `tsconfig.json` - TypeScript config
- `.env` - Environment variables (not in version control)

## Debugging

### Frontend Logs
- Open DevTools in Tauri window
- Or check terminal running `npm run tauri dev`

### Backend Logs
- Use `println!()` or `eprintln!()` in Rust code
- Appears in terminal running Tauri

### Database Inspection
```bash
# Find database location
# macOS: ~/Library/Application Support/com.cskim.tasker/
# Linux: ~/.local/share/com.cskim.tasker/
# Windows: %APPDATA%/com.cskim.tasker/

sqlite3 path/to/tasker.db
.schema tasks
SELECT * FROM tasks ORDER BY position;
```

## Dependencies

### Critical Dependencies
- **Tauri v2:** Desktop app framework (must match across CLI and runtime)
- **SvelteKit 2.9 + Svelte 5:** Frontend framework (breaking changes from v4)
- **rusqlite 0.38:** SQLite bindings (thread-safety via Mutex)
- **reqwest:** HTTP client for OAuth (requires features: json, form, query)

### Style Dependencies
- **Tailwind CSS v4:** Uses new Vite plugin (`@tailwindcss/vite`)
- **Lucide Svelte:** Icon components (tree-shakeable)

## Korean Language Support

This project is primarily documented in Korean (Korean task titles, UI text, commit messages). When working on this codebase:
- Code comments can be in Korean
- Commit messages are in Korean
- User-facing text is in Korean
- Variable names and function names remain in English