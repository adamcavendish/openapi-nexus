//! OpenAPI specification for the Petstore API

use crate::handlers::*;
use crate::models::*;

/// OpenAPI documentation
#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        update_pet,
        add_pet,
        find_pets_by_status,
        find_pets_by_tags,
        get_pet_by_id,
        update_pet_with_form,
        delete_pet,
        upload_file,
        get_inventory,
        place_order,
        get_order_by_id,
        delete_order,
        create_user,
        create_users_with_list_input,
        login_user,
        logout_user,
        get_user_by_name,
        update_user,
        delete_user
    ),
    components(
        schemas(
            Pet,
            PetStatus,
            Category,
            Tag,
            Order,
            OrderStatus,
            User,
            ApiResponse,
            ErrorResponse
        )
    ),
    tags(
        (name = "pet", description = "Everything about your Pets"),
        (name = "store", description = "Access to Petstore orders"),
        (name = "user", description = "Operations about user")
    ),
    info(
        title = "Petstore API",
        version = "1.0.0",
        description = "This is a sample Pet Store Server based on the OpenAPI 3.0 specification",
        contact(
            email = "apiteam@swagger.io"
        ),
        license(
            name = "Apache 2.0",
            url = "https://www.apache.org/licenses/LICENSE-2.0.html"
        )
    )
)]
pub struct ApiDoc;
