# Improvement Plan: rust-test-ms → production-ready auth MS

> **Цель:** превратить учебный `rust-test-ms` в полноценный переиспользуемый auth микросервис, который можно за пару минут подключить к любому проекту.

> **Статус:** плановый документ. Без кода — только roadmap, обсуждение направления и оценки.

---

## Решения (применены 2026-06-12, 8 ответов)

| #  | Вопрос                         | Ответ                                       | Влияние на план                                                          |
| -- | ------------------------------ | ------------------------------------------- | ------------------------------------------------------------------------ |
| 1  | Лицензия                       | Для личного использования                   | `LICENSE` (MIT) добавляется в Phase 0                                    |
| 2  | Скоуп                          | OAuth2-провайдер + свой UI в перспективе    | Phase 4: фокус на API + admin API endpoints; UI — позже                 |
| 3  | Email                          | Встроенный, но pluggable                    | Phase 2: trait `EmailSender` + SMTP-impl + file-based для dev            |
| 4  | Tenants                        | Пока не понимаю                              | Phase 5 (multi-tenancy) **убран**, перенесён в **Future**               |
| 5  | gRPC                           | Потом добавим                                | Убран из Phase 4 → **Future** (Phase 7+)                                 |
| 6  | Migration runner               | Не очень понимаю                             | Дефолт: `diesel migration run` в entrypoint; override через `RUN_MIGRATIONS=false` для K8s Job-паттерна |
| 7  | OpenAPI UI                     | **Scalar**                                  | Phase 4: Scalar вместо Swagger UI                                        |
| 8  | Стек                           | **Axum вместо Rocket**                       | **Phase 0 полностью переписан** — переезд HTTP слоя                      |

---

## Phase 0: Rewrite HTTP layer on Axum — **~15-25 ч**

**Зачем:** современный, async-first, совместим с tower/tokio-экосистемой. Service + repository слои остаются как есть (от Rocket не зависят).

**Scope:**

### 0.1 `Cargo.toml`
- ❌ `rocket = "0.5.0-rc.2"`, `rocket_sync_db_pools`, `rocket_db_pools`
- ✅ `axum = "0.7"`, `tower = "0.4"`, `tower-http = { features = ["trace","cors","limit"] }`
- ✅ `tokio = { version = "1", features = ["full"] }`
- ✅ `async-trait` (для DI в extractors)
- Сохраняем: `diesel = "1.4"`, `diesel_migrations`, `argon2`, `jsonwebtoken`, `dotenvy`, `serde`, `chrono`, `uuid`

### 0.2 HTTP handlers (`src/controllers/`)
Переписать все 12 endpoints с Rocket на Axum:

| Rocket                                       | Axum                                                       |
| -------------------------------------------- | ---------------------------------------------------------- |
| `#[get("/users")]`                           | `async fn users(State(s): State<Arc<AppState>>) -> ...`    |
| `Json<Vec<UserDto>>`                         | `Json<Vec<UserDto>>` (тот же тип, axum имеет свой)         |
| `Status::NotFound`                           | `StatusCode::NOT_FOUND`                                    |
| `Result<Json<T>, Status>`                    | `Result<Json<T>, AppError>` с `IntoResponse`               |
| `web::Data<T>` / `State<T>`                  | `axum::extract::State<Arc<T>>`                             |
| `From<...> for TokenAuth` guard              | `impl FromRequestParts<Arc<AppState>> for AuthUser`         |
| `From<...> for AdminAuth` guard              | `impl FromRequestParts<...> for AdminUser`                  |
| `routes![users, get_user, ...]`              | `Router::new().route("/users", get(users))...`             |
| `rocket::build().mount("/", routes![...])`   | `axum::serve(listener, app)`                               |

