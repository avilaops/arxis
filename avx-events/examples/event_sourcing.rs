//! Event sourcing example with EventStore.
//!
//! Run with: cargo run --example event_sourcing

use avx_events::{Event, EventStore};
use serde::{Deserialize, Serialize};

// Domain Events
#[derive(Debug, Clone, Serialize, Deserialize)]
struct AccountCreated {
    account_id: String,
    owner: String,
    initial_balance: f64,
}

impl Event for AccountCreated {
    fn event_type(&self) -> &'static str {
        "account.created"
    }
    fn aggregate_id(&self) -> String {
        self.account_id.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MoneyDeposited {
    account_id: String,
    amount: f64,
}

impl Event for MoneyDeposited {
    fn event_type(&self) -> &'static str {
        "money.deposited"
    }
    fn aggregate_id(&self) -> String {
        self.account_id.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MoneyWithdrawn {
    account_id: String,
    amount: f64,
}

impl Event for MoneyWithdrawn {
    fn event_type(&self) -> &'static str {
        "money.withdrawn"
    }
    fn aggregate_id(&self) -> String {
        self.account_id.clone()
    }
}

// Aggregate
#[derive(Debug, Default)]
struct BankAccount {
    id: String,
    owner: String,
    balance: f64,
    version: u64,
}

// Note: In a real implementation, you'd use an enum for all event types
impl BankAccount {
    fn apply_created(&mut self, event: AccountCreated) {
        self.id = event.account_id;
        self.owner = event.owner;
        self.balance = event.initial_balance;
        self.version += 1;
    }

    fn apply_deposited(&mut self, event: MoneyDeposited) {
        self.balance += event.amount;
        self.version += 1;
    }

    fn apply_withdrawn(&mut self, event: MoneyWithdrawn) {
        self.balance -= event.amount;
        self.version += 1;
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    println!("🏦 AVX Events - Event Sourcing Example\n");

    let store = EventStore::new();
    let account_id = "account-001";

    println!("--- Creating Account ---");

    // Initial events
    store
        .append(
            account_id,
            vec![AccountCreated {
                account_id: account_id.into(),
                owner: "Alice Silva".into(),
                initial_balance: 1000.0,
            }],
        )
        .await
        .unwrap();

    println!("✅ Account created with R$ 1000.00\n");

    // Deposit
    store
        .append(
            account_id,
            vec![MoneyDeposited {
                account_id: account_id.into(),
                amount: 500.0,
            }],
        )
        .await
        .unwrap();

    // Withdrawal
    store
        .append(
            account_id,
            vec![MoneyWithdrawn {
                account_id: account_id.into(),
                amount: 200.0,
            }],
        )
        .await
        .unwrap();

    // Another deposit
    store
        .append(
            account_id,
            vec![MoneyDeposited {
                account_id: account_id.into(),
                amount: 150.0,
            }],
        )
        .await
        .unwrap();

    println!("✅ Transactions recorded\n");

    println!("--- Rebuilding State from Events ---\n");

    // Rebuild account state by replaying events
    let mut account = BankAccount::default();

    // Get and replay AccountCreated events
    let created_events = store
        .get_events::<AccountCreated>(account_id, 0)
        .await
        .unwrap();
    for event in created_events {
        println!("📝 Replaying: Account created for {}", event.owner);
        account.apply_created(event);
    }

    // Get and replay MoneyDeposited events
    let deposit_events = store
        .get_events::<MoneyDeposited>(account_id, 0)
        .await
        .unwrap();
    for event in deposit_events {
        println!("📝 Replaying: Deposited R$ {:.2}", event.amount);
        account.apply_deposited(event);
    }

    // Get and replay MoneyWithdrawn events
    let withdrawal_events = store
        .get_events::<MoneyWithdrawn>(account_id, 0)
        .await
        .unwrap();
    for event in withdrawal_events {
        println!("📝 Replaying: Withdrawn R$ {:.2}", event.amount);
        account.apply_withdrawn(event);
    }

    println!("\n--- Final State ---");
    println!("Account ID: {}", account.id);
    println!("Owner: {}", account.owner);
    println!("Balance: R$ {:.2}", account.balance);
    println!("Version: {}", account.version);

    println!("\n✅ Event sourcing example completed!");
}
