# 🖥️ AVL Console

**Developer Portal and Web Dashboard for AVL Cloud Platform**

[![Crates.io](https://img.shields.io/crates/v/avl-console.svg)](https://crates.io/crates/avl-console)
[![Documentation](https://docs.rs/avl-console/badge.svg)](https://docs.rs/avl-console)
[![AVL Cloud](https://img.shields.io/badge/AVL-Cloud%20Platform-00d4ff)](https://avila.cloud)

🏛️ **Complete Control** | ⚙️ **Intuitive UI** | 📊 **Real-Time Monitoring**

---

## Features

- **Dashboard**: Overview of all AVL resources
- **Resource Management**: Create, update, delete resources
- **AvilaDB Explorer**: Browse and query database
- **Storage Browser**: Navigate S3-compatible storage
- **Metrics & Logs**: Real-time observability
- **Billing**: Usage tracking and cost management
- **API Explorer**: Test APIs interactively

## Architecture

- **Frontend**: Modern web UI (HTML/CSS/JS)
- **Backend**: Axum REST API
- **Authentication**: AVL Auth integration
- **Real-Time**: WebSocket for live updates
- **Templates**: Askama for server-side rendering

## Screenshots

```
┌─────────────────────────────────────────┐
│ AVL Console - Dashboard                 │
├─────────────────────────────────────────┤
│ Resources:                              │
│  ✓ AvilaDB: 3 databases (healthy)      │
│  ✓ Storage: 15 buckets (128 GB)        │
│  ✓ Queue: 5 topics (1.2M msgs/day)     │
│                                         │
│ Recent Activity:                        │
│  • user123 created database "prod"     │
│  • api-key-xyz uploaded 15 files       │
│  • Queue "events" processed 50K msgs   │
└─────────────────────────────────────────┘
```

## Access

- **Web UI**: https://console.avila.cloud
- **API**: https://api.avila.cloud/console
- **CLI**: `avl console open`

🏛️ **Built by Avila** - Part of AVL Cloud Platform