**Controllers list** (12 шт.):
- `auth/login.rs` — POST `/auth/login`
- `auth/register.rs` — POST `/auth/register`
- `auth/refresh.rs` — POST `/auth/refresh`
- `auth/change_password.rs` — POST `/auth/change-password`
- `users/get.rs` — GET `/users`
- `users/get_by_id.rs` — GET `/users/<id>`
- `users/create.rs` — POST `/users`
- `users/update.rs` — PATCH `/users/<id>`
- `users/delete.rs` — DELETE `/users/<id>`
- `roles/get.rs` — GET `/roles`
- `roles/create.rs` — POST `/roles`
- `roles/update.rs` — PATCH `/roles/<id>`
- `roles/delete.rs` — DELETE `/roles/<id>`

### 0.3 AppState, error type
- `pub struct AppState { pub pool: DbPool, pub jwt_secret: String, pub jwt_access_ttl: i64, pub jwt_refresh_ttl: i64 }` — обёрнут в `Arc`
- `enum AppError { Db(...), NotFound, Unauthorized, Forbidden, BadRequest(String), Internal }` с `impl IntoResponse`

### 0.4 Middleware
- `tower_http::trace::TraceLayer::new_for_http()` — structured request logging
- `tower_http::cors::CorsLayer` — permissive defaults (настроим в Phase 3)

### 0.5 Server bootstrap
```rust
// main.rs
let pool = init_pool(&database_url)?;
let state = Arc::new(AppState { pool, ... });
let app = Router::new()
    .route("/auth/login", post(auth::login))
    .route("/auth/register", post(auth::register))
    // ...
    .layer(TraceLayer::new_for_http())
    .with_state(state);
let listener = TcpListener::bind("0.0.0.0:8000").await?;
axum::serve(listener, app).await?;
```

### 0.6 Tests
- `tests/integration.rs` — поднимаем `TestApp` с in-memory SQLite или test Postgres
- Smoke-тесты: register → login → access protected → refresh → logout
- Используем `tower::ServiceExt::oneshot` для in-process запросов

### 0.7 README + LICENSE
- README: cargo run, env vars, curl-примеры
- LICENSE: MIT (для персонального использования)
- .gitignore, .dockerignore: убрать `target/`, добавить `*.swp`

**Поставка:** 1 PR с ~4-6 squash-коммитами:
1. `chore(deps): swap rocket for axum + tower stack`
2. `feat(http): port auth controllers to axum`
3. `feat(http): port user/role controllers to axum`
4. `feat(http): axum AppState, error type, server bootstrap`
5. `test: integration tests for all 12 endpoints`
6. `docs: README rewrite + MIT LICENSE`

**Out of scope** (Phase 1+):
- CI, Docker, structured logging
- Data model изменения (email, audit, etc.)
- OAuth2, OpenAPI/Scalar
- Diesel → sqlx — отложено в **Future** (Phase 10)

---

## Phase 1: Quality infrastructure — **~25-35 ч**

| Что                                        | Подробности                                                  |
| ------------------------------------------ | ------------------------------------------------------------ |
| **Unit + integration tests**               | `cargo test` в CI, coverage ≥ 80% на services/, ≥ 60% на repos/ |
| **CI** (GitHub Actions)                    | `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`, `cargo build --release`, `cargo audit` |
| **Dockerfile** (multi-stage)               | `rust:1.x-slim` builder → `debian:bookworm-slim` runtime, non-root user, `tini` init |
| **docker-compose.yml**                     | `auth-ms` + `postgres:16` + `mailhog` (dev SMTP)             |
| **Structured logging** (`tracing`)         | JSON-вывод, request_id, span propagation, `RUST_LOG` env     |
| **Health endpoints**                       | `GET /livez`, `GET /readyz`                                  |
| **Pre-commit hooks** (опц.)                | `cargo fmt`, `cargo clippy`, `cargo check`                   |

**Поставка:** 1 PR

---

## Phase 2: Data model v2 — **~25-30 ч**

