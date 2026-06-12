# 🗺 План развития `Rust-auth-ms` → полноценный Auth МС

> **Цель:** превратить существующий CRUD-над-пользователями в переиспользуемый микросервис авторизации с управлением ролями, который можно подключить к любому проекту «как зависимость».
>
> **Ключевая идея:** через месяц ты пишешь новый проект, добавляешь его в docker-compose рядом с этим МС, настраиваешь 5 env-переменных — и получаешь полный OAuth2/OIDC + роли + аудит без копипасты.

---

## 📊 Текущее состояние (найдено при разведке)

### ✅ Что уже хорошо
- **Стек:** Rocket 0.5.0-rc.2 + Diesel 1.4.5 + PostgreSQL + Argon2 + jsonwebtoken — зрелые, безопасные выборы
- **Архитектура:** классическая 3-слойная (controllers → services → repositories), разделение ответственности есть
- **Auth flow:** JWT access + refresh (HS256, env-конфигурируемые TTL)
- **12 эндпоинтов** работают: auth (login/register/refresh/change-password), users (CRUD + list), roles (CRUD + list)
- **Guards:** TokenAuth (валидный JWT) + AdminAuth (JWT + role_id)
- **Схема БД:** users + roles с UUID PK
- **Bootstrap:** авто-создание роли `admin` и пользователя `admin/admin123` при первом запуске
- **Error handling:** CustomError → ErrorResponse с HTTP-кодами
- **Пагинация:** size/page → total_count/total_pages
- **~1000 строк** Rust — компактно, понятно

### ❌ Критические пробелы

| Категория | Что отсутствует |
|---|---|
| **Тестирование** | Ноль тестов: ни unit, ни integration, ни test-БД |
| **DevOps** | Нет CI (GitHub Actions), Dockerfile, docker-compose, миграционного runner'а |
| **Документация** | Нет README, .env.example, LICENSE, OpenAPI/Swagger |
| **Observability** | Нет логирования (только `.expect()` → panic), метрик, трейсинга |
| **Безопасность** | Нет rate-limit, brute-force защиты, refresh-rotation, blacklist'а токенов |
| **Email** | Поле `email` отсутствует → логин по `name` (не уникален, неудобен) |
| **Верификация** | Нет email-verify, password-reset, MFA/TOTP |
| **Стандарты** | Нет OAuth2/OIDC, нет `/v1/` versioning, нет PKCE |
| **Мульти-тенантность** | Single-tenant, нет изоляции по проектам |
| **Production** | Нет K8s-манифестов, Helm chart, graceful shutdown, health-checks |
| **Качество кода** | `.expect()` повсюду, `routes.rs` — мёртвый код, устаревшие зависимости |
| **RBAC** | Роль одна на пользователя, нет permission'ов, нет many-to-many |
| **Аудит** | Нет лога «кто что когда сделал» |
| **Секреты** | `SECRET_KEY` без валидации, дефолтный `admin123` — prod-риск |

---

## 🏗 План: 6 фаз, каждая = mergeable PR

Каждая фаза заканчивается **работающим, протестированным, обратно совместимым** состоянием. Если что-то в середине пойдёт не так — откатываем один PR, остальное стоит.

### Phase 0 — Гигиена и зависимости
**Цель:** сделать репо пригодным для стороннего разработчика. _Не ломает API._

| # | Задача | Трудозатраты |
|---|---|---|
| 0.1 | Переименовать пакет: `rust-test-ms` → `auth-ms` | 5 мин |
| 0.2 | Bump: Rocket 0.5.0-rc.2 → 0.5.1, Diesel 1.4.5 → 2.2.x, обновить остальные | 1-2 ч |
| 0.3 | `README.md` — quickstart, env, API, deployment, contributing | 2 ч |
| 0.4 | `.env.example` с описанием каждой переменной | 30 мин |
| 0.5 | `LICENSE` (MIT или Apache-2.0 — на твой выбор) | 5 мин |
| 0.6 | `CONTRIBUTING.md`, `CHANGELOG.md`, `SECURITY.md` | 1 ч |
| 0.7 | `.gitignore` — добавить `.env`, `target/`, IDE-мусор | 5 мин |
| 0.8 | Удалить мёртвый `src/routes.rs` (он не используется в `main.rs`) | 5 мин |
| 0.9 | Поправить `package.metadata` в `Cargo.toml` (repo, license, description) | 10 мин |

**PR:** `chore: phase-0 — repo hygiene, dep bumps, docs`
**Тесты:** `cargo build` + `cargo run` (smoke)

---

### Phase 1 — Качество кода и DevOps
**Цель:** сделать так, чтобы можно было спать спокойно, делая изменения.

