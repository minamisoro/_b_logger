-- Create tags table
-- Hierarchical tag system with parent-child relationships forming a tree structure
CREATE TABLE tags (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    name VARCHAR(100) NOT NULL,
    slug VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    parent_id UUID REFERENCES tags(id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    CONSTRAINT chk_no_self_reference CHECK (id != parent_id)
);

-- Create indexes for hierarchical queries
CREATE INDEX idx_tags_parent_id ON tags(parent_id);
CREATE INDEX idx_tags_slug ON tags(slug);
CREATE INDEX idx_tags_name ON tags(name);
