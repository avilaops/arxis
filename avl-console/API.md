# AVL Console API Reference

## Base URL

```
http://localhost:8080
```

## Authentication

All API endpoints (except `/api/health`, `/api/version`, and `/api/auth/login`) require authentication via session cookie.

### Login

```http
POST /api/auth/login
Content-Type: application/json

{
  "username": "admin",
  "password": "admin"
}
```

**Response:**

```json
{
  "session_id": "sess_abc123",
  "user": {
    "id": "user_001",
    "username": "admin",
    "email": "admin@avila.cloud",
    "role": "admin"
  }
}
```

**Headers:**
- `Set-Cookie: avl_session=sess_abc123; HttpOnly; Secure; SameSite=Strict; Path=/; Max-Age=86400`

### Logout

```http
POST /api/auth/logout
Cookie: avl_session=sess_abc123
```

**Response:** 200 OK

### Get Current User

```http
GET /api/auth/me
Cookie: avl_session=sess_abc123
```

**Response:**

```json
{
  "id": "user_001",
  "username": "admin",
  "email": "admin@avila.cloud",
  "role": "admin"
}
```

## Health & Version

### Health Check

```http
GET /api/health
```

**Response:**

```json
{
  "status": "healthy",
  "version": "0.1.0",
  "services": {
    "aviladb": true,
    "storage": true,
    "observability": true
  }
}
```

### Version Info

```http
GET /api/version
```

**Response:**

```json
{
  "version": "0.1.0",
  "build_date": "2024-11-23",
  "git_commit": "dev"
}
```

## Dashboard

### Get Metrics

```http
GET /dashboard/metrics
Cookie: avl_session=sess_abc123
```

**Response:**

```json
{
  "databases": 3,
  "storage_buckets": 15,
  "storage_size_gb": 128.0,
  "active_connections": 42,
  "requests_per_minute": 1250,
  "uptime_seconds": 3600
}
```

## AvilaDB Explorer

### List Databases

```http
GET /databases/list
Cookie: avl_session=sess_abc123
```

**Response:**

```json
[
  {
    "id": "db_prod_001",
    "name": "production",
    "region": "sa-east-1",
    "collections": 15,
    "size_bytes": 2500000000,
    "created_at": "2024-11-01T10:00:00Z"
  }
]
```

### Get Database Details

```http
GET /databases/{db_id}
Cookie: avl_session=sess_abc123
```

**Response:**

```json
{
  "id": "db_prod_001",
  "name": "production",
  "region": "sa-east-1",
  "collections": 15,
  "size_bytes": 2500000000,
  "created_at": "2024-11-01T10:00:00Z"
}
```

### Execute Query

```http
POST /databases/{db_id}/query
Cookie: avl_session=sess_abc123
Content-Type: application/json

{
  "query": "SELECT * FROM users WHERE active = true"
}
```

**Response:**

```json
{
  "rows": [
    {"id": 1, "name": "User 1", "active": true},
    {"id": 2, "name": "User 2", "active": true}
  ],
  "count": 2,
  "execution_time_ms": 15
}
```

### List Collections

```http
GET /databases/{db_id}/collections
Cookie: avl_session=sess_abc123
```

**Response:**

```json
[
  {
    "name": "users",
    "partition_key": "userId",
    "document_count": 10000,
    "size_bytes": 50000000
  }
]
```

## Storage Browser

### List Buckets

```http
GET /storage/buckets
Cookie: avl_session=sess_abc123
```

**Response:**

```json
[
  {
    "name": "user-uploads",
    "region": "sa-east-1",
    "files": 1250,
    "size_bytes": 5000000000,
    "created_at": "2024-10-01T10:00:00Z"
  }
]
```

### List Files in Bucket

```http
GET /storage/buckets/{bucket}/files
Cookie: avl_session=sess_abc123
```

**Response:**

```json
[
  {
    "name": "image1.jpg",
    "size_bytes": 2500000,
    "content_type": "image/jpeg",
    "last_modified": "2024-11-20T15:30:00Z"
  }
]
```

### Upload File

```http
POST /storage/buckets/{bucket}/upload
Cookie: avl_session=sess_abc123
Content-Type: application/json

{
  "filename": "document.pdf",
  "content_type": "application/pdf",
  "data": "base64_encoded_data_here"
}
```

