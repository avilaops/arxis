# AVX Platform - Event-Driven Architecture

## System Overview

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                              AVX Platform Stack                                  │
│                                                                                   │
│  ┌────────────────────────────────────────────────────────────────────────────┐ │
│  │                          Presentation Layer                                 │ │
│  │                                                                              │ │
│  │  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐                 │ │
│  │  │  avx-http    │    │ avx-gateway  │    │  avx-cli     │                 │ │
│  │  │   Server     │    │  API Gateway │    │   CLI Tool   │                 │ │
│  │  └──────┬───────┘    └──────┬───────┘    └──────┬───────┘                 │ │
│  │         │                    │                    │                         │ │
│  └─────────┼────────────────────┼────────────────────┼─────────────────────────┘ │
│            │                    │                    │                           │
│  ┌─────────▼────────────────────▼────────────────────▼─────────────────────────┐ │
│  │                          Application Layer                                   │ │
│  │                                                                               │ │
│  │  ┌──────────────────────────────────────────────────────────────────┐       │ │
│  │  │                      avx-events (Event Bus)                       │       │ │
│  │  │                                                                    │       │ │
│  │  │  ┌───────────┐  ┌───────────┐  ┌───────────┐  ┌───────────┐    │       │ │
│  │  │  │ EventBus  │  │ TopicBus  │  │EventStore │  │CommandBus │    │       │ │
│  │  │  │ (Pub/Sub) │  │ (Topics)  │  │(Sourcing) │  │  (CQRS)   │    │       │ │
│  │  │  └─────┬─────┘  └─────┬─────┘  └─────┬─────┘  └─────┬─────┘    │       │ │
│  │  │        │              │              │              │            │       │ │
│  │  │        └──────────────┴──────────────┴──────────────┘            │       │ │
│  │  │                           │                                       │       │ │
│  │  └───────────────────────────┼───────────────────────────────────────┘       │ │
│  │                              │                                               │ │
│  │  ┌───────────────────────────▼───────────────────────────────────────┐      │ │
│  │  │                      Event Subscribers                             │      │ │
│  │  │                                                                     │      │ │
│  │  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐               │      │ │
│  │  │  │  Analytics  │  │   Logging   │  │   Metrics   │               │      │ │
│  │  │  │  Service    │  │   Service   │  │   Service   │               │      │ │
│  │  │  └─────────────┘  └─────────────┘  └─────────────┘               │      │ │
│  │  │                                                                     │      │ │
│  │  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐               │      │ │
│  │  │  │    Email    │  │Notification │  │    Audit    │               │      │ │
│  │  │  │   Service   │  │   Service   │  │   Service   │               │      │ │
│  │  │  └─────────────┘  └─────────────┘  └─────────────┘               │      │ │
│  │  └───────────────────────────────────────────────────────────────────┘      │ │
│  │                                                                               │ │
│  └───────────────────────────────────────────────────────────────────────────────┘ │
│                                                                                   │
│  ┌───────────────────────────────────────────────────────────────────────────┐  │
│  │                       Cross-Cutting Concerns                               │  │
│  │                                                                             │  │
│  │  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐                │  │
│  │  │avx-telemetry │    │  avx-config  │    │  avx-auth    │                │  │
│  │  │ Observability│    │Configuration │    │Authorization │                │  │
│  │  └──────────────┘    └──────────────┘    └──────────────┘                │  │
│  └───────────────────────────────────────────────────────────────────────────┘  │
│                                                                                   │
│  ┌───────────────────────────────────────────────────────────────────────────┐  │
│  │                          Storage & Infrastructure                          │  │
│  │                                                                             │  │
│  │  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐                │  │
│  │  │   AvilaDB    │    │    Redis     │    │  AVL Queue   │                │  │
│  │  │  NoSQL DB    │    │   In-Memory  │    │ Message Bus  │                │  │
│  │  └──────────────┘    └──────────────┘    └──────────────┘                │  │
│  └───────────────────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────────────────┘
```

## Event Flow

```
┌──────────────┐
│ HTTP Request │
└──────┬───────┘
       │
       ▼
┌──────────────────────────┐
│  avx-http Server         │
│  EventPublisher          │
│  Middleware              │
└──────┬───────────────────┘
       │
       │ publish(HttpRequestEvent)
       ▼
┌──────────────────────────────────────────────────────────────┐
│                      avx-events EventBus                      │
│                                                                │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐       │
│  │   Channel 1  │  │   Channel 2  │  │   Channel 3  │       │
│  │  Analytics   │  │   Logging    │  │   Metrics    │       │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘       │
└─────────┼──────────────────┼──────────────────┼───────────────┘
          │                  │                  │
          ▼                  ▼                  ▼
┌──────────────────┐  ┌──────────────────┐  ┌──────────────────┐
│ Analytics        │  │ Logging          │  │ Metrics          │
│ Subscriber       │  │ Subscriber       │  │ Subscriber       │
│                  │  │                  │  │                  │
│ - Track user     │  │ - Log request    │  │ - Count requests │
│ - A/B testing    │  │ - Structured     │  │ - Latency        │
│ - Behavior       │  │   JSON logs      │  │ - Error rate     │
└──────────────────┘  └──────────────────┘  └──────────────────┘
          │                  │                  │
          ▼                  ▼                  ▼
┌──────────────────┐  ┌──────────────────┐  ┌──────────────────┐
│   AvilaDB        │  │  avx-telemetry   │  │  Prometheus      │
│  (Storage)       │  │   (Logging)      │  │  (Metrics)       │
└──────────────────┘  └──────────────────┘  └──────────────────┘
```

## CQRS Pattern

```
┌──────────────┐
│   Client     │
└──────┬───────┘
       │
       ├─────────────────────────┐
       │                         │
       │ Command                 │ Query
       ▼                         ▼
┌──────────────────┐      ┌──────────────────┐
│  CommandBus      │      │   QueryBus       │
│                  │      │                  │
│ - CreateUser     │      │ - GetUser        │
│ - UpdateUser     │      │ - ListUsers      │
│ - DeleteUser     │      │ - SearchUsers    │
└──────┬───────────┘      └──────┬───────────┘
       │                         │
       │ publish(Event)          │ read
       ▼                         ▼
┌──────────────────────────────────────────┐
│              EventStore                   │
│                                           │
│  ┌────────────────────────────────────┐  │
│  │  Events:                            │  │
│  │  - UserCreated                      │  │
│  │  - UserUpdated                      │  │
│  │  - UserDeleted                      │  │
│  └────────────────────────────────────┘  │
└──────┬───────────────────────────────────┘
       │
       │ replay events
       ▼
┌──────────────────┐
│  Read Model      │
│  (Projection)    │
│                  │
│  - Optimized     │
│  - Denormalized  │
│  - Fast queries  │
└──────────────────┘
```

## Topic Routing

```
Publisher                       TopicBus                    Subscribers
─────────                       ────────                    ───────────

publish("users.created")  ──►  Topic: users.created  ──►  Subscribe("users.created")
                                                      ──►  Subscribe("users.*")
                                                      ──►  Subscribe("**")

publish("orders.paid")    ──►  Topic: orders.paid    ──►  Subscribe("orders.paid")
                                                      ──►  Subscribe("orders.*")
                                                      ──►  Subscribe("**")

publish("users.deleted")  ──►  Topic: users.deleted  ──►  Subscribe("users.deleted")
                                                      ──►  Subscribe("users.*")
                                                      ──►  Subscribe("**")

Wildcard Patterns:
- "users.*"       matches: users.created, users.updated, users.deleted
- "users.**"      matches: users.created, users.profile.updated, users.settings.changed
- "**"            matches: ALL topics
```

## Event Sourcing Flow

```
┌──────────────┐
│   Command    │
│ CreateOrder  │
└──────┬───────┘
       │
       ▼
┌────────────────────────────┐
│  Command Handler           │
│                            │
│  1. Validate business rules│
│  2. Generate events        │
│  3. Apply to aggregate     │
└──────┬─────────────────────┘
       │
       │ emit(OrderCreated)
       ▼
┌────────────────────────────────────────┐
│           Event Store                   │
│                                         │
│  Aggregate: order-123                   │
│  Version 1: OrderCreated                │
│  Version 2: OrderPaid                   │
│  Version 3: OrderShipped                │
└──────┬──────────────────────────────────┘
       │
       │ publish(OrderCreated)
       ▼
