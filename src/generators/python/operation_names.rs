use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PythonOperationNames {
    pub method: String,
    pub with_http_info_method: String,
}

/// Reserve both public method names for every operation in an API class.
///
/// The metadata suffix can otherwise collide with another valid operation
/// name, such as `getPet` and `getPetWithHttpInfo`.
pub(crate) fn plan_python_operation_names(
    preferred_methods: impl IntoIterator<Item = String>,
) -> Vec<PythonOperationNames> {
    let mut used = HashSet::new();
    let methods: Vec<String> = preferred_methods
        .into_iter()
        .map(|preferred| unique_name(&preferred, &mut used))
        .collect();

    methods
        .into_iter()
        .map(|method| {
            let with_http_info_method = unique_name(&format!("{method}_with_http_info"), &mut used);
            PythonOperationNames {
                method,
                with_http_info_method,
            }
        })
        .collect()
}

fn unique_name(preferred: &str, used: &mut HashSet<String>) -> String {
    if used.insert(preferred.to_string()) {
        return preferred.to_string();
    }
    for index in 2..=u32::MAX {
        let candidate = format!("{preferred}{index}");
        if used.insert(candidate.clone()) {
            return candidate;
        }
    }
    unreachable!("Python operation name space exhausted")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metadata_method_does_not_replace_an_operation_method() {
        let names = plan_python_operation_names([
            "get_pet".to_string(),
            "get_pet_with_http_info".to_string(),
        ]);

        assert_eq!(names[0].method, "get_pet");
        assert_eq!(names[0].with_http_info_method, "get_pet_with_http_info2");
        assert_eq!(names[1].method, "get_pet_with_http_info");
        assert_eq!(
            names[1].with_http_info_method,
            "get_pet_with_http_info_with_http_info"
        );
    }
}