| # | Задача | Трудозатраты |
|---|---|---|
| 1.1 | **Unit-тесты** во всех модулях: `helpers/`, `error/`, `wrappers/`, `pagination/` | 4-6 ч |
| 1.2 | **Integration-тесты:** `tests/auth.rs`, `tests/users.rs`, `tests/roles.rs` + testcontainers (postgres) | 6-8 ч |
| 1.3 | **CI:** `.github/workflows/ci.yml` — check, test, clippy, fmt, audit, coverage (cargo-tarpaulin) | 3 ч |
| 1.4 | **Pre-commit:** `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo deny` | 1 ч |
| 1.5 | **Dockerfile:** multi-stage (builder → distroless/cc-debian12), non-root, healthcheck | 2 ч |
| 1.6 | **docker-compose.yml:** `auth-ms` + `postgres:16-alpine` для dev | 1 ч |
| 1.7 | **Makefile/justfile:** `dev`, `test`, `lint`, `build`, `docker-build`, `migrate` | 1 ч |
| 1.8 | **Логирование:** заменить все `.expect()` на `tracing` + proper error propagation | 4-6 ч |
| 1.9 | **Config validation:** `figment` или самописный валидатор env при старте | 2-3 ч |
| 1.10 | **Error story:** `thiserror` для всех доменных ошибок, единый error chain | 2 ч |
| 1.11 | Dependabot/Renovate config | 30 мин |
| 1.12 | Code coverage badge в README | 30 мин |

**PR:** `feat: phase-1 — tests, CI, Docker, structured logging`
**Coverage target:** 70%+
**Обратная совместимость:** 100%

---

### Phase 2 — Модель данных v2
**Цель:** подготовить схему к реальному использованию. _Ломает API — нужна стратегия миграции._

| # | Задача | Трудозатраты |
|---|---|---|
| 2.1 | Добавить `email` (unique, citext, validated) — заменить `name` как login identifier | 3 ч |
| 2.2 | `username` (unique) оставить как legacy + alias на `email` | 2 ч |
| 2.3 | `created_at`, `updated_at` (auto), `deleted_at` (soft delete) на users + roles | 2 ч |
| 2.4 | `is_active` flag + `disabled_reason` (текстом) | 1 ч |
| 2.5 | `email_verified_at` + `password_changed_at` | 1 ч |
| 2.6 | **Many-to-many:** `user_roles (user_id, role_id, granted_at, granted_by)` — много ролей на юзера | 3 ч |
| 2.7 | **Permissions:** `permissions`, `role_permissions (role_id, permission_id)` | 3 ч |
| 2.8 | **Refresh tokens:** `refresh_tokens (id, user_id, token_hash, family_id, expires_at, revoked_at, replaced_by)` | 4 ч |
| 2.9 | **Email verification:** `email_verification_tokens (token, user_id, expires_at, used_at)` | 1 ч |
| 2.10 | **Password reset:** `password_reset_tokens (token, user_id, expires_at, used_at)` | 1 ч |
| 2.11 | **Audit log:** `audit_log (id, actor_user_id, action, resource_type, resource_id, ip, ua, metadata JSONB, created_at)` | 2 ч |
| 2.12 | Индексы: unique(email), unique(username), index на FK, partial index `WHERE deleted_at IS NULL` | 1 ч |
| 2.13 | Backfill: admin → `admin@example.com`, default password сменён на env-обязательный | 2 ч |

**API impact:** новые поля в ответах (не ломает), но `name` → `email` для login = breaking.
**Миграция:** dual-field период, deprecation warning в `/auth/login` с `name`.
**PR:** `feat: phase-2 — data model v2 with multi-role, permissions, audit, sessions`

---

### Phase 3 — Безопасность auth
**Цель:** довести auth до уровня, при котором не стыдно показать.

| # | Задача | Трудозатраты |
|---|---|---|
| 3.1 | **Argon2id** с явными параметрами (memory, iterations, parallelism) | 1 ч |
| 3.2 | **Password policy:** min length, character classes, breach-check через HIBP k-anonymity | 4 ч |
| 3.3 | **Rate limit:** `tower-governor` или Redis-бэкенд, 5 login attempts / 15 min / email + IP | 4-6 ч |
| 3.4 | **Generic error responses** на login/register (no info leak) | 30 мин |
| 3.5 | **Refresh token rotation:** каждый refresh → новый access+refresh, старый invalidated | 4 ч |
| 3.6 | **Reuse detection:** если использованный rotated refresh приходит → revoke entire family + alert | 2 ч |
| 3.7 | **JWT upgrade:** HS256 → RS256, key pair через env, `iss`, `aud`, `jti`, `iat`, `nbf` claims | 4 ч |
| 3.8 | **Email verification flow:** token + email-template (есть Tera/Handlebars/Letter) | 6-8 ч |
| 3.9 | **Password reset flow:** request → email → set new | 4-6 ч |
| 3.10 | **MFA / TOTP** (опционально per-user): `totp-rs` crate, QR через `/auth/mfa/setup` | 6-8 ч |
| 3.11 | **Session management:** list/terminate sessions per user | 3 ч |
| 3.12 | **Audit log hooks:** login, logout, password change, role change, email change | 3 ч |
| 3.13 | **CORS:** configurable via env, sensible defaults | 1 ч |
| 3.14 | **Security headers:** HSTS, X-Content-Type-Options, X-Frame-Options, CSP | 1 ч |