**Response:**

```json
{
  "success": true,
  "filename": "document.pdf",
  "bucket": "user-uploads"
}
```

## Observability

### Get Metrics

```http
GET /observability/metrics
Cookie: avl_session=sess_abc123
```

**Response:**

```json
{
  "cpu_usage": [
    {"timestamp": 1700000000, "value": 45.2},
    {"timestamp": 1700000060, "value": 48.1}
  ],
  "memory_usage": [
    {"timestamp": 1700000000, "value": 62.5},
    {"timestamp": 1700000060, "value": 64.1}
  ],
  "request_rate": [
    {"timestamp": 1700000000, "value": 1250.0},
    {"timestamp": 1700000060, "value": 1320.0}
  ],
  "error_rate": [
    {"timestamp": 1700000000, "value": 0.8},
    {"timestamp": 1700000060, "value": 1.2}
  ]
}
```

### Get Logs

```http
GET /observability/logs
Cookie: avl_session=sess_abc123
```

**Response:**

```json
[
  {
    "timestamp": "2024-11-23T15:30:01Z",
    "level": "INFO",
    "message": "Request processed successfully",
    "service": "aviladb"
  },
  {
    "timestamp": "2024-11-23T15:30:05Z",
    "level": "WARN",
    "message": "High memory usage detected",
    "service": "storage"
  }
]
```

## Billing

### Get Usage

```http
GET /billing/usage
Cookie: avl_session=sess_abc123
```

**Response:**

```json
{
  "current_month": {
    "period": "Novembro 2024",
    "total_cost_brl": 125.50,
    "estimated_cost_brl": 180.00
  },
  "breakdown": [
    {
      "service": "AvilaDB",
      "usage": "3 databases, 15M operations",
      "cost_brl": 75.00
    },
    {
      "service": "Storage",
      "usage": "128 GB, 50K requests",
      "cost_brl": 35.50
    }
  ]
}
```

### Get Invoices

```http
GET /billing/invoices
Cookie: avl_session=sess_abc123
```

**Response:**

```json
[
  {
    "id": "inv_2024_11",
    "period": "Novembro 2024",
    "amount_brl": 125.50,
    "status": "current",
    "due_date": "2024-12-01"
  },
  {
    "id": "inv_2024_10",
    "period": "Outubro 2024",
    "amount_brl": 98.30,
    "status": "paid",
    "due_date": "2024-11-01"
  }
]
```

## WebSocket

### Connect

```javascript
const ws = new WebSocket('ws://localhost:8080/ws');

ws.onopen = () => {
  console.log('Connected to AVL Console');
};

ws.onmessage = (event) => {
  const message = JSON.parse(event.data);
  console.log('Received:', message);
};
```

### Message Types

**Connected:**
```json
{
  "type": "connected",
  "payload": "Welcome to AVL Console"
}
```

**Subscribe to Topic:**
```json
{
  "type": "subscribe",
  "payload": "metrics"
}
```

**Subscribed Confirmation:**
```json
{
  "type": "subscribed",
  "payload": "metrics"
}
```

**Ping/Pong:**
```json
{
  "type": "ping"
}
```

```json
{
  "type": "pong"
}
```

**Error:**
```json
{
  "type": "error",
  "payload": "Maximum WebSocket connections reached"
}
```

## Error Responses

All errors follow this format:

```json
{
  "error": "error_type",
  "message": "Human readable error message",
  "details": "Optional additional details"
}
```

### Error Types

- `authentication_error` (401): Authentication failed
- `authorization_error` (403): Authorization failed
- `invalid_input` (400): Invalid request data
- `not_found` (404): Resource not found
- `rate_limit_exceeded` (429): Too many requests
- `service_error` (502): External service error
- `internal_error` (500): Internal server error

## Rate Limiting

- **Default**: 100 requests per minute per user
- **Header**: `X-RateLimit-Remaining` shows remaining requests
- **Response**: 429 Too Many Requests when exceeded

## CORS

Configured origins can be set via `AVL_CONSOLE_CORS_ORIGINS` environment variable.

Default: `http://localhost:8080`

---

**AVL Console API** - Complete control over your cloud infrastructure 🚀
