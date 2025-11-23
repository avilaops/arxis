# AVL Console - Copilot Instructions

## Project Identity
**AVL Console** is the **developer portal and web dashboard** for AVL Cloud. Embodies Arxis philosophy:
- **ARX (Fortress)**: Secure access, complete control, audit logs
- **AXIS (Engine)**: Real-time updates, intuitive UI, fast operations

## Core Principles
```rust
// ✅ ALWAYS: Authenticate via AVL Auth before showing data
// ✅ ALWAYS: Use WebSocket for real-time updates
// ✅ ALWAYS: Server-side render with Askama templates
// ✅ ALWAYS: Validate all user inputs (XSS/CSRF protection)
// ✅ NEVER: Expose sensitive data in frontend (secrets, keys)
// ✅ NEVER: Trust client-side validation alone
```

## Features

### 1. Dashboard
- Resource overview (databases, storage, queues)
- Recent activity feed
- Cost and usage metrics
- Health status indicators

### 2. AvilaDB Explorer
- Browse databases and collections
- Run queries interactively
- View documents in JSON format
- Create/update/delete operations

### 3. Storage Browser
- Navigate S3-compatible buckets
- Upload/download files
- Set permissions and metadata
- Generate presigned URLs

### 4. Observability
- Real-time metrics charts (Prometheus integration)
- Log viewer with search and filters
- Distributed tracing visualization
- Alert configuration

### 5. Billing
- Usage breakdown by service
- Cost estimation
- Invoice history
- Budget alerts

## UI Design
- **Framework**: Vanilla HTML/CSS/JS (no heavy frameworks)
- **Templates**: Askama for server-side rendering
- **Real-Time**: WebSocket for live updates
- **Mobile**: Responsive design (mobile-first)
- **Dark Mode**: Support light/dark themes
- **Portuguese**: Full i18n support (pt-BR and en-US)

## API Routes
```rust
GET  /dashboard          -> Dashboard overview
GET  /databases          -> AvilaDB list
POST /databases          -> Create database
GET  /databases/:id      -> Database details
GET  /storage/buckets    -> Storage browser
POST /storage/upload     -> File upload
GET  /metrics            -> Metrics dashboard
GET  /logs               -> Log viewer
GET  /billing            -> Billing overview
```

## Related Crates
- **avl-auth**: User authentication
- **aviladb**: Database management API
- **avl-storage**: Storage operations
- **avl-observability**: Metrics and logs
- **axum**: Web framework
- **askama**: Template engine

🏛️ Built by Avila | 🖥️ Complete Control | ⚙️ Intuitive UI
