//! Vector Store Persistence for AvilaDB
//!
//! Persists embeddings and RAG knowledge base to AvilaDB for production use.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::embeddings::{VectorStore, EmbeddedDocument};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistedDocument {
    pub id: String,
    pub content: String,
    pub embedding: Vec<f32>,
    pub metadata: HashMap<String, String>,
    pub collection: String,
    pub created_at: u64,
    pub updated_at: u64,
}

impl PersistedDocument {
    pub fn from_document(doc: EmbeddedDocument, collection: &str) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            id: doc.id,
            content: doc.content,
            embedding: doc.embedding,
            metadata: doc.metadata,
            collection: collection.to_string(),
            created_at: timestamp,
            updated_at: timestamp,
        }
    }

    pub fn to_document(&self) -> EmbeddedDocument {
        EmbeddedDocument {
            id: self.id.clone(),
            content: self.content.clone(),
            embedding: self.embedding.clone(),
            metadata: self.metadata.clone(),
        }
    }
}

pub struct VectorPersistence {
    collection_name: String,
    cache: HashMap<String, PersistedDocument>,
}

impl VectorPersistence {
    pub fn new(collection_name: String) -> Self {
        Self {
            collection_name,
            cache: HashMap::new(),
        }
    }

    /// Save vector store to AvilaDB (simulated for now)
    pub async fn save_vector_store(&mut self, store: &VectorStore) -> Result<usize, String> {
        let mut saved_count = 0;

        for doc in &store.documents {
            let persisted = PersistedDocument::from_document(doc.clone(), &self.collection_name);
            self.cache.insert(persisted.id.clone(), persisted);
            saved_count += 1;
        }

        // TODO: Integrate with AvilaDB client when available
        // Example:
        // let client = AvilaClient::connect(endpoint).await?;
        // let db = client.database("vectors").await?;
        // let collection = db.collection(&self.collection_name).await?;
        //
        // for doc in &store.documents {
        //     let persisted = PersistedDocument::from_document(doc.clone(), &self.collection_name);
        //     collection.insert(persisted).await?;
        //     saved_count += 1;
        // }

        Ok(saved_count)
    }

    /// Load vector store from AvilaDB (simulated for now)
    pub async fn load_vector_store(&self) -> Result<VectorStore, String> {
        let mut store = VectorStore::new();

        for persisted_doc in self.cache.values() {
            store.add_document(persisted_doc.to_document());
        }

        // TODO: Integrate with AvilaDB client
        // Example:
        // let client = AvilaClient::connect(endpoint).await?;
        // let db = client.database("vectors").await?;
        // let collection = db.collection(&self.collection_name).await?;
        //
        // let query = format!("SELECT * FROM {} WHERE collection = @collection", self.collection_name);
        // let results = collection.query(&query)
        //     .param("collection", &self.collection_name)
        //     .execute()
        //     .await?;
        //
        // for result in results {
        //     let persisted: PersistedDocument = serde_json::from_value(result)?;
        //     store.add_document(persisted.to_document());
        // }

        Ok(store)
    }

    /// Incremental update: add new document to existing collection
    pub async fn add_document(&mut self, doc: EmbeddedDocument) -> Result<(), String> {
        let persisted = PersistedDocument::from_document(doc, &self.collection_name);
        self.cache.insert(persisted.id.clone(), persisted.clone());

        // TODO: Insert into AvilaDB
        // collection.insert(persisted).await?;

        Ok(())
    }

    /// Update existing document
    pub async fn update_document(&mut self, doc: EmbeddedDocument) -> Result<(), String> {
        if let Some(existing) = self.cache.get_mut(&doc.id) {
            existing.content = doc.content;
            existing.embedding = doc.embedding;
            existing.metadata = doc.metadata;
            existing.updated_at = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();

            // TODO: Update in AvilaDB
            // collection.update_one(
            //     json!({"id": doc.id}),
            //     json!({"$set": existing})
            // ).await?;

            Ok(())
        } else {
            Err(format!("Document {} not found", doc.id))
        }
    }

    /// Delete document by ID
    pub async fn delete_document(&mut self, id: &str) -> Result<(), String> {
        if self.cache.remove(id).is_some() {
            // TODO: Delete from AvilaDB
            // collection.delete_one(json!({"id": id})).await?;
            Ok(())
        } else {
            Err(format!("Document {} not found", id))
        }
    }

