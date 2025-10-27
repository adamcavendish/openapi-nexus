//! TypeScript type utilities

/// Check if a type name is a primitive TypeScript type
pub fn is_primitive_type(type_name: &str) -> bool {
    // Handle generic types like Promise<T>, Array<T>, etc.
    if type_name.contains('<') {
        let base_type = type_name.split('<').next().unwrap_or(type_name);
        return matches!(
            base_type,
            "Promise"
                | "Array"
                | "Map"
                | "Set"
                | "WeakMap"
                | "WeakSet"
                | "ReadonlyArray"
                | "ReadonlyMap"
                | "ReadonlySet"
        );
    }

    matches!(
        type_name,
        "string"
            | "number"
            | "boolean"
            | "any"
            | "unknown"
            | "null"
            | "undefined"
            | "void"
            | "object"
            | "Promise"
            | "Array"
            | "Response"
            | "Error"
            | "Date"
            | "RegExp"
            | "Map"
            | "Set"
            | "WeakMap"
            | "WeakSet"
            | "ReadonlyArray"
            | "ReadonlyMap"
            | "ReadonlySet"
            | "Partial"
            | "Required"
            | "Pick"
            | "Omit"
            | "Record"
            | "Exclude"
            | "Extract"
            | "NonNullable"
            | "Parameters"
            | "ReturnType"
            | "InstanceType"
            | "RequestInit"
    )
}

/// Check if a type name is a runtime type (from our runtime library)
pub fn is_runtime_type(type_name: &str) -> bool {
    matches!(
        type_name,
        "BaseAPI" | "Configuration" | "RequestContext" | "ApiResponse" | "HttpMethod"
    )
}
