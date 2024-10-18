use std::collections::HashMap;

pub fn merge_styles(
    a: HashMap<String, String>,
    b: Option<HashMap<String, String>>,
) -> HashMap<String, String> {
    let mut result = HashMap::new();

    result.extend(a);

    if let Some(b) = b {
        result.extend(b);
    }

    result
}
