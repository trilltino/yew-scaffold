use sqlx::PgPool;
use crate::services::AuthService;
use crate::database::repositories::user_repository::UserRepository;
use crate::config::AppConfig;
use crate::extractors::CurrentUser;
use crate::error::{AppError, Result};
use axum::{
    extract::{Json, State},
    response::IntoResponse,
    http::StatusCode,
};
use shared::dto::auth::{Guest, SignupRequest, LoginRequest, LoginResponse, LinkWalletRequest};
use shared::dto::common::ApiResponse;
use tower_cookies::Cookies;
use tracing::{info, error, debug, warn};
use serde::{Serialize, Deserialize};

/// Response for signup endpoint
#[derive(Debug, Serialize, Deserialize)]
pub struct SignupResponse {
    pub user_id: i32,
    pub username: String,
    pub email: String,
    pub role: String,
}

/// Response for "me" endpoint (current user)
#[derive(Debug, Serialize, Deserialize)]
pub struct MeResponse {
    pub id: i32,
    pub username: String,
    pub email: Option<String>,
    pub wallet_address: String,
    pub role: String,
    pub email_verified: bool,
}

/// Legacy wallet-only registration (guest user)
pub async fn register_guest(
    State(pool): State<PgPool>,
    Json(guest): Json<Guest>,
) -> impl IntoResponse {
    info!("[AUTH] Received guest registration - username: {}, wallet: {}...{}",
          guest.username,
          &guest.wallet_address[..6],
          &guest.wallet_address[guest.wallet_address.len()-6..]);

    match AuthService::register_or_login_guest(&pool, guest).await {
        Ok(response) => {
            info!("[AUTH] ✅ Guest user saved - user_id: {}, message: {}",
                  response.user.id,
                  response.message);
            (
                StatusCode::OK,
                Json(ApiResponse::success(response, "Authentication successful"))
            ).into_response()
        },
        Err(err) => {
            error!("[AUTH] ❌ Failed to save guest user: {:?}", err);
            err.into_response()
        },
    }
}

/// Sign up with email and password
pub async fn signup_with_password(
    State(pool): State<PgPool>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse> {
    info!("[AUTH] Signup request - username: {}, email: {}", request.username, request.email);

    // Validate email format
    if !request.email.contains('@') {
        warn!("[AUTH] ❌ Invalid email format: {}", request.email);
        return Err(AppError::BadRequest("Invalid email format".to_string()));
    }

    // Check if email already exists
    if let Some(_existing_user) = UserRepository::find_by_email(&pool, &request.email).await
        .map_err(|e| AppError::Database(format!("Failed to check email: {}", e)))? {
        warn!("[AUTH] ❌ Email already registered: {}", request.email);
        return Err(AppError::Conflict("Email already registered".to_string()));
    }

    // Hash password
    let password_hash = crate::auth::hash_password(&request.password)
        .map_err(|e| AppError::Internal(format!("Failed to hash password: {}", e)))?;

    // Create user
    let user = UserRepository::create_with_password(&pool, &request.username, &request.email, &password_hash).await
        .map_err(|e| AppError::Database(format!("Failed to create user: {}", e)))?;

    info!("[AUTH] ✅ User created successfully - id: {}, username: {}, email: {}",
          user.id, user.username, user.email.as_deref().unwrap_or("none"));

    let response = SignupResponse {
        user_id: user.id,
        username: user.username,
        email: user.email.unwrap_or_default(),
        role: user.role,
    };

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(response, "User created successfully"))
    ))
}

