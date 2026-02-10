// ============================================================
// Generated from Pact module: user-service
// Version: 7
// Spec: SPEC-2024-0042
// Author: agent:claude-v4
// ============================================================

use pact_runtime::prelude::*;
use serde::{Serialize, Deserialize};
use std::fmt;

/// Type: User
///
/// Invariants:
/// - (> (strlen name) 0)
/// - (matches email #/.+@.+\..+/)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

impl HasId for User {
    fn id(&self) -> Uuid { self.id }
}

impl HasUniqueFields for User {
    fn unique_fields(&self) -> Vec<(&'static str, String)> {
        vec![("email", self.email.clone())]
    }
}

impl User {
    pub fn validate(&self) -> Vec<ValidationError> {
        let mut errors = Vec::new();
        if self.name.len() < 1 { errors.push(ValidationError { field: "name".into(), message: "must be at least 1 characters".into() }); }
        if self.name.len() > 200 { errors.push(ValidationError { field: "name".into(), message: "must be at most 200 characters".into() }); }
        errors
    }

    pub fn validate_input(input: &CreateUserInput) -> Vec<ValidationError> {
        let mut errors = Vec::new();
        if input.name.len() < 1 { errors.push(ValidationError { field: "name".into(), message: "must be at least 1 characters".into() }); }
        if input.name.len() > 200 { errors.push(ValidationError { field: "name".into(), message: "must be at most 200 characters".into() }); }
        errors
    }

    pub fn from_input(input: CreateUserInput) -> Self {
        User {
            id: Uuid::new_v4(),
            name: input.name,
            email: input.email,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserInput {
    pub name: String,
    pub email: String,
}

/// Total: this function handles all cases exhaustively
#[derive(Debug)]
pub enum GetUserByIdResult {
    /// HTTP 200
    Ok(User),
    /// HTTP 404
    NotFound { id: String },
    /// HTTP 400
    InvalidId { id: String },
}

impl GetUserByIdResult {
    pub fn http_status(&self) -> u16 {
        match self {
            GetUserByIdResult::Ok(_) => 200,
            GetUserByIdResult::NotFound { .. } => 404,
            GetUserByIdResult::InvalidId { .. } => 400,
        }
    }
}

impl fmt::Display for GetUserByIdResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GetUserByIdResult::Ok(v) => write!(f, "Ok: {:?}", v),
            GetUserByIdResult::NotFound { .. } => write!(f, "Error: not-found"),
            GetUserByIdResult::InvalidId { .. } => write!(f, "Error: invalid-id"),
        }
    }
}

/// Total: this function handles all cases exhaustively
#[derive(Debug)]
pub enum CreateUserResult {
    /// HTTP 201
    Ok(User),
    /// HTTP 409
    DuplicateEmail { email: String },
    /// HTTP 422
    ValidationFailed(Vec<ValidationError>),
}

impl CreateUserResult {
    pub fn http_status(&self) -> u16 {
        match self {
            CreateUserResult::Ok(_) => 201,
            CreateUserResult::DuplicateEmail { .. } => 409,
            CreateUserResult::ValidationFailed(_) => 422,
        }
    }
}

impl fmt::Display for CreateUserResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CreateUserResult::Ok(v) => write!(f, "Ok: {:?}", v),
            CreateUserResult::DuplicateEmail { .. } => write!(f, "Error: duplicate-email"),
            CreateUserResult::ValidationFailed(v) => write!(f, "Error(validation-failed): {:?}", v),
        }
    }
}

/// Spec: SPEC-2024-0042#section-3
/// Total: handles all cases exhaustively
pub fn get_user_by_id(store: &impl Store<User>, id: &str) -> GetUserByIdResult {
    let validated_id = validate_uuid(id);
    match validated_id {
        Err(_) => GetUserByIdResult::InvalidId { id: id.to_string() },
        Ok(uuid) => match store.query_by_id(&uuid) {
            None => GetUserByIdResult::NotFound { id: uuid.to_string() },
            Some(u) => GetUserByIdResult::Ok(u),
        },
    }
}

/// Spec: SPEC-2024-0041
/// Total: handles all cases exhaustively
pub fn create_user(store: &mut impl Store<User>, input: CreateUserInput) -> CreateUserResult {
    let errors = User::validate_input(&input);
    if non_empty(&errors) {
        CreateUserResult::ValidationFailed(errors)
    } else {
        match store.insert(User::from_input(input.clone())) {
            Err(StoreError::UniqueViolation { .. }) => CreateUserResult::DuplicateEmail { email: input.email.clone() },
            Ok(user) => CreateUserResult::Ok(user),
            Err(e) => panic!("unexpected store error: {:?}", e),
        }
    }
}

