//! # aviladb - Database placeholder
pub struct Database {
    pub name: String,
}

impl Database {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_db() {
        let db = Database::new("test".into());
        assert_eq!(db.name, "test");
    }
}
