//! Petstore API example using Axum and utoipa

pub mod handlers;
pub mod models;
pub mod openapi;

// Re-export commonly used items
pub use handlers::{
    update_pet, add_pet, find_pets_by_status, find_pets_by_tags, get_pet_by_id,
    update_pet_with_form, delete_pet, upload_file, get_inventory, place_order,
    get_order_by_id, delete_order, create_user, create_users_with_list_input,
    login_user, logout_user, get_user_by_name, update_user, delete_user,
};
pub use models::{
    Pet, PetStatus, Category, Tag, Order, OrderStatus, User, ApiResponse, ErrorResponse,
};
pub use openapi::ApiDoc;
