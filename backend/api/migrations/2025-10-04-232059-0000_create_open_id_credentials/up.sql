-- Create open_id_credentials table
-- Decoupled authentication system supporting OpenID (Google) and future providers
CREATE TABLE open_id_credentials (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    provider VARCHAR(50) NOT NULL, -- e.g., 'google', 'github', etc.
    provider_user_id VARCHAR(255) NOT NULL, -- User ID from the OAuth provider
    email VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(provider, provider_user_id)
);

-- Create indexes for lookups
CREATE INDEX idx_open_id_credentials_user_id ON open_id_credentials(user_id);
CREATE INDEX idx_open_id_credentials_provider ON open_id_credentials(provider);
CREATE INDEX idx_open_id_credentials_email ON open_id_credentials(email);