**PR:** `feat: phase-3 — auth security hardening`
**Breaking:** refresh token rotation = старые refresh-токены не работают. Деплой → одновременный logout всех.

---

### Phase 4 — API standards & reusability
**Цель:** сделать так, чтобы любой проект мог интегрироваться за минуты.

| # | Задача | Трудозатраты |
|---|---|---|
| 4.1 | **API versioning:** все маршруты под `/v1/` | 2 ч |
| 4.2 | **OpenAPI 3.1:** `utoipa` для генерации из кода | 6-8 ч |
| 4.3 | **Swagger UI:** `/docs` (auth-protected в prod) | 1 ч |
| 4.4 | **OAuth2 / OIDC:** | 16-24 ч |
| | • `/oauth2/authorize`, `/oauth2/token`, `/oauth2/userinfo`, `/oauth2/jwks.json` | |
| | • `.well-known/openid-configuration` | |
| | • Flows: Authorization Code + PKCE, Client Credentials, Refresh Token | |
| | • ID tokens (OIDC), `id_token_signed_response_alg: RS256` | |
| 4.5 | **OAuth2 client management:** CRUD на OAuth2-приложения (client_id/secret, redirect_uris, scopes) | 6-8 ч |
| 4.6 | **gRPC** (опционально): `tonic` + proto для service-to-service | 8-12 ч |
| 4.7 | **Webhooks:** events (user.created, role.assigned, etc.) + retry + signature | 6-8 ч |
| 4.8 | **Bulk operations:** `POST /users/bulk`, `DELETE /users/bulk` | 2 ч |
| 4.9 | **Filter + sort + search** в list-эндпоинтах | 4 ч |
| 4.10 | **Field selection:** `?fields=id,email,role` | 2 ч |
| 4.11 | **Cursor pagination** как альтернатива offset | 3 ч |
| 4.12 | **Idempotency keys** для POST | 2 ч |
| 4.13 | **Client SDK (генерация):** Rust, TypeScript, Python — через openapi-generator | 3 ч |
| 4.14 | **Postman/Insomnia/HTTPie collection** в репо | 2 ч |

**PR серия:** 4.0 (versioning+OpenAPI), 4.1 (OAuth2 core), 4.2 (OAuth2 clients), 4.3 (gRPC), 4.4 (bulk+filter), 4.5 (SDKs)
**Breaking:** `/auth/login` → `/v1/auth/login`, новые endpoints

---

### Phase 5 — Мульти-тенантность
**Цель:** одна инсталляция обслуживает несколько проектов.

| # | Задача | Трудозатраты |
|---|---|---|
| 5.1 | **Tenants** table + CRUD | 4 ч |
| 5.2 | **Tenant isolation** на уровне repository (middleware + filter) | 6-8 ч |
| 5.3 | `tid` claim в JWT, валидация при каждом запросе | 3 ч |
| 5.4 | **Tenant-scoped roles & permissions** | 4 ч |
| 5.5 | Tenant admin role (в дополнение к global admin) | 2 ч |
| 5.6 | Cross-tenant access (опционально, explicit allow) | 4 ч |
| 5.7 | SAML 2.0 (опционально): `samael` | 12-16 ч |
| 5.8 | LDAP integration (опционально): `ldap3` | 8-12 ч |

**PR:** `feat: phase-5 — multi-tenancy`
**Migration:** текущая single-tenant база оборачивается в `default_tenant`, `tid = uuid_nil()` для совместимости.

---

### Phase 6 — Production-ready operations
**Цель:** развернуть в K8s за один `helm install`.