### 2.1 EmailSender trait (pluggable)
```rust
#[async_trait]
pub trait EmailSender: Send + Sync {
    async fn send(&self, msg: EmailMessage) -> Result<(), EmailError>;
}
```
Реализации:
- `SmtpSender` — через `lettre` (production)
- `FileSender` — пишет в `./tmp/emails/<id>.eml` (dev/test)
- (Future) `SesSender`, `SendGridSender`, etc.

Конфиг через env: `EMAIL_BACKEND=smtp|file`, `SMTP_URL=...`, `SMTP_FROM=...`

### 2.2 Schema migration (Diesel)
```sql
-- Up
ALTER TABLE users ADD COLUMN email VARCHAR(255) NOT NULL DEFAULT '';
CREATE UNIQUE INDEX users_email_unique ON users(LOWER(email));
ALTER TABLE users ADD COLUMN created_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
ALTER TABLE users ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
ALTER TABLE users ADD COLUMN deleted_at TIMESTAMPTZ;        -- soft delete
ALTER TABLE users ADD COLUMN email_verified_at TIMESTAMPTZ;
ALTER TABLE users ADD COLUMN password_reset_token VARCHAR(64);
ALTER TABLE users ADD COLUMN password_reset_expires_at TIMESTAMPTZ;

-- Multi-role: many-to-many user_roles
CREATE TABLE user_roles (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role_id UUID NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    granted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    granted_by UUID REFERENCES users(id),
    PRIMARY KEY (user_id, role_id)
);

-- Permissions: many-to-many role_permissions
CREATE TABLE permissions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(64) NOT NULL UNIQUE,     -- "users.read", "admin.write"
    description TEXT
);
CREATE TABLE role_permissions (
    role_id UUID NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    permission_id UUID NOT NULL REFERENCES permissions(id) ON DELETE CASCADE,
    PRIMARY KEY (role_id, permission_id)
);

-- Refresh token store (rotation + reuse detection)
CREATE TABLE refresh_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash VARCHAR(64) NOT NULL UNIQUE,    -- SHA-256 of the actual token
    issued_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL,
    revoked_at TIMESTAMPTZ,
    replaced_by UUID REFERENCES refresh_tokens(id)
);
CREATE INDEX refresh_tokens_user_idx ON refresh_tokens(user_id);

-- Audit log
CREATE TABLE audit_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    actor_id UUID REFERENCES users(id),
    actor_ip INET,
    action VARCHAR(64) NOT NULL,            -- "user.login", "user.password.change"
    target_type VARCHAR(32),                -- "user", "role", "token"
    target_id UUID,
    metadata JSONB
);
CREATE INDEX audit_events_actor_idx ON audit_events(actor_id, occurred_at DESC);
```

### 2.3 Repositories
- `UserRepository::find_by_email(email)`, `soft_delete(id)`, `verify_email(id)`
- `RefreshTokenRepository::insert(...)`, `find_by_hash(...)`, `revoke(id)`, `mark_replaced(...)`
- `AuditRepository::log(event)` — best-effort, не блокирует запрос при ошибке

### 2.4 Migration runner (по умолчанию)
```rust
// main.rs
if env::var("RUN_MIGRATIONS").unwrap_or_else(|_| "true".into()) == "true" {
    run_migrations(&pool)?;
}
```
- `RUN_MIGRATIONS=true` (default) — для локалки и dev
- `RUN_MIGRATIONS=false` — для K8s, где миграции крутятся отдельным Job/init-контейнером

### 2.5 Backward compatibility
- Все старые endpoints работают, но `email` теперь required при register (dual-write: генерим placeholder email если не указан, требуем верификацию)
- Bootstrap admin: `admin@local`, пароль из `INITIAL_ADMIN_PASSWORD` env

**Поставка:** 1 PR (большой, ~20 файлов), с явным `BREAKING:` в коммит-сообщении и dual-period поддержкой

---

## Phase 3: Auth security — **~35-50 ч**

