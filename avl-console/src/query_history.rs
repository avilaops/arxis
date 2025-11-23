//! Query History Indexing and Management

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryHistoryEntry {
    pub id: String,
    pub user_id: String,
    pub query: String,
    pub timestamp: u64,
    pub execution_time_ms: u64,
    pub success: bool,
    pub row_count: Option<usize>,
    pub error_message: Option<String>,
    pub database: String,
    pub collection: Option<String>,
}

impl QueryHistoryEntry {
    pub fn new(
        user_id: String,
        query: String,
        database: String,
    ) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let id = format!("qh_{}_{}", user_id, timestamp);

        Self {
            id,
            user_id,
            query,
            timestamp,
            execution_time_ms: 0,
            success: false,
            row_count: None,
            error_message: None,
            database,
            collection: None,
        }
    }

    pub fn with_result(
        mut self,
        success: bool,
        execution_time_ms: u64,
        row_count: Option<usize>,
        error_message: Option<String>,
    ) -> Self {
        self.success = success;
        self.execution_time_ms = execution_time_ms;
        self.row_count = row_count;
        self.error_message = error_message;
        self
    }
}

pub struct QueryHistory {
    entries: Arc<Mutex<Vec<QueryHistoryEntry>>>,
    user_index: Arc<Mutex<HashMap<String, Vec<String>>>>,
    database_index: Arc<Mutex<HashMap<String, Vec<String>>>>,
    max_entries: usize,
}

