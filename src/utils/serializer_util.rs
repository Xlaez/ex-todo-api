use crate::{models::UserModel, schemas::UserResponse};

impl From<UserModel> for UserResponse{
    fn from(value: UserModel) -> Self {
        UserResponse {
            id: value.id,
            username: value.username,
            email: value.email,
            email_verified: value.email_verified,
            img: value.img,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}