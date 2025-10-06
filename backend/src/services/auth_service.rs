use crate::database::connection::DbPool;
use crate::database::repositories::user_repository::UserRepository;
use crate::database::models::User;
use crate::error::Result;
use shared::dto::auth::Guest;
use shared::dto::user::{SignUpResponse, UserPublic};
use tracing::{info, error};

pub struct AuthService;

impl AuthService {
    pub async fn register_or_login_guest(pool: &DbPool, guest: Guest) -> Result<SignUpResponse> {
        info!("AUTH SERVICE: Processing registration - username={}, wallet_address={}", guest.username, guest.wallet_address);

        // Check if user already exists
        info!("AUTH SERVICE: Checking if wallet already exists in database...");
        match UserRepository::find_by_wallet_address(pool, &guest.wallet_address).await? {
            Some(existing_user) => {
                info!("AUTH SERVICE: Found existing user - id={}, username={}", existing_user.id, existing_user.username);

                let user = if existing_user.username != guest.username {
                    info!("AUTH SERVICE: Username changed from '{}' to '{}', updating...", existing_user.username, guest.username);
                    // Update username if it's different
                    match UserRepository::update_username(pool, &guest.wallet_address, &guest.username).await {
                        Ok(updated_user) => {
                            info!("AUTH SERVICE: Username updated successfully");
                            updated_user
                        },
                        Err(e) => {
                            error!("ERROR AUTH SERVICE: Failed to update username: {:?}", e);
                            existing_user // Fall back to existing user
                        }
                    }
                } else {
                    info!("AUTH SERVICE: Username unchanged, using existing user");
                    existing_user
                };

                Ok(SignUpResponse {
                    user: Self::create_user_public(&user),
                    message: "Welcome back! Login successful.".to_string(),
                })
            },
            None => {
                info!("AUTH SERVICE: No existing user found, creating new user...");
                // Create new user
                let new_user = UserRepository::create_guest(pool, &guest.username, &guest.wallet_address).await?;
                info!("AUTH SERVICE: New user created - id={}, username={}", new_user.id, new_user.username);

                Ok(SignUpResponse {
                    user: Self::create_user_public(&new_user),
                    message: "Account created successfully! Welcome.".to_string(),
                })
            }
        }
    }

    fn create_user_public(user: &User) -> UserPublic {
        UserPublic {
            id: user.id.to_string(),
            username: user.username.clone(),
            wallet_address: user.wallet_address.clone(),
            created_at: user.created_at.map_or("Unknown".to_string(), |dt| dt.to_string()),
        }
    }
}