## ADR‑006: PostgreSQL with JSONB for Context Storage

**Date**: 2026‑01‑09  
**Status**: Accepted

### Context

Nodes have user‑defined contexts with flexible, schema‑less data. A traditional relational schema would be too rigid, but a document database would sacrifice ACID guarantees and complex joins.

### Decision

Use **PostgreSQL** as the primary database, storing context data in `JSONB` columns. This combines relational integrity (users, nodes, relationships) with document flexibility (contexts).

### Consequences

- **Positive**:
    - ACID transactions and strong consistency.
    - JSONB allows indexing, querying, and partial updates.
    - Single database to manage, reducing operational overhead.
- **Negative**:
    - Schema‑less data may lead to data quality issues without validation.
    - Complex queries mixing relational and JSON paths can be verbose.