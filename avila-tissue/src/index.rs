//! Índice de busca full-text

use avila_error::{Error, ErrorKind, Result};
use avila_id::Id;
use tantivy::{Index, IndexWriter, schema::*};
use std::path::Path;

/// Índice de busca de emails
pub struct EmailIndex {
    index: Index,
    schema: Schema,
}

impl EmailIndex {
    /// Cria novo índice
    pub fn create(path: impl AsRef<Path>) -> Result<Self> {
        let mut schema_builder = Schema::builder();

        schema_builder.add_text_field("id", TEXT | STORED);
        schema_builder.add_text_field("from", TEXT | STORED);
        schema_builder.add_text_field("to", TEXT);
        schema_builder.add_text_field("subject", TEXT | STORED);
        schema_builder.add_text_field("body", TEXT);
        schema_builder.add_date_field("date", INDEXED | STORED);

        let schema = schema_builder.build();

        let index = Index::create_in_dir(path, schema.clone())
            .map_err(|e| Error::new(ErrorKind::Io, format!("Falha ao criar índice: {}", e)))?;

        Ok(Self { index, schema })
    }

    /// Abre índice existente
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let index = Index::open_in_dir(path)
            .map_err(|e| Error::new(ErrorKind::Io, format!("Falha ao abrir índice: {}", e)))?;

        let schema = index.schema();

        Ok(Self { index, schema })
    }

    /// Adiciona email ao índice
    pub fn add_email(&self, email: &avila_cell::message::Email) -> Result<()> {
        let mut index_writer = self.index.writer(50_000_000)
            .map_err(|e| Error::new(ErrorKind::Io, format!("Falha ao criar writer: {}", e)))?;

        let id_field = self.schema.get_field("id").unwrap();
        let from_field = self.schema.get_field("from").unwrap();
        let subject_field = self.schema.get_field("subject").unwrap();
        let body_field = self.schema.get_field("body").unwrap();

        let mut doc = tantivy::TantivyDocument::default();
        doc.add_text(id_field, &email.id);
        doc.add_text(from_field, &email.from.to_string());
        doc.add_text(subject_field, &email.subject);
        doc.add_text(body_field, &email.body);

        index_writer.add_document(doc)
            .map_err(|e| Error::new(ErrorKind::Io, format!("Falha ao adicionar doc: {}", e)))?;

        index_writer.commit()
            .map_err(|e| Error::new(ErrorKind::Io, format!("Falha ao commit: {}", e)))?;

        Ok(())
    }

    /// Busca emails por query
    pub fn search(&self, query_str: &str, limit: usize) -> Result<Vec<String>> {
        // TODO: Implementar busca usando tantivy
        // Por enquanto retorna vazio
        Ok(Vec::new())
    }
}