    /// Search by embedding similarity (delegated to VectorStore)
    pub async fn search_similar(&self, query_embedding: &[f32], top_k: usize) -> Result<Vec<EmbeddedDocument>, String> {
        let store = self.load_vector_store().await?;

        // Use VectorStore's internal documents for similarity search
        let mut scored: Vec<(f32, &EmbeddedDocument)> = store.documents
            .iter()
            .map(|doc| {
                let score = crate::embeddings::EmbeddingGenerator::cosine_similarity(
                    &query_embedding.to_vec(),
                    &doc.embedding
                );
                (score, doc)
            })
            .collect();

        scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

        let results = scored
            .into_iter()
            .take(top_k)
            .map(|(_, doc)| doc.clone())
            .collect();

        Ok(results)
    }

    /// Get collection statistics
    pub fn get_stats(&self) -> VectorCollectionStats {
        let total_docs = self.cache.len();
        let total_size_bytes: usize = self.cache.values()
            .map(|doc| doc.content.len() + doc.embedding.len() * 4)
            .sum();

        let avg_embedding_dim = if !self.cache.is_empty() {
            self.cache.values().next().unwrap().embedding.len()
        } else {
            0
        };

        VectorCollectionStats {
            collection_name: self.collection_name.clone(),
            total_documents: total_docs,
            total_size_bytes,
            avg_embedding_dimension: avg_embedding_dim,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorCollectionStats {
    pub collection_name: String,
    pub total_documents: usize,
    pub total_size_bytes: usize,
    pub avg_embedding_dimension: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::embeddings::EmbeddingGenerator;

    #[tokio::test]
    async fn test_save_and_load() {
        let mut persistence = VectorPersistence::new("test_collection".to_string());

        let generator = EmbeddingGenerator::new(128);
        let embedding = generator.embed("test document");

        let doc = EmbeddedDocument {
            id: "doc1".to_string(),
            content: "test document".to_string(),
            embedding,
            metadata: HashMap::new(),
        };

        let mut store = VectorStore::new();
        store.add_document(doc.clone());

        let saved = persistence.save_vector_store(&store).await.unwrap();
        assert_eq!(saved, 1);

        let loaded_store = persistence.load_vector_store().await.unwrap();
        assert_eq!(loaded_store.documents.len(), 1);
    }

    #[tokio::test]
    async fn test_incremental_add() {
        let mut persistence = VectorPersistence::new("test_collection".to_string());

        let generator = EmbeddingGenerator::new(128);
        let embedding = generator.embed("new document");

        let doc = EmbeddedDocument {
            id: "doc2".to_string(),
            content: "new document".to_string(),
            embedding,
            metadata: HashMap::new(),
        };

        persistence.add_document(doc.clone()).await.unwrap();

        let loaded = persistence.load_vector_store().await.unwrap();
        assert_eq!(loaded.documents.len(), 1);
    }

    #[tokio::test]
    async fn test_update_document() {
        let mut persistence = VectorPersistence::new("test_collection".to_string());

        let generator = EmbeddingGenerator::new(128);
        let embedding = generator.embed("original");

        let doc = EmbeddedDocument {
            id: "doc3".to_string(),
            content: "original".to_string(),
            embedding: embedding.clone(),
            metadata: HashMap::new(),
        };

        persistence.add_document(doc.clone()).await.unwrap();

        let updated_doc = EmbeddedDocument {
            id: "doc3".to_string(),
            content: "updated content".to_string(),
            embedding,
            metadata: HashMap::new(),
        };

        persistence.update_document(updated_doc).await.unwrap();

        let loaded = persistence.load_vector_store().await.unwrap();
        assert_eq!(loaded.documents[0].content, "updated content");
    }

    #[tokio::test]
    async fn test_delete_document() {
        let mut persistence = VectorPersistence::new("test_collection".to_string());

        let generator = EmbeddingGenerator::new(128);
        let embedding = generator.embed("to delete");

        let doc = EmbeddedDocument {
            id: "doc4".to_string(),
            content: "to delete".to_string(),
            embedding,
            metadata: HashMap::new(),
        };

        persistence.add_document(doc).await.unwrap();
        persistence.delete_document("doc4").await.unwrap();

        let loaded = persistence.load_vector_store().await.unwrap();
        assert_eq!(loaded.documents.len(), 0);
    }

    #[test]
    fn test_get_stats() {
        let mut persistence = VectorPersistence::new("test_collection".to_string());

        let generator = EmbeddingGenerator::new(128);
        let embedding = generator.embed("stats test");

        let doc = EmbeddedDocument {
            id: "doc5".to_string(),
            content: "stats test".to_string(),
            embedding,
            metadata: HashMap::new(),
        };

        let persisted = PersistedDocument::from_document(doc, "test_collection");
        persistence.cache.insert(persisted.id.clone(), persisted);

        let stats = persistence.get_stats();
        assert_eq!(stats.total_documents, 1);
        assert_eq!(stats.collection_name, "test_collection");
        assert_eq!(stats.avg_embedding_dimension, 128);
    }
}
