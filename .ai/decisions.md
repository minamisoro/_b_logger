# Architecture Decision Records (ADR)

## Overview

This document records key architectural decisions made during the development of _b_logger. Each entry follows the [MADR template](https://adr.github.io/madr/) and includes context, decision, consequences, and status.

## ADR Index

- [ADR‑001: Protobuf as Single Source of Truth](./adrs/001_adr_protobuf-as-single-source-of-truth.md)
- [ADR‑002: Shared Rust Core Library](./adrs/002_adr_shared-rust-core-library.md)
- [ADR‑003: Monorepo with Workspaces](./adrs/003_adr_monorepo-with-workspaces.md)
- [ADR‑004: Axum and Tonic for Backend](./adrs/004_adr_axum-and-tonic-for-backend.md)
- [ADR‑005: REST Gateway for Web Clients](./adrs/005_adr_rest-gateway-for-web-clients.md)
- [ADR‑006: PostgreSQL with JSONB for Context Storage](./adrs/006_adr_postgresql-with-jsonb-for-context-storage.md)
- [ADR‑007: WebAssembly and FFI for Cross‑Platform Logic](./adrs/007_adr_webassembly-and-ffi-for-cross-platform-logic.md)
- [ADR‑008: REST with Protobuf Payloads](./adrs/008_adr_rest-with-protobuf-payloads.md)

## Pending Decisions

- Authentication provider (OAuth2 vs JWT vs sessions)
- Real‑time transport (WebSocket vs Server‑Sent Events)
- File storage strategy (S3 vs local vs IPFS)
- Monitoring and observability stack