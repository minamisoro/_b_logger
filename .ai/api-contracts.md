# API Contracts

## Overview

All API contracts are defined using Protocol Buffers (`.proto` files) as the single source of truth. The definitions are used to generate:

- **Rust server types** (via `prost`)
- **TypeScript client types** (via `protobuf‑ts` or similar)
- **Dart client types** (via `protobuf` package)
- **REST endpoints** that accept `application/protobuf` (and optionally `application/json`) payloads.

## Protobuf Service Definitions

### 1. Authentication Service (`auth.proto`)
Handles user registration, login, OAuth2, and session management.

```protobuf
service AuthenticationService {
    rpc Register(RegistrationRequest) returns (RegistrationResponse);
    rpc Login(LoginRequest) returns (LoginResponse);
    rpc GetOAuth2AuthUrl(GetOAuth2AuthUrlRequest) returns (GetOAuth2AuthUrlResponse);
    rpc RefreshToken(RefreshTokenRequest) returns (RefreshTokenResponse);
    rpc Logout(LogoutRequest) returns (LogoutResponse);
    rpc ValidateToken(ValidateTokenRequest) returns (ValidateTokenResponse);
}
```

### 2. Node Service (`node.proto`)
Core operations for creating, reading, updating, and deleting nodes.

```protobuf
service NodeService {
    rpc CreateNode(CreateNodeRequest) returns (CreateNodeResponse);
    rpc GetNode(GetNodeRequest) returns (GetNodeResponse);
    rpc UpdateNode(UpdateNodeRequest) returns (UpdateNodeResponse);
    rpc DeleteNode(DeleteNodeRequest) returns (DeleteNodeResponse);
    rpc ListNodes(ListNodesRequest) returns (ListNodesResponse);
    rpc AttachContext(AttachContextRequest) returns (AttachContextResponse);
    rpc DetachContext(DetachContextRequest) returns (DetachContextResponse);
}
```

### 3. Context Service (`context.proto`)
Management of user‑defined context types and instances.

```protobuf
service ContextService {
    rpc DefineContextType(DefineContextTypeRequest) returns (DefineContextTypeResponse);
    rpc GetContextType(GetContextTypeRequest) returns (GetContextTypeResponse);
    rpc ListContextTypes(ListContextTypesRequest) returns (ListContextTypesResponse);
    rpc CreateContextInstance(CreateContextInstanceRequest) returns (CreateContextInstanceResponse);
    rpc UpdateContextInstance(UpdateContextInstanceRequest) returns (UpdateContextInstanceResponse);
    rpc DeleteContextInstance(DeleteContextInstanceRequest) returns (DeleteContextInstanceResponse);
}
```

### 4. Query Service (`query.proto`)
User‑defined queries for navigating the node network.

```protobuf
service QueryService {
    rpc DefineQuery(DefineQueryRequest) returns (DefineQueryResponse);
    rpc ExecuteQuery(ExecuteQueryRequest) returns (ExecuteQueryResponse);
    rpc ListQueries(ListQueriesRequest) returns (ListQueriesResponse);
    rpc DeleteQuery(DeleteQueryRequest) returns (DeleteQueryResponse);
}
```

### 5. Feed Service (`feed.proto`)
Real‑time feeds and notifications.

```protobuf
service FeedService {
    rpc GetPersonalFeed(GetPersonalFeedRequest) returns (stream FeedItem);
    rpc GetNodeFeed(GetNodeFeedRequest) returns (stream FeedItem);
    rpc SubscribeToNode(SubscribeToNodeRequest) returns (stream NodeUpdate);
}
```

## Core Data Structures

### Node
```protobuf
message Node {
    string id = 1;                      // UUID
    string author_id = 2;               // User ID
    repeated ContextInstance contexts = 3;
    int64 created_at = 4;
    int64 updated_at = 5;
    Visibility visibility = 6;          // PUBLIC, PRIVATE, FOLLOWERS
}
```

### ContextInstance
```protobuf
message ContextInstance {
    string context_type_id = 1;
    map<string, google.protobuf.Value> data = 2;
}
```

### ContextType
```protobuf
message ContextType {
    string id = 1;
    string name = 2;
    string schema = 3;                  // JSON Schema defining allowed fields
    string creator_id = 4;
    int64 created_at = 5;
}
```

### User
```protobuf
message User {
    string id = 1;
    string handle = 2;
    string display_name = 3;
    string email = 4;
    string avatar_url = 5;
    UserPreferences preferences = 6;
}
```

## REST Endpoint Mapping

Each Protobuf service method maps to a RESTful endpoint. The mapping follows the [Google API HTTP pattern](https://cloud.google.com/apis/design/standard_methods). Clients can send either binary Protobuf (`application/protobuf`) or JSON (`application/json`) payloads, with the response format matching the request's `Accept` header.

Example: `NodeService.CreateNode`

- **Protobuf definition**: `rpc CreateNode(CreateNodeRequest) returns (CreateNodeResponse)`
- **REST**: `POST /v1/nodes` with request body as `CreateNodeRequest` serialized as Protobuf or JSON
- **Response**: `CreateNodeResponse` serialized accordingly

The server automatically handles:

- Content‑Type negotiation (`application/json`, `application/protobuf`)
- Error translation (HTTP status codes with error details in Protobuf/JSON)
- Cross‑Origin Resource Sharing (CORS) for web clients

## Versioning

API versioning is path‑based (`/v1/`). Breaking changes require a new version (`/v2/`). Non‑breaking additions (new fields, new endpoints) are backward compatible.

## Client SDKs

Generated SDKs are published as:

- **TypeScript/JavaScript**: `@blogger/client‑sdk` (npm)
- **Dart**: `blogger_client` (pub.dev)
- **Rust**: `blogger‑client` (crates.io)

Each SDK includes:

- Type‑safe client methods for all services
- Authentication helpers (token storage, refresh)
- Reactive data‑binding utilities (where applicable)

## Code Generation Pipeline

1. Protobuf definitions are validated with `buf lint`.
2. `buf generate` produces language‑specific stubs.
3. Custom templates add convenience wrappers and documentation.
4. Generated code is placed in `/packages` for each language.
5. SDKs are published automatically on CI after a tag push.

## Next Steps

- Define detailed request/response messages for each service.
- Write `.proto` files in `/proto` directory.
- Set up `buf.gen.yaml` with required plugins.
- Implement the REST gateway configuration.