| Тема                                | Реализация                                                   |
| ----------------------------------- | ------------------------------------------------------------ |
| **Argon2id** с явными параметрами   | `Argon2::new(Algorithm::Argon2id, Version::V0x13, Params::new(19456, 2, 1))` |
| **Rate limiting**                   | `tower-governor` или `redis-cell`: 5 login/15min per IP, 100 req/min per user |
| **Refresh rotation + reuse detect** | При refresh: revoke old + issue new, в транзакции. Если `revoked_at` уже стоит — revoken ВСЮ цепочку пользователя → force re-login |
| **JWT claims**                      | `iss`, `aud`, `sub`, `iat`, `nbf`, `exp`, `jti` (uuid), `roles`, `scope` |
| **JWT signing**                     | RS256 (генерим ключи в setup, secret в env), kid в header    |
| **Password policy**                 | Min 12 chars, 3 of {lower, upper, digit, symbol}, проверка при register/change |
| **HIBP breach check** (опц.)        | k-anonymity API: SHA-1 password → первые 5 chars → проверить suffix |
| **CORS**                            | `CorsLayer::default().allow_origin(["https://..."])` (env-configurable) |
| **Security headers**                | `tower-http::set_header` для `X-Content-Type-Options`, `X-Frame-Options`, `Referrer-Policy`, `Strict-Transport-Security` (в HTTPS-режиме) |
| **Audit hooks**                     | `AuditMiddleware` логирует auth-события, можно подписаться на произвольные actions |

**Out of scope (Future):** MFA/TOTP, WebAuthn, SAML, LDAP

**Поставка:** 1 PR, ~15-20 файлов

---

## Phase 4: API standards — **~50-70 ч**

### 4.1 Versioning + OpenAPI
- Все endpoints под `/v1/...` (alias без префикса → 301 redirect)
- OpenAPI 3.1 спека через `utoipa` макросы на каждом handler
- **Scalar UI** на `/docs` (self-hosted HTML+JS, см. https://github.com/scalar/scalar)
- Спека экспортируется в `openapi.json` (download), подключается к SDK-генератору

### 4.2 OAuth2 / OIDC (как **identity provider**)
- **Authorization Code + PKCE** (для SPA / native apps)
- **Client Credentials** (для service-to-service)
- **Refresh Token** (с rotation из Phase 3)
- **Discovery**: `GET /.well-known/openid-configuration`, `GET /.well-known/oauth-authorization-server`
- **JWKS**: `GET /.well-known/jwks.json` (публичные ключи для верификации)
- **UserInfo**: `GET /v1/userinfo` (OIDC)
- **Token Introspection**: `POST /v1/oauth/introspect` (RFC 7662)
- **Token Revocation**: `POST /v1/oauth/revoke` (RFC 7009)

### 4.3 Client management
- `POST /v1/oauth/clients` — register client (получает `client_id` + `client_secret`)
- `GET /v1/oauth/clients`, `GET/PATCH/DELETE /v1/oauth/clients/<id>`
- Scopes: `openid`, `profile`, `email`, `offline_access`, custom
- Redirect URIs validation (exact match)
- Client types: `confidential`, `public`

### 4.4 Admin API (для будущего UI)
- `GET/POST/PATCH/DELETE /v1/admin/users`
- `GET/POST/PATCH/DELETE /v1/admin/roles`
- `GET/POST/PATCH/DELETE /v1/admin/clients`
- `GET /v1/admin/audit` (filter, paginate, export CSV)
- `GET /v1/admin/stats` (active users, tokens issued, login rate)
- Все требуют scope `admin:read` или `admin:write`

### 4.5 Прочее
- Webhooks (HTTP callbacks на события: user.created, role.assigned)
- Bulk operations (`POST /v1/admin/users/bulk-create`)
- Filter / sort / search на list endpoints (`?filter[email]=*@example.com&sort=-created_at`)
- Field selection (`?fields=id,email,name`)
- Cursor pagination (опц., для больших списков)
- Idempotency keys (`Idempotency-Key` header, TTL 24h)
- **Generated SDKs:**
  - Rust (через `progenitor` или `openapi-gen`)
  - TypeScript (через `openapi-typescript-codegen` или `openapi-generator`)
  - Python (через `openapi-python-client` или `openapi-generator`)

**Поставка:** 4-5 PR-ов:
- `feat(api): v1 versioning + utoipa OpenAPI spec`
- `feat(oauth): authorization code + PKCE + client credentials`
- `feat(admin): admin API endpoints + audit query`
- `feat(perf): filter, sort, search, field selection`
- `feat(sdk): generate Rust/TS/Python SDKs from OpenAPI`

---

## Phase 5: Production ops — **~35-45 ч**

| Категория       | Что                                                              |
| --------------- | ---------------------------------------------------------------- |
| Observability   | `/livez`, `/readyz`, `/healthz` (deep), Prometheus `/metrics`, OpenTelemetry traces, structured JSON logs |
| Graceful        | `tokio::signal::ctrl_c` → drain → shutdown timeout, in-flight request tracker |
| Docker          | multi-stage, non-root, multi-arch (amd64 + arm64), cosign-подпись, SBOM (syft) |
| K8s             | **Helm chart** в `deploy/helm/auth-ms/`: deployment, service, ingress, configmap, secret, hpa, poddisruptionbudget, networkpolicy, serviceaccount, servicemonitor |
| Docs            | `mdbook` в `docs/`: quickstart, concepts, deployment, API reference, security model, migration guide |
| Performance     | `criterion` benchmarks для Argon2 verify, JWT sign/verify, refresh flow |
| Load testing    | `k6` скрипты в `tests/load/`: login burst, refresh storm, sustained 1k RPS |
| CI/CD           | GitHub Actions: build → test → audit → docker build+push → helm lint → trivy scan → sign |
| Security scan   | `cargo audit`, `cargo deny`, `trivy fs`, `grype`                 |

**Поставка:** 5 PR-ов

---

## Future (Phase 7+, по запросу)

| #  | Что                              | Триггер / приоритет                                          |
| -- | -------------------------------- | ------------------------------------------------------------ |
| 7  | gRPC API                         | Когда появится внутренний сервис, которому нужна низкая latency |
| 8  | Admin UI                         | Phase 4 готов, не хочется Postman'ом рулить                  |
| 9  | Multi-tenancy                    | Когда проект станет B2B / SaaS                               |
| 10 | Diesel → sqlx (async)            | Когда Axum + sync-Diesel начнёт быть узким местом           |
| 11 | MFA / TOTP / WebAuthn            | Когда аудитория вырастет или требования compliance          |
| 12 | SAML / LDAP                      | Enterprise-клиенты                                           |
| 13 | Geo-distributed / multi-region   | Глобальный launch                                            |
| 14 | Email templates engine          | Когда разных писем станет > 5 и хочется версионировать       |

---

## Оценки сводно

| Фаза   | Часы          | PR-ов | Готовность |
| ------ | ------------- | ----- | ---------- |
| 0      | 15-25         | 1     | 🟡 Нужен аппрув перед стартом |
| 1      | 25-35         | 1     | 🟢 После 0 |
| 2      | 25-30         | 1     | 🟢 После 1 |
| 3      | 35-50         | 1     | 🟢 После 2 |
| 4      | 50-70         | 4-5   | 🟢 После 3 |
| 5      | 35-45         | 5     | 🟢 После 4 |
| **Итого** | **~200-280**  | **~14** | **~3-5 месяцев** (part-time) |

---

## Следующий шаг

**Phase 0 (Axum rewrite) — стартую как draft PR** сразу после твоего подтверждения этого плана. Без подтверждения кода не пишу.

Что будет в draft PR:
- `Cargo.toml` обновлён (rocket → axum)
- Все 12 handlers переписаны на axum
- AppState + error type
- Integration tests (smoke)
- README + LICENSE

После того как ты смерджишь Phase 0 → Phase 1 (тесты, CI, Docker) → Phase 2 (data model v2) → ...

Готов начинать?
