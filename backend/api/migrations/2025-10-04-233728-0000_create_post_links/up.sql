-- Create post_links table
-- Allows posts to reference other posts (e.g., related articles, series, references)
CREATE TABLE post_links (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    source_post_id UUID NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    target_post_id UUID NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    CONSTRAINT chk_no_self_link CHECK (source_post_id != target_post_id),
    UNIQUE(source_post_id, target_post_id)
);

-- Create indexes for post link queries
CREATE INDEX idx_post_links_source_post_id ON post_links(source_post_id);
CREATE INDEX idx_post_links_target_post_id ON post_links(target_post_id);
