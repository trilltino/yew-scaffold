-- Add authentication fields to users table
-- Email field (unique, nullable to allow existing users)
ALTER TABLE users ADD COLUMN IF NOT EXISTS email VARCHAR(255) UNIQUE;

-- Password hash field (nullable to allow wallet-only users)
ALTER TABLE users ADD COLUMN IF NOT EXISTS password_hash VARCHAR(255);

-- User role field (default 'user')
ALTER TABLE users ADD COLUMN IF NOT EXISTS role VARCHAR(50) DEFAULT 'user';

-- Email verification status
ALTER TABLE users ADD COLUMN IF NOT EXISTS email_verified BOOLEAN DEFAULT FALSE;

-- Last login timestamp
ALTER TABLE users ADD COLUMN IF NOT EXISTS last_login TIMESTAMP WITH TIME ZONE;

-- Create indices for performance
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email) WHERE email IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_users_role ON users(role);

-- Add check constraint for role values
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint
        WHERE conname = 'check_user_role'
        AND conrelid = 'users'::regclass
    ) THEN
        ALTER TABLE users ADD CONSTRAINT check_user_role
            CHECK (role IN ('user', 'admin', 'chapter_lead'));
    END IF;
END $$;

-- Comment on columns for documentation
COMMENT ON COLUMN users.email IS 'User email address for password-based authentication';
COMMENT ON COLUMN users.password_hash IS 'Bcrypt password hash (nullable for wallet-only users)';
COMMENT ON COLUMN users.role IS 'User role: user, admin, or chapter_lead';
COMMENT ON COLUMN users.email_verified IS 'Whether user has verified their email address';
COMMENT ON COLUMN users.last_login IS 'Timestamp of user last login';
