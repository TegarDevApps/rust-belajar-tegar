use warp::{Reply, Rejection};
use sqlx::MySqlPool;
use serde_json::json;
use crate::models::{LoginRequest, RegisterRequest, CreateUser, AuthResponse};
use crate::database::{create_user, get_user_by_email};
use crate::utils::{hash_password, verify_password, create_jwt};

pub async fn register_handler(
    user_data: RegisterRequest,
    pool: MySqlPool,
) -> Result<impl Reply, Rejection> {
    // Validasi input
    if user_data.username.is_empty() || user_data.email.is_empty() || user_data.password.is_empty() {
        return Ok(warp::reply::with_status(
            warp::reply::json(&json!({
                "error": "All fields are required"
            })),
            warp::http::StatusCode::BAD_REQUEST,
        ));
    }

    if user_data.password.len() < 6 {
        return Ok(warp::reply::with_status(
            warp::reply::json(&json!({
                "error": "Password must be at least 6 characters"
            })),
            warp::http::StatusCode::BAD_REQUEST,
        ));
    }

    // Cek apakah email sudah terdaftar
    match get_user_by_email(&pool, &user_data.email).await {
        Ok(Some(_)) => {
            return Ok(warp::reply::with_status(
                warp::reply::json(&json!({
                    "error": "Email already registered"
                })),
                warp::http::StatusCode::CONFLICT,
            ));
        }
        Ok(None) => {
            // Email belum terdaftar, lanjutkan
        }
        Err(_) => {
            return Ok(warp::reply::with_status(
                warp::reply::json(&json!({
                    "error": "Database error"
                })),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
    }

    // Hash password
    let hashed_password = match hash_password(&user_data.password) {
        Ok(hash) => hash,
        Err(_) => {
            return Ok(warp::reply::with_status(
                warp::reply::json(&json!({
                    "error": "Failed to hash password"
                })),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
    };

    // Buat user baru
    let new_user = CreateUser {
        username: user_data.username.clone(),
        email: user_data.email.clone(),
        password_hash: hashed_password,
    };

    match create_user(&pool, new_user).await {
        Ok(user_id) => {
            // Generate JWT token
            match create_jwt(user_id, &user_data.email) {
                Ok(token) => {
                    let response = AuthResponse {
                        token,
                        user: crate::models::UserResponse {
                            id: user_id,
                            username: user_data.username,
                            email: user_data.email,
                        },
                    };
                    Ok(warp::reply::with_status(
                        warp::reply::json(&response),
                        warp::http::StatusCode::CREATED,
                    ))
                }
                Err(_) => Ok(warp::reply::with_status(
                    warp::reply::json(&json!({
                        "error": "Failed to generate token"
                    })),
                    warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                )),
            }
        }
        Err(_) => Ok(warp::reply::with_status(
            warp::reply::json(&json!({
                "error": "Failed to create user"
            })),
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}

pub async fn login_handler(
    login_data: LoginRequest,
    pool: MySqlPool,
) -> Result<impl Reply, Rejection> {
    // Validasi input
    if login_data.email.is_empty() || login_data.password.is_empty() {
        return Ok(warp::reply::with_status(
            warp::reply::json(&json!({
                "error": "Email and password are required"
            })),
            warp::http::StatusCode::BAD_REQUEST,
        ));
    }

    // Cari user berdasarkan email
    match get_user_by_email(&pool, &login_data.email).await {
        Ok(Some(user)) => {
            // Verifikasi password
            match verify_password(&login_data.password, &user.password_hash) {
                Ok(true) => {
                    // Password benar, generate JWT
                    match create_jwt(user.id, &user.email) {
                        Ok(token) => {
                            let response = AuthResponse {
                                token,
                                user: user.into(),
                            };
                            Ok(warp::reply::with_status(
                                warp::reply::json(&response),
                                warp::http::StatusCode::OK,
                            ))
                        }
                        Err(_) => Ok(warp::reply::with_status(
                            warp::reply::json(&json!({
                                "error": "Failed to generate token"
                            })),
                            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                        )),
                    }
                }
                Ok(false) => Ok(warp::reply::with_status(
                    warp::reply::json(&json!({
                        "error": "Invalid credentials"
                    })),
                    warp::http::StatusCode::UNAUTHORIZED,
                )),
                Err(_) => Ok(warp::reply::with_status(
                    warp::reply::json(&json!({
                        "error": "Password verification failed"
                    })),
                    warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                )),
            }
        }
        Ok(None) => Ok(warp::reply::with_status(
            warp::reply::json(&json!({
                "error": "Invalid credentials"
            })),
            warp::http::StatusCode::UNAUTHORIZED,
        )),
        Err(_) => Ok(warp::reply::with_status(
            warp::reply::json(&json!({
                "error": "Database error"
            })),
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}