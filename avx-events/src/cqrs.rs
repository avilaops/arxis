//! CQRS (Command Query Responsibility Segregation) patterns.

use crate::Result;
use async_trait::async_trait;
use std::fmt::Debug;

/// Trait for command handlers (write operations).
///
/// Commands represent intentions to change state and should be validated
/// before execution.
///
/// # Example
///
/// ```rust,no_run
/// use avx_events::cqrs::CommandHandler;
/// use async_trait::async_trait;
///
/// pub struct CreateUserCommand {
///     pub email: String,
///     pub name: String,
/// }
///
/// pub struct CreateUserHandler {
///     // dependencies like event bus, repositories, etc.
/// }
///
/// #[async_trait]
/// impl CommandHandler<CreateUserCommand> for CreateUserHandler {
///     type Result = String; // user_id
///
///     async fn handle(&self, cmd: CreateUserCommand) -> avx_events::Result<String> {
///         // Validate
///         if cmd.email.is_empty() {
///             return Err(avx_events::Error::validation("Email required"));
///         }
///
///         // Execute business logic
///         let user_id = uuid::Uuid::new_v4().to_string();
///
///         // Publish domain event
///         // self.event_bus.publish(UserCreated { ... }).await?;
///
///         Ok(user_id)
///     }
/// }
/// ```
#[async_trait]
pub trait CommandHandler<C>: Send + Sync {
    /// The result type returned by this handler.
    type Result: Send;

    /// Handles a command and returns a result.
    async fn handle(&self, command: C) -> Result<Self::Result>;
}

/// Trait for query handlers (read operations).
///
/// Queries retrieve data without modifying state.
///
/// # Example
///
/// ```rust,no_run
/// use avx_events::cqrs::QueryHandler;
/// use async_trait::async_trait;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Serialize, Deserialize)]
/// pub struct GetUserQuery {
///     pub user_id: String,
/// }
///
/// #[derive(Serialize, Deserialize)]
/// pub struct UserView {
///     pub id: String,
///     pub email: String,
///     pub name: String,
/// }
///
/// pub struct GetUserHandler {
///     // read model repository
/// }
///
/// #[async_trait]
/// impl QueryHandler<GetUserQuery> for GetUserHandler {
///     type Result = UserView;
///
///     async fn handle(&self, query: GetUserQuery) -> avx_events::Result<UserView> {
///         // Query read model
///         // let user = self.read_model.find_by_id(&query.user_id).await?;
///         Ok(UserView {
///             id: query.user_id,
///             email: "user@example.com".into(),
///             name: "User".into(),
///         })
///     }
/// }
/// ```
#[async_trait]
pub trait QueryHandler<Q>: Send + Sync {
    /// The result type returned by this handler.
    type Result: Send;

    /// Handles a query and returns a result.
    async fn handle(&self, query: Q) -> Result<Self::Result>;
}

/// Command bus for dispatching commands to handlers.
///
/// Provides a centralized way to execute commands with middleware support
/// (validation, logging, authorization, etc.).
pub struct CommandBus {
    // Middleware removed due to dyn-incompatibility with generic methods
}

impl CommandBus {
    /// Creates a new command bus.
    pub fn new() -> Self {
        Self {}
    }

    /// Dispatches a command to its handler.
    pub async fn dispatch<C, H>(&self, command: C, handler: &H) -> Result<H::Result>
    where
        C: Debug + Send + Sync,
        H: CommandHandler<C>,
    {
        // Execute handler directly (middleware support removed)
        handler.handle(command).await
    }
}impl Default for CommandBus {
    fn default() -> Self {
        Self::new()
    }
}

/// Query bus for dispatching queries to handlers.
pub struct QueryBus {
    // Middleware removed due to dyn-incompatibility with generic methods
}

impl QueryBus {
    /// Creates a new query bus.
    pub fn new() -> Self {
        Self {}
    }

    /// Dispatches a query to its handler.
    pub async fn dispatch<Q, H>(&self, query: Q, handler: &H) -> Result<H::Result>
    where
        Q: Debug + Send + Sync,
        H: QueryHandler<Q>,
    {
        // Execute handler directly (middleware support removed)
        handler.handle(query).await
    }
}impl Default for QueryBus {
    fn default() -> Self {
        Self::new()
    }
}

/// Middleware for commands.
#[async_trait]
pub trait CommandMiddleware: Send + Sync {
    /// Called before command execution.
    async fn before_command<C: Debug + Send + Sync>(&self, command: &C) -> Result<()>;

    /// Called after command execution.
    async fn after_command<R: Send + Sync>(&self, result: &Result<R>) -> Result<()>;
}

/// Middleware for queries.
#[async_trait]
pub trait QueryMiddleware: Send + Sync {
    /// Called before query execution.
    async fn before_query<Q: Debug + Send + Sync>(&self, query: &Q) -> Result<()>;

    /// Called after query execution.
    async fn after_query<R: Send + Sync>(&self, result: &Result<R>) -> Result<()>;
}

/// Logging middleware for commands.
pub struct LoggingCommandMiddleware;

#[async_trait]
impl CommandMiddleware for LoggingCommandMiddleware {
    async fn before_command<C: Debug + Send + Sync>(&self, command: &C) -> Result<()> {
        tracing::info!(command = ?command, "Executing command");
        Ok(())
    }

    async fn after_command<R: Send + Sync>(&self, result: &Result<R>) -> Result<()> {
        match result {
            Ok(_) => tracing::info!("Command executed successfully"),
            Err(e) => tracing::error!(error = %e, "Command execution failed"),
        }
        Ok(())
    }
}

/// Logging middleware for queries.
pub struct LoggingQueryMiddleware;

#[async_trait]
impl QueryMiddleware for LoggingQueryMiddleware {
    async fn before_query<Q: Debug + Send + Sync>(&self, query: &Q) -> Result<()> {
        tracing::debug!(query = ?query, "Executing query");
        Ok(())
    }

    async fn after_query<R: Send + Sync>(&self, result: &Result<R>) -> Result<()> {
        match result {
            Ok(_) => tracing::debug!("Query executed successfully"),
            Err(e) => tracing::error!(error = %e, "Query execution failed"),
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestCommand {
        value: i32,
    }

    struct TestCommandHandler;

    #[async_trait]
    impl CommandHandler<TestCommand> for TestCommandHandler {
        type Result = i32;

        async fn handle(&self, command: TestCommand) -> Result<i32> {
            Ok(command.value * 2)
        }
    }

    #[tokio::test]
    async fn test_command_handler() {
        let handler = TestCommandHandler;
        let command = TestCommand { value: 21 };

        let result = handler.handle(command).await.unwrap();
        assert_eq!(result, 42);
    }

    #[tokio::test]
    async fn test_command_bus() {
        let bus = CommandBus::new();
        let handler = TestCommandHandler;
        let command = TestCommand { value: 10 };

        let result = bus.dispatch(command, &handler).await.unwrap();
        assert_eq!(result, 20);
    }
}
