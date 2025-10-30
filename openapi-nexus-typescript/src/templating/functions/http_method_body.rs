pub fn http_method_body(http_method: &str) -> String {
    let template_name = match http_method.to_uppercase().as_str() {
        "GET" => "api_method_get",
        "POST" | "PUT" | "PATCH" => "api_method_post_put_patch",
        "DELETE" => "api_method_delete",
        _ => "default_method",
    };

    template_name.to_string()
}
