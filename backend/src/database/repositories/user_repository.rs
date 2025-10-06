use crate::database::models::User;
use crate::database::connection::DbPool;
use sqlx::{Error as SqlxError, Row};
use tracing::{info, error, debug};

pub struct UserRepository;

impl UserRepository {
    /// Create a new guest user (wallet-only, no password)
    pub async fn create_guest(
        pool: &DbPool,
        username: &str,
        wallet_address: &str,
    ) -> Result<User, SqlxError> {
        info!("[REPOSITORY] Inserting new guest user - username={}, wallet={}", username, wallet_address);

        let result = sqlx::query(
            r#"
            INSERT INTO users (username, wallet_address, created_at, role, email_verified)
            VALUES ($1, $2, NOW(), 'user', false)
            RETURNING id, username, wallet_address, created_at, email, password_hash, role, email_verified, last_login
            "#
        )
        .bind(username)
        .bind(wallet_address)
        .fetch_one(pool)
        .await;

        match result {
            Ok(row) => {
                let user = User {
                    id: row.try_get("id")?,
                    username: row.try_get("username")?,
                    wallet_address: row.try_get("wallet_address")?,
                    created_at: row.try_get("created_at")?,
                    email: row.try_get("email")?,
                    password_hash: row.try_get("password_hash")?,
                    role: row.try_get("role")?,
                    email_verified: row.try_get("email_verified")?,
                    last_login: row.try_get("last_login")?,
                };
                info!("[REPOSITORY] ✅ Guest user inserted - id={}, username={}", user.id, user.username);
                Ok(user)
            }
            Err(e) => {
                error!("[REPOSITORY] ❌ Failed to insert guest user: {:?}", e);
                Err(e)
            }
        }
    }

    /// Create a new user with email and password
    pub async fn create_with_password(
        pool: &DbPool,
        username: &str,
        email: &str,
        password_hash: &str,
    ) -> Result<User, SqlxError> {
        info!("[REPOSITORY] Creating new user with password - username={}, email={}", username, email);

        let result = sqlx::query(
            r#"
            INSERT INTO users (username, wallet_address, email, password_hash, created_at, role, email_verified)
            VALUES ($1, '', $2, $3, NOW(), 'user', false)
            RETURNING id, username, wallet_address, created_at, email, password_hash, role, email_verified, last_login
            "#
        )
        .bind(username)
        .bind(email)
        .bind(password_hash)
        .fetch_one(pool)
        .await;

        match result {
            Ok(row) => {
                let user = User {
                    id: row.try_get("id")?,
                    username: row.try_get("username")?,
                    wallet_address: row.try_get("wallet_address")?,
                    created_at: row.try_get("created_at")?,
                    email: row.try_get("email")?,
                    password_hash: row.try_get("password_hash")?,
                    role: row.try_get("role")?,
                    email_verified: row.try_get("email_verified")?,
                    last_login: row.try_get("last_login")?,
                };
                info!("[REPOSITORY] ✅ User created with password - id={}, username={}", user.id, user.username);
                Ok(user)
            }
            Err(e) => {
                error!("[REPOSITORY] ❌ Failed to create user with password: {:?}", e);
                Err(e)
            }
        }
    }

    /// Find user by email address
    pub async fn find_by_email(
        pool: &DbPool,
        email: &str,
    ) -> Result<Option<User>, SqlxError> {
        debug!("[REPOSITORY] Querying user by email={}", email);

        let row = sqlx::query(
            r#"
            SELECT id, username, wallet_address, created_at, email, password_hash, role, email_verified, last_login
            FROM users
            WHERE email = $1
            "#
        )
        .bind(email)
        .fetch_optional(pool)
        .await?;

        if let Some(row) = row {
            let user = User {
                id: row.try_get("id")?,
                username: row.try_get("username")?,
                wallet_address: row.try_get("wallet_address")?,
                created_at: row.try_get("created_at")?,
                email: row.try_get("email")?,
                password_hash: row.try_get("password_hash")?,
                role: row.try_get("role")?,
                email_verified: row.try_get("email_verified")?,
                last_login: row.try_get("last_login")?,
            };
            debug!("[REPOSITORY] ✅ Found user by email - id={}, username={}", user.id, user.username);
            Ok(Some(user))
        } else {
            debug!("[REPOSITORY] No user found with email={}", email);
            Ok(None)
        }
    }