┌────────────────────────────────────────┐
│           Event Bus                     │
│                                         │
│  ┌────────────┐    ┌────────────┐      │
│  │ Subscriber │    │ Subscriber │      │
│  │   Email    │    │  Inventory │      │
│  └────────────┘    └────────────┘      │
└────────────────────────────────────────┘
       │                    │
       ▼                    ▼
┌──────────────┐    ┌──────────────┐
│ Send email   │    │ Reserve stock│
└──────────────┘    └──────────────┘
```

## Distributed Architecture (Future)

```
┌─────────────────────────────────────────────────────────────────┐
│                     Multi-Region Deployment                      │
│                                                                   │
│  ┌───────────────────┐         ┌───────────────────┐            │
│  │   Brazil Region   │         │     US Region     │            │
│  │                   │         │                   │            │
│  │ ┌───────────────┐ │         │ ┌───────────────┐ │            │
│  │ │ avx-events    │ │         │ │ avx-events    │ │            │
│  │ │ EventBus      │ │◄───────►│ │ EventBus      │ │            │
│  │ └───────┬───────┘ │         │ └───────┬───────┘ │            │
│  │         │         │         │         │         │            │
│  │         ▼         │         │         ▼         │            │
│  │ ┌───────────────┐ │         │ ┌───────────────┐ │            │
│  │ │  Redis Pub/Sub│ │◄───────►│ │  Redis Pub/Sub│ │            │
│  │ └───────────────┘ │         │ └───────────────┘ │            │
│  │         │         │         │         │         │            │
│  │         ▼         │         │         ▼         │            │
│  │ ┌───────────────┐ │         │ ┌───────────────┐ │            │
│  │ │   AvilaDB     │ │         │ │   AvilaDB     │ │            │
│  │ │ (AVL-BR)      │ │◄───────►│ │ (AVL-US)      │ │            │
│  │ └───────────────┘ │         │ └───────────────┘ │            │
│  └───────────────────┘         └───────────────────┘            │
│                                                                   │
│         ▲                               ▲                        │
│         │                               │                        │
│         │      Global Event Stream      │                        │
│         │                               │                        │
│         └───────────────┬───────────────┘                        │
│                         │                                        │
│                 ┌───────▼────────┐                               │
│                 │  Event Router  │                               │
│                 │  (AVL Queue)   │                               │
│                 └────────────────┘                               │
└─────────────────────────────────────────────────────────────────┘
```

## Technology Stack

| Component     | Technology     | Purpose                       |
| ------------- | -------------- | ----------------------------- |
| Event Bus     | Tokio channels | In-memory pub/sub             |
| Serialization | Serde JSON     | Event serialization           |
| Concurrency   | Tokio async    | Async event handling          |
| Storage       | AvilaDB        | Event sourcing persistence    |
| Distributed   | Redis          | Multi-node event distribution |
| Logging       | avx-telemetry  | Structured JSON logs          |
| Config        | avx-config     | Environment-based config      |
| HTTP          | avx-http       | HTTP server with events       |

## Performance Characteristics

| Operation          | Latency  | Throughput        |
| ------------------ | -------- | ----------------- |
| EventBus publish   | < 1 µs   | 100K+ events/sec  |
| EventBus subscribe | < 1 ms   | N/A               |
| TopicBus routing   | < 10 µs  | 50K+ matches/sec  |
| EventStore append  | < 100 µs | 10K+ events/sec   |
| EventStore replay  | < 1 ms   | 1M+ events/sec    |
| CQRS command       | < 5 ms   | 5K+ commands/sec  |
| HTTP event publish | < 100 µs | 10K+ requests/sec |

## Key Benefits

✅ **Decoupling**: Services communicate via events, not direct calls
✅ **Scalability**: Horizontal scaling with distributed backends
✅ **Resilience**: Failed events go to DLQ for retry
✅ **Observability**: All events logged with correlation IDs
✅ **Flexibility**: Add/remove subscribers without code changes
✅ **Event Sourcing**: Complete audit trail and time travel
✅ **CQRS**: Separate read/write models for optimization

---

**Built for AVX Platform** | Version 0.1.0 | MIT OR Apache-2.0
