# rust-auth-ms

Reusable auth microservice. JWT (access + refresh), Argon2id passwords, RBAC
via roles + permissions, pluggable email (SMTP / file), OpenAPI/Scalar docs,
and an admin API — all behind a stable HTTP surface you can drop into any
project.

> **Status:** Phase 0 — HTTP layer rewrite from Rocket → Axum. The
> workspace, build, and minimal server are in place; the full port of the
> original 12 endpoints lands on this branch in the next commits.

---

## Workspace layout

```
.
├── crates/
│   ├── api/      # HTTP layer (axum 0.7 + tower 0.4 + tokio)
│   ├── core/     # Business logic: models, repositories, services
│   └── email/    # Pluggable email sender (SMTP, file-based, future SES/SendGrid)
├── migrations/   # Diesel migrations (shared across the workspace)
├── IMPROVEMENT_PLAN.md   # 6-phase roadmap + Future phases
├── LICENSE
└── README.md
```

`api` depends on `core` + `email`. `core` is pure business logic with no
HTTP or I/O frameworks — it's reusable in any future surface (CLI, gRPC).

## Quickstart (dev)

```bash
# 1. Install Rust (1.75+)
# 2. Clone & build
git clone https://github.com/Arteminariy/Rust-auth-ms.git
cd Rust-auth-ms
cargo build

# 3. Run
cp .env.example .env  # then edit DB_* values
cargo run -p auth-ms-api
# → listening on 0.0.0.0:8000
# → GET /        health
# → GET /livez   liveness probe
```

## Environment

See [`.env.example`](.env.example) for the full schema. Quick reference:

| Var                 | Default            | Notes                                  |
| ------------------- | ------------------ | -------------------------------------- |
| `BIND_ADDR`         | `0.0.0.0:8000`     | axum listen address                    |
| `DATABASE_URL`      | _required_         | Postgres connection string             |
| `JWT_SECRET`        | _required_         | HS256 secret (Phase 3 → RS256)         |
| `JWT_ACCESS_TTL`    | `900`              | seconds (15 min)                       |
| `JWT_REFRESH_TTL`   | `2592000`          | seconds (30 days)                      |
| `RUN_MIGRATIONS`    | `true`             | Set `false` for K8s sidecar pattern    |
| `RUST_LOG`          | `info`             | tracing-subscriber filter              |

## Roadmap

See [IMPROVEMENT_PLAN.md](IMPROVEMENT_PLAN.md) for the 6-phase plan:

| Phase | Goal                                      | Hours     |
| ----- | ----------------------------------------- | --------- |
| **0** | HTTP rewrite (Rocket → Axum)              | 15-25     |
| 1     | Quality infra (tests, CI, Docker, logs)   | 25-35     |
| 2     | Data model v2 (email, audit, refresh DB)  | 25-30     |
| 3     | Auth security (Argon2id, rate-limit, JWT) | 35-50     |
| 4     | API standards (v1/, OAuth2/OIDC, Scalar)  | 50-70     |
| 5     | Production ops (Helm, observability)      | 35-45     |

Phases 7+ (gRPC, admin UI, multi-tenancy, MFA, …) are tracked in
**Future** at the bottom of the plan.

## License

MIT — see [LICENSE](LICENSE).