/// Login with email and password
pub async fn login(
    State(pool): State<PgPool>,
    State(config): State<AppConfig>,
    cookies: Cookies,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse> {
    info!("[AUTH] Login request - email: {}", request.email);

    // Find user by email
    let user = UserRepository::find_by_email(&pool, &request.email).await
        .map_err(|e| AppError::Database(format!("Database error: {}", e)))?
        .ok_or_else(|| {
            warn!("[AUTH] ❌ Login failed: User not found - email: {}", request.email);
            AppError::Unauthorized("Invalid email or password".to_string())
        })?;

    // Verify password
    let password_hash = user.password_hash
        .as_ref()
        .ok_or_else(|| {
            error!("[AUTH] ❌ User has no password hash (wallet-only user) - id: {}", user.id);
            AppError::Unauthorized("This account uses wallet authentication".to_string())
        })?;

    let is_valid = crate::auth::verify_password(&request.password, password_hash)
        .map_err(|e| AppError::Internal(format!("Password verification failed: {}", e)))?;

    if !is_valid {
        warn!("[AUTH] ❌ Login failed: Invalid password - user_id: {}", user.id);
        return Err(AppError::Unauthorized("Invalid email or password".to_string()));
    }

    // Generate JWT token
    let token = crate::auth::encode_jwt(
        user.id,
        user.username.clone(),
        user.role.clone(),
        &config.jwt_secret,
        config.jwt_expiration_hours,
    )
    .map_err(|e| AppError::Internal(format!("Failed to generate token: {}", e)))?;

    // Create auth cookie
    let cookie = crate::auth::create_auth_cookie(token.clone(), &config);
    cookies.add(cookie);

    // Update last login
    UserRepository::update_last_login(&pool, user.id).await
        .map_err(|e| {
            error!("[AUTH] ⚠️ Failed to update last_login for user {}: {}", user.id, e);
        })
        .ok(); // Don't fail login if this fails

    info!("[AUTH] ✅ Login successful - user_id: {}, username: {}", user.id, user.username);

    let response = LoginResponse {
        user_id: user.id,
        username: user.username,
        email: user.email,
        wallet_address: if user.wallet_address.is_empty() { None } else { Some(user.wallet_address) },
        role: user.role,
        token,
    };

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(response, "Login successful"))
    ))
}

/// Logout (clear auth cookie)
pub async fn logout(
    State(config): State<AppConfig>,
    cookies: Cookies,
) -> Result<impl IntoResponse> {
    info!("[AUTH] Logout request");

    let cookie = crate::auth::create_logout_cookie(&config);
    cookies.add(cookie);

    info!("[AUTH] ✅ Logout successful - cookie cleared");

    Ok((
        StatusCode::OK,
        Json(ApiResponse::<()>::success_no_data("Logged out successfully"))
    ))
}

/// Get current authenticated user
pub async fn me(
    State(pool): State<PgPool>,
    current_user: CurrentUser,
) -> Result<impl IntoResponse> {
    debug!("[AUTH] Fetching current user - user_id: {}", current_user.user_id);

    // Fetch fresh user data from database
    let user = UserRepository::find_by_id(&pool, current_user.user_id).await
        .map_err(|e| AppError::Database(format!("Database error: {}", e)))?
        .ok_or_else(|| {
            error!("[AUTH] ❌ User not found in database - user_id: {}", current_user.user_id);
            AppError::Unauthorized("User not found".to_string())
        })?;

    debug!("[AUTH] ✅ Current user fetched - username: {}, role: {}", user.username, user.role);

    let response = MeResponse {
        id: user.id,
        username: user.username,
        email: user.email,
        wallet_address: user.wallet_address,
        role: user.role,
        email_verified: user.email_verified,
    };

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(response, "Current user"))
    ))
}

/// Link wallet address to existing authenticated user
pub async fn link_wallet(
    State(pool): State<PgPool>,
    current_user: CurrentUser,
    Json(request): Json<LinkWalletRequest>,
) -> Result<impl IntoResponse> {
    info!("[AUTH] Link wallet request - user_id: {}, wallet: {}...{}",
          current_user.user_id,
          &request.wallet_address[..6],
          &request.wallet_address[request.wallet_address.len()-6..]);

    // Link wallet to user
    let user = UserRepository::link_wallet(&pool, current_user.user_id, &request.wallet_address).await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                warn!("[AUTH] ❌ Wallet already linked to another account");
                AppError::Conflict("Wallet address already linked to another account".to_string())
            }
            _ => AppError::Database(format!("Failed to link wallet: {}", e))
        })?;

    info!("[AUTH] ✅ Wallet linked successfully - user_id: {}, wallet: {}", user.id, user.wallet_address);

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(user, "Wallet linked successfully"))
    ))
}