impl QueryHistory {
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: Arc::new(Mutex::new(Vec::new())),
            user_index: Arc::new(Mutex::new(HashMap::new())),
            database_index: Arc::new(Mutex::new(HashMap::new())),
            max_entries,
        }
    }

    pub fn add_entry(&self, entry: QueryHistoryEntry) {
        let mut entries = self.entries.lock().unwrap();
        let mut user_index = self.user_index.lock().unwrap();
        let mut db_index = self.database_index.lock().unwrap();

        // Enforce max entries (FIFO)
        if entries.len() >= self.max_entries {
            if let Some(old) = entries.first() {
                // Remove from indexes
                if let Some(user_entries) = user_index.get_mut(&old.user_id) {
                    user_entries.retain(|id| id != &old.id);
                }
                if let Some(db_entries) = db_index.get_mut(&old.database) {
                    db_entries.retain(|id| id != &old.id);
                }
            }
            entries.remove(0);
        }

        // Add to indexes
        user_index
            .entry(entry.user_id.clone())
            .or_insert_with(Vec::new)
            .push(entry.id.clone());

        db_index
            .entry(entry.database.clone())
            .or_insert_with(Vec::new)
            .push(entry.id.clone());

        entries.push(entry);
    }

    pub fn get_by_user(&self, user_id: &str, limit: usize) -> Vec<QueryHistoryEntry> {
        let entries = self.entries.lock().unwrap();
        let user_index = self.user_index.lock().unwrap();

        if let Some(ids) = user_index.get(user_id) {
            ids.iter()
                .rev()
                .take(limit)
                .filter_map(|id| entries.iter().find(|e| &e.id == id).cloned())
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn get_by_database(&self, database: &str, limit: usize) -> Vec<QueryHistoryEntry> {
        let entries = self.entries.lock().unwrap();
        let db_index = self.database_index.lock().unwrap();

        if let Some(ids) = db_index.get(database) {
            ids.iter()
                .rev()
                .take(limit)
                .filter_map(|id| entries.iter().find(|e| &e.id == id).cloned())
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn get_recent(&self, limit: usize) -> Vec<QueryHistoryEntry> {
        let entries = self.entries.lock().unwrap();
        entries.iter().rev().take(limit).cloned().collect()
    }

    pub fn get_failed_queries(&self, limit: usize) -> Vec<QueryHistoryEntry> {
        let entries = self.entries.lock().unwrap();
        entries
            .iter()
            .rev()
            .filter(|e| !e.success)
            .take(limit)
            .cloned()
            .collect()
    }

    pub fn get_slow_queries(&self, threshold_ms: u64, limit: usize) -> Vec<QueryHistoryEntry> {
        let entries = self.entries.lock().unwrap();
        entries
            .iter()
            .rev()
            .filter(|e| e.execution_time_ms > threshold_ms)
            .take(limit)
            .cloned()
            .collect()
    }

    pub fn search(&self, pattern: &str, limit: usize) -> Vec<QueryHistoryEntry> {
        let entries = self.entries.lock().unwrap();
        let pattern_lower = pattern.to_lowercase();
        entries
            .iter()
            .rev()
            .filter(|e| e.query.to_lowercase().contains(&pattern_lower))
            .take(limit)
            .cloned()
            .collect()
    }

    pub fn get_stats(&self) -> QueryHistoryStats {
        let entries = self.entries.lock().unwrap();

        let total = entries.len();
        let successful = entries.iter().filter(|e| e.success).count();
        let failed = total - successful;

        let avg_execution_time = if !entries.is_empty() {
            entries.iter().map(|e| e.execution_time_ms).sum::<u64>() / entries.len() as u64
        } else {
            0
        };

        let mut user_counts: HashMap<String, usize> = HashMap::new();
        for entry in entries.iter() {
            *user_counts.entry(entry.user_id.clone()).or_insert(0) += 1;
        }
        let top_users: Vec<(String, usize)> = {
            let mut counts: Vec<_> = user_counts.into_iter().collect();
            counts.sort_by(|a, b| b.1.cmp(&a.1));
            counts.into_iter().take(5).collect()
        };

        QueryHistoryStats {
            total_queries: total,
            successful_queries: successful,
            failed_queries: failed,
            avg_execution_time_ms: avg_execution_time,
            top_users,
        }
    }

    pub fn clear(&self) {
        self.entries.lock().unwrap().clear();
        self.user_index.lock().unwrap().clear();
        self.database_index.lock().unwrap().clear();
    }
}

impl Default for QueryHistory {
    fn default() -> Self {
        Self::new(10000)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryHistoryStats {
    pub total_queries: usize,
    pub successful_queries: usize,
    pub failed_queries: usize,
    pub avg_execution_time_ms: u64,
    pub top_users: Vec<(String, usize)>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_entry() {
        let history = QueryHistory::new(100);
        let entry = QueryHistoryEntry::new(
            "user1".to_string(),
            "SELECT * FROM users".to_string(),
            "main_db".to_string(),
        );
        history.add_entry(entry);

        let recent = history.get_recent(10);
        assert_eq!(recent.len(), 1);
    }

    #[test]
    fn test_user_index() {
        let history = QueryHistory::new(100);

        for i in 0..5 {
            let entry = QueryHistoryEntry::new(
                "user1".to_string(),
                format!("SELECT {} FROM users", i),
                "main_db".to_string(),
            );
            history.add_entry(entry);
        }

        let user_queries = history.get_by_user("user1", 10);
        assert_eq!(user_queries.len(), 5);
    }

    #[test]
    fn test_database_index() {
        let history = QueryHistory::new(100);

        let entry1 = QueryHistoryEntry::new(
            "user1".to_string(),
            "SELECT * FROM users".to_string(),
            "db1".to_string(),
        );
        let entry2 = QueryHistoryEntry::new(
            "user2".to_string(),
            "SELECT * FROM posts".to_string(),
            "db1".to_string(),
        );

        history.add_entry(entry1);
        history.add_entry(entry2);

        let db_queries = history.get_by_database("db1", 10);
        assert_eq!(db_queries.len(), 2);
    }

    #[test]
    fn test_failed_queries() {
        let history = QueryHistory::new(100);

        let entry = QueryHistoryEntry::new(
            "user1".to_string(),
            "INVALID SQL".to_string(),
            "db1".to_string(),
        ).with_result(false, 100, None, Some("Syntax error".to_string()));

        history.add_entry(entry);

        let failed = history.get_failed_queries(10);
        assert_eq!(failed.len(), 1);
        assert!(!failed[0].success);
    }

    #[test]
    fn test_slow_queries() {
        let history = QueryHistory::new(100);

        let slow = QueryHistoryEntry::new(
            "user1".to_string(),
            "SELECT * FROM huge_table".to_string(),
            "db1".to_string(),
        ).with_result(true, 5000, Some(1000), None);

        history.add_entry(slow);

        let slow_queries = history.get_slow_queries(1000, 10);
        assert_eq!(slow_queries.len(), 1);
        assert!(slow_queries[0].execution_time_ms > 1000);
    }

    #[test]
    fn test_search() {
        let history = QueryHistory::new(100);

        let entry = QueryHistoryEntry::new(
            "user1".to_string(),
            "SELECT name FROM users WHERE active = true".to_string(),
            "db1".to_string(),
        );
        history.add_entry(entry);

        let results = history.search("users", 10);
        assert_eq!(results.len(), 1);

        let no_results = history.search("nonexistent", 10);
        assert_eq!(no_results.len(), 0);
    }

    #[test]
    fn test_max_entries() {
        let history = QueryHistory::new(5);

        for i in 0..10 {
            let entry = QueryHistoryEntry::new(
                format!("user{}", i),
                format!("SELECT {}", i),
                "db1".to_string(),
            );
            history.add_entry(entry);
        }

        let recent = history.get_recent(100);
        assert_eq!(recent.len(), 5);
    }

    #[test]
    fn test_stats() {
        let history = QueryHistory::new(100);

        for i in 0..10 {
            let success = i % 2 == 0;
            let entry = QueryHistoryEntry::new(
                format!("user{}", i % 3),
                format!("SELECT {}", i),
                "db1".to_string(),
            ).with_result(success, 100, Some(10), None);
            history.add_entry(entry);
        }

        let stats = history.get_stats();
        assert_eq!(stats.total_queries, 10);
        assert_eq!(stats.successful_queries, 5);
        assert_eq!(stats.failed_queries, 5);
    }
}
