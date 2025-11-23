//! CQRS pattern example with commands and queries.
//!
//! Run with: cargo run --example cqrs

use avx_events::cqrs::{CommandBus, CommandHandler, QueryHandler};
use avx_events::{Event, EventBus};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// Domain Events
#[derive(Debug, Clone, Serialize, Deserialize)]
struct UserCreated {
    user_id: String,
    email: String,
    name: String,
}

impl Event for UserCreated {
    fn event_type(&self) -> &'static str {
        "user.created"
    }
    fn aggregate_id(&self) -> String {
        self.user_id.clone()
    }
}

// Commands (Write Side)
#[derive(Debug)]
struct CreateUserCommand {
    email: String,
    name: String,
}

struct CreateUserHandler {
    event_bus: EventBus,
}

#[async_trait]
impl CommandHandler<CreateUserCommand> for CreateUserHandler {
    type Result = String; // Returns user_id

    async fn handle(&self, cmd: CreateUserCommand) -> avx_events::Result<String> {
        // Validate
        if cmd.email.is_empty() {
            return Err(avx_events::Error::validation("Email is required"));
        }
        if cmd.name.is_empty() {
            return Err(avx_events::Error::validation("Name is required"));
        }

        // Generate ID
        let user_id = format!("user-{}", uuid::Uuid::new_v4());

        // Publish domain event
        self.event_bus
            .publish(UserCreated {
                user_id: user_id.clone(),
                email: cmd.email,
                name: cmd.name,
            })
            .await?;

        Ok(user_id)
    }
}

// Read Model
#[derive(Debug, Clone, Serialize, Deserialize)]
struct UserView {
    id: String,
    email: String,
    name: String,
}

// Read Model Repository
#[derive(Clone)]
struct UserReadModel {
    users: Arc<RwLock<HashMap<String, UserView>>>,
}

impl UserReadModel {
    fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    async fn insert(&self, user: UserView) {
        let mut users = self.users.write().await;
        users.insert(user.id.clone(), user);
    }

    async fn find_by_id(&self, id: &str) -> Option<UserView> {
        let users = self.users.read().await;
        users.get(id).cloned()
    }

    async fn list_all(&self) -> Vec<UserView> {
        let users = self.users.read().await;
        users.values().cloned().collect()
    }
}

// Queries (Read Side)
#[derive(Debug)]
struct GetUserQuery {
    user_id: String,
}

struct GetUserHandler {
    read_model: UserReadModel,
}

#[async_trait]
impl QueryHandler<GetUserQuery> for GetUserHandler {
    type Result = UserView;

    async fn handle(&self, query: GetUserQuery) -> avx_events::Result<UserView> {
        self.read_model
            .find_by_id(&query.user_id)
            .await
            .ok_or_else(|| avx_events::Error::not_found("User not found"))
    }
}

#[derive(Debug)]
struct ListUsersQuery;

struct ListUsersHandler {
    read_model: UserReadModel,
}

#[async_trait]
impl QueryHandler<ListUsersQuery> for ListUsersHandler {
    type Result = Vec<UserView>;

    async fn handle(&self, _query: ListUsersQuery) -> avx_events::Result<Vec<UserView>> {
        Ok(self.read_model.list_all().await)
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    println!("📋 AVX Events - CQRS Example\n");

    // Setup
    let event_bus = EventBus::new();
    let read_model = UserReadModel::new();

    // Update read model from events
    let mut subscriber = event_bus.subscribe::<UserCreated>().await;
    tokio::spawn({
        let read_model = read_model.clone();
        async move {
            while let Some(envelope) = subscriber.recv().await {
                let user = UserView {
                    id: envelope.event.user_id,
                    email: envelope.event.email,
                    name: envelope.event.name,
                };
                read_model.insert(user).await;
                println!("📝 Read model updated");
            }
        }
    });

    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    println!("--- Executing Commands (Write) ---\n");

    // Command Bus
    let command_bus = CommandBus::new();

    // Handlers
    let create_handler = CreateUserHandler {
        event_bus: event_bus.clone(),
    };
    let get_handler = GetUserHandler {
        read_model: read_model.clone(),
    };
    let list_handler = ListUsersHandler {
        read_model: read_model.clone(),
    };

    // Execute commands
    let user_id_1 = command_bus
        .dispatch(
            CreateUserCommand {
                email: "alice@example.com".into(),
                name: "Alice Silva".into(),
            },
            &create_handler,
        )
        .await
        .unwrap();
    println!("✅ Created user: {}\n", user_id_1);

    let user_id_2 = command_bus
        .dispatch(
            CreateUserCommand {
                email: "bob@example.com".into(),
                name: "Bob Santos".into(),
            },
            &create_handler,
        )
        .await
        .unwrap();
    println!("✅ Created user: {}\n", user_id_2);

    // Wait for read model to update
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    println!("--- Executing Queries (Read) ---\n");

    // Execute queries
    let user = get_handler
        .handle(GetUserQuery {
            user_id: user_id_1.clone(),
        })
        .await
        .unwrap();
    println!("👤 Found user: {} - {}", user.name, user.email);

    let all_users = list_handler.handle(ListUsersQuery).await.unwrap();
    println!("\n📋 All users:");
    for user in all_users {
        println!("  - {} ({}) - {}", user.name, user.id, user.email);
    }

    println!("\n✅ CQRS example completed!");
}