    /// Find user by wallet address
    pub async fn find_by_wallet_address(
        pool: &DbPool,
        wallet_address: &str,
    ) -> Result<Option<User>, SqlxError> {
        debug!("[REPOSITORY] Querying user by wallet_address={}", wallet_address);

        let row = sqlx::query(
            r#"
            SELECT id, username, wallet_address, created_at, email, password_hash, role, email_verified, last_login
            FROM users
            WHERE wallet_address = $1
            "#
        )
        .bind(wallet_address)
        .fetch_optional(pool)
        .await?;

        if let Some(row) = row {
            let user = User {
                id: row.try_get("id")?,
                username: row.try_get("username")?,
                wallet_address: row.try_get("wallet_address")?,
                created_at: row.try_get("created_at")?,
                email: row.try_get("email")?,
                password_hash: row.try_get("password_hash")?,
                role: row.try_get("role")?,
                email_verified: row.try_get("email_verified")?,
                last_login: row.try_get("last_login")?,
            };
            debug!("[REPOSITORY] ✅ Found user - id={}, username={}", user.id, user.username);
            Ok(Some(user))
        } else {
            debug!("[REPOSITORY] No user found with wallet_address={}", wallet_address);
            Ok(None)
        }
    }

    /// Find user by ID
    pub async fn find_by_id(
        pool: &DbPool,
        user_id: i32,
    ) -> Result<Option<User>, SqlxError> {
        debug!("[REPOSITORY] Querying user by id={}", user_id);

        let row = sqlx::query(
            r#"
            SELECT id, username, wallet_address, created_at, email, password_hash, role, email_verified, last_login
            FROM users
            WHERE id = $1
            "#
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

        if let Some(row) = row {
            let user = User {
                id: row.try_get("id")?,
                username: row.try_get("username")?,
                wallet_address: row.try_get("wallet_address")?,
                created_at: row.try_get("created_at")?,
                email: row.try_get("email")?,
                password_hash: row.try_get("password_hash")?,
                role: row.try_get("role")?,
                email_verified: row.try_get("email_verified")?,
                last_login: row.try_get("last_login")?,
            };
            debug!("[REPOSITORY] ✅ Found user by id - username={}", user.username);
            Ok(Some(user))
        } else {
            debug!("[REPOSITORY] No user found with id={}", user_id);
            Ok(None)
        }
    }

    /// Update username for a user
    pub async fn update_username(
        pool: &DbPool,
        wallet_address: &str,
        new_username: &str,
    ) -> Result<User, SqlxError> {
        info!("[REPOSITORY] Updating username for wallet={}", wallet_address);

        let row = sqlx::query(
            r#"
            UPDATE users
            SET username = $1
            WHERE wallet_address = $2
            RETURNING id, username, wallet_address, created_at, email, password_hash, role, email_verified, last_login
            "#
        )
        .bind(new_username)
        .bind(wallet_address)
        .fetch_one(pool)
        .await?;

        let user = User {
            id: row.try_get("id")?,
            username: row.try_get("username")?,
            wallet_address: row.try_get("wallet_address")?,
            created_at: row.try_get("created_at")?,
            email: row.try_get("email")?,
            password_hash: row.try_get("password_hash")?,
            role: row.try_get("role")?,
            email_verified: row.try_get("email_verified")?,
            last_login: row.try_get("last_login")?,
        };

        info!("[REPOSITORY] ✅ Username updated - id={}, new_username={}", user.id, user.username);
        Ok(user)
    }

    /// Update last login timestamp for a user
    pub async fn update_last_login(
        pool: &DbPool,
        user_id: i32,
    ) -> Result<(), SqlxError> {
        debug!("[REPOSITORY] Updating last_login for user_id={}", user_id);

        sqlx::query(
            r#"
            UPDATE users
            SET last_login = NOW()
            WHERE id = $1
            "#
        )
        .bind(user_id)
        .execute(pool)
        .await?;

        debug!("[REPOSITORY] ✅ Last login updated for user_id={}", user_id);
        Ok(())
    }

    /// Link a wallet address to an existing user account
    pub async fn link_wallet(
        pool: &DbPool,
        user_id: i32,
        wallet_address: &str,
    ) -> Result<User, SqlxError> {
        info!("[REPOSITORY] Linking wallet to user_id={}, wallet={}", user_id, wallet_address);

        // Check if wallet is already linked to another account
        if let Some(existing_user) = Self::find_by_wallet_address(pool, wallet_address).await? {
            if existing_user.id != user_id {
                error!("[REPOSITORY] ❌ Wallet already linked to another account (user_id={})", existing_user.id);
                return Err(SqlxError::RowNotFound); // Return error if wallet is taken
            }
        }

        let row = sqlx::query(
            r#"
            UPDATE users
            SET wallet_address = $1
            WHERE id = $2
            RETURNING id, username, wallet_address, created_at, email, password_hash, role, email_verified, last_login
            "#
        )
        .bind(wallet_address)
        .bind(user_id)
        .fetch_one(pool)
        .await?;

        let user = User {
            id: row.try_get("id")?,
            username: row.try_get("username")?,
            wallet_address: row.try_get("wallet_address")?,
            created_at: row.try_get("created_at")?,
            email: row.try_get("email")?,
            password_hash: row.try_get("password_hash")?,
            role: row.try_get("role")?,
            email_verified: row.try_get("email_verified")?,
            last_login: row.try_get("last_login")?,
        };

        info!("[REPOSITORY] ✅ Wallet linked successfully - user_id={}, wallet={}", user.id, user.wallet_address);
        Ok(user)
    }
}