-- Create user_group_members table
-- Many-to-many relationship between user groups and users
CREATE TABLE user_group_members (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    group_id UUID NOT NULL REFERENCES user_groups(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(group_id, user_id)
);

-- Create indexes for common queries
CREATE INDEX idx_user_group_members_group_id ON user_group_members(group_id);
CREATE INDEX idx_user_group_members_user_id ON user_group_members(user_id);