| # | Задача | Трудозатраты |
|---|---|---|
| 6.1 | **Health endpoints:** `/livez`, `/readyz` (с проверкой БД), `/healthz` | 2 ч |
| 6.2 | **Metrics:** Prometheus `/metrics` (`prometheus` crate) | 3 ч |
| 6.3 | **Distributed tracing:** OpenTelemetry export (`tracing-opentelemetry`) | 4-6 ч |
| 6.4 | **Structured JSON logs** (для Loki/ELK) | 2 ч |
| 6.5 | **Graceful shutdown:** SIGTERM → drain connections, finish in-flight | 2 ч |
| 6.6 | **Migration runner:** `diesel migration run` в entrypoint или sidecar | 1 ч |
| 6.7 | **Multi-arch Docker:** `linux/amd64` + `linux/arm64` через `docker buildx` | 2 ч |
| 6.8 | **Image signing:** cosign | 1 ч |
| 6.9 | **SBOM:** cargo-cyclonedx в CI | 1 ч |
| 6.10 | **Helm chart:** values.yaml, templates/deployment, service, ingress, configmap, secret, hpa, poddisruptionbudget | 8-12 ч |
| 6.11 | **Документация:** mdbook с architecture.md, api.md, ops.md, dev.md, security.md | 6-8 ч |
| 6.12 | **Performance benchmarks:** `criterion` на hot paths (auth/login, jwt decode) | 4 ч |
| 6.13 | **Load test setup:** k6/wrk2 скрипты | 3 ч |
| 6.14 | **Backup/restore runbook** для Postgres | 1 ч |
| 6.15 | **Vulnerability scanning** в CI: Trivy, cargo-audit | 2 ч |

**PR:** серия `ops/phase-6-...`

---

## 🔀 Стратегия совместимости

1. **Phase 0-1:** 100% backwards compatible — только инфраструктура
2. **Phase 2:** `name` → `email` для login = breaking. Решение: dual-period (3 мес), в логи пишем deprecation warning
3. **Phase 3:** refresh-token rotation = форсированный logout всех при деплое
4. **Phase 4-6:** версионирование `/v1/` — старые пути можно держать как алиасы 6 мес

Все breaking changes — отдельный PR + CHANGELOG.md + announcement в release notes.

---

## 🤝 Cross-cutting concerns (везде)

- **Безопасность:** PR-review чеклист (нет `.unwrap()`, нет SQL-injection, нет hardcoded secrets)
- **Тесты:** каждый PR должен иметь тесты, coverage не падает
- **Документация:** каждый публичный API — OpenAPI аннотация
- **Observability:** каждое значимое действие — log event
- **Локализация:** тексты ошибок на английском (стандарт API), но легко извлекаются

---

## 📅 Рекомендованный порядок

```
Phase 0  →  Phase 1  →  Phase 2  →  Phase 3  →  Phase 4  →  Phase 5  →  Phase 6
   ⬇         ⬇          ⬇          ⬇          ⬇          ⬇          ⬇
  1 PR      1 PR        1 PR       1 PR       4 PR       1 PR       5 PR
```

**Итого:** ~14 PR, оценка 200-300 ч разработки. По 4-8 ч/неделю = 6-9 месяцев.

Можно **параллелить** некоторые фазы (например, Phase 1 тесты + Phase 0 документация), но архитектурно они линейны.

---

## ❓ Открытые вопросы (нужен твой ввод)

1. **Лицензия:** MIT или Apache-2.0?
2. **Скоуп проекта:** плагин OAuth2-провайдера (только keycloak-like features) или **полный IDP** с собственным UI админки?
3. **Email-доставка:** встроенный SMTP или через env-снапшот к внешнему (SendGrid, AWS SES)?
4. **Tenants:** обязательно в Phase 5 или опционально?
5. **gRPC:** нужен ли на старте или отложить?
6. **Database migration runner:** `diesel migration` в entrypoint или отдельный job?
7. **OpenAPI codegen:** `utoipa` (compile-time) vs hand-written (полнее контроль)?
8. **Приоритеты фаз:** что важнее для твоих ближайших проектов?
   - Phase 0-1 (quality) — must
   - Phase 2 (data model) — must before Phase 3+
   - Phase 3 (security) — must
   - Phase 4 (OAuth2) — must для переиспользования
   - Phase 5 (tenants) — можно отложить
   - Phase 6 (ops) — must для prod

---

## 🎯 Что я предлагаю начать прямо сейчас

После твоего апрува плана, **Phase 0 + 1 объединю в один PR** (мелкая инфраструктура, легко мерджить). Это ~10-15 ч работы, оформлю как 1 PR с под-PR-ами (squash 3-4 коммита).

**Phase 2 начну только после мержа Phase 0-1** — иначе схема будет мигрировать поверх устаревших deps.

Каждая фаза — отдельный PR. Если что-то пойдёт не так — откатываем один PR.

---

_План составлен Hermes Agent на основе чтения ~1000 строк существующего кода (Rocket 0.5-rc, Diesel 1.4, JWT, RBAC). Готов выслушать правки и приоритеты._
