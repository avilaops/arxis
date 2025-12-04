-- ============================================================================
-- ARXIS BIM PLATFORM - SCHEMA POSTGRESQL
-- ============================================================================

-- Extensões
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pg_trgm"; -- trigram search

-- ============================================================================
-- USERS & AUTH
-- ============================================================================

CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    name VARCHAR(255),
    role VARCHAR(50) NOT NULL DEFAULT 'user', -- admin, user
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_users_email ON users(email);

-- ============================================================================
-- PROJECTS
-- ============================================================================

CREATE TABLE IF NOT EXISTS projects (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    owner_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_projects_owner ON projects(owner_id);

-- ============================================================================
-- PROJECT ACL (Access Control List)
-- ============================================================================

CREATE TABLE IF NOT EXISTS project_acl (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role VARCHAR(50) NOT NULL, -- viewer, editor, admin
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(project_id, user_id)
);

CREATE INDEX idx_project_acl_project ON project_acl(project_id);
CREATE INDEX idx_project_acl_user ON project_acl(user_id);

-- ============================================================================
-- MODELS
-- ============================================================================

CREATE TABLE IF NOT EXISTS models (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    version INT NOT NULL DEFAULT 1,
    status VARCHAR(50) NOT NULL DEFAULT 'uploading', -- uploading, converting, ready, error
    error_message TEXT,
    ifc_url TEXT,
    ifc_size_bytes BIGINT,
    glb_url TEXT,
    glb_size_bytes BIGINT,
    metadata JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(project_id, name, version)
);

CREATE INDEX idx_models_project ON models(project_id);
CREATE INDEX idx_models_status ON models(status);

-- ============================================================================
-- ELEMENTS (BIM Elements)
-- ============================================================================

CREATE TABLE IF NOT EXISTS elements (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    model_id UUID NOT NULL REFERENCES models(id) ON DELETE CASCADE,
    guid VARCHAR(22) NOT NULL, -- IFC GUID
    type VARCHAR(100) NOT NULL, -- IfcWall, IfcSlab, etc.
    name VARCHAR(255),
    description TEXT,
    properties JSONB, -- {Material: "Concrete", LoadBearing: true, ...}
    hierarchy_path VARCHAR(500), -- /Project/Site/Building/Storey/Wall
    geometry_bounds JSONB, -- {min: [x,y,z], max: [x,y,z]}
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_elements_model ON elements(model_id);
CREATE INDEX idx_elements_guid ON elements(guid);
CREATE INDEX idx_elements_type ON elements(type);
CREATE INDEX idx_elements_hierarchy ON elements USING gin (hierarchy_path gin_trgm_ops);

-- ============================================================================
-- ANNOTATIONS
-- ============================================================================

CREATE TABLE IF NOT EXISTS annotations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    model_id UUID NOT NULL REFERENCES models(id) ON DELETE CASCADE,
    element_guid VARCHAR(22),
    position JSONB NOT NULL, -- {x, y, z}
    camera JSONB NOT NULL, -- {position: [x,y,z], target: [x,y,z], up: [x,y,z]}
    author_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    status VARCHAR(50) DEFAULT 'open', -- open, in_progress, resolved
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_annotations_project ON annotations(project_id);
CREATE INDEX idx_annotations_model ON annotations(model_id);
CREATE INDEX idx_annotations_author ON annotations(author_id);
CREATE INDEX idx_annotations_status ON annotations(status);

-- ============================================================================
-- COMMENTS (on annotations)
-- ============================================================================

CREATE TABLE IF NOT EXISTS comments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    annotation_id UUID NOT NULL REFERENCES annotations(id) ON DELETE CASCADE,
    author_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_comments_annotation ON comments(annotation_id);
CREATE INDEX idx_comments_author ON comments(author_id);

-- ============================================================================
-- FILES (track all uploaded/generated files)
-- ============================================================================

CREATE TABLE IF NOT EXISTS files (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    model_id UUID NOT NULL REFERENCES models(id) ON DELETE CASCADE,
    type VARCHAR(50) NOT NULL, -- ifc, glb, texture, thumbnail
    url TEXT NOT NULL,
    size_bytes BIGINT,
    mime_type VARCHAR(100),
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_files_model ON files(model_id);
CREATE INDEX idx_files_type ON files(type);

-- ============================================================================
-- NOTIFICATIONS
-- ============================================================================

CREATE TABLE IF NOT EXISTS notifications (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    type VARCHAR(50) NOT NULL, -- email, push, in_app
    title VARCHAR(255) NOT NULL,
    message TEXT NOT NULL,
    read BOOLEAN DEFAULT FALSE,
    sent_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_notifications_user ON notifications(user_id);
CREATE INDEX idx_notifications_read ON notifications(read);

-- ============================================================================
-- TRIGGERS (updated_at)
-- ============================================================================

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE PROCEDURE update_updated_at_column();

CREATE TRIGGER update_projects_updated_at BEFORE UPDATE ON projects
    FOR EACH ROW EXECUTE PROCEDURE update_updated_at_column();

CREATE TRIGGER update_models_updated_at BEFORE UPDATE ON models
    FOR EACH ROW EXECUTE PROCEDURE update_updated_at_column();

CREATE TRIGGER update_annotations_updated_at BEFORE UPDATE ON annotations
    FOR EACH ROW EXECUTE PROCEDURE update_updated_at_column();

-- ============================================================================
-- SEED DATA (desenvolvimento)
-- ============================================================================

-- Admin user (senha: admin123)
INSERT INTO users (id, email, password_hash, name, role)
VALUES (
    '00000000-0000-0000-0000-000000000001',
    'admin@arxis.bim',
    '$argon2id$v=19$m=19456,t=2,p=1$AAAAAAAAAAAAAAAAAAAAAA$BBBBBBBBBBBBBBBBBBBBBB', -- TODO: hash real
    'Admin User',
    'admin'
) ON CONFLICT DO NOTHING;

-- Projeto de exemplo
INSERT INTO projects (id, name, description, owner_id)
VALUES (
    '00000000-0000-0000-0000-000000000002',
    'Projeto Demo',
    'Projeto de demonstração da plataforma ARXIS BIM',
    '00000000-0000-0000-0000-000000000001'
) ON CONFLICT DO NOTHING;
