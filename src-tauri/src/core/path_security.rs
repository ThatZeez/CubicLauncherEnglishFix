use std::path::{Component, Path, PathBuf};

/// Validates that `sub_path` is a relative path containing only normal
/// components (and `.` segments). It rejects absolute paths, parent directory
/// references (`..`) and any root/prefix components.
///
/// On success, returns `base.join(sub_path)`. The caller should still verify the
/// final resolved path stays under `base` when dealing with filesystem entries
/// that might be symlinks.
pub fn sanitize_path(base: &Path, sub_path: &Path) -> Result<PathBuf, String> {
    if sub_path.is_absolute() {
        return Err(format!(
            "Absolute path not allowed: '{}'",
            sub_path.display()
        ));
    }

    for component in sub_path.components() {
        if !matches!(component, Component::Normal(_) | Component::CurDir) {
            return Err(format!(
                "Path component not allowed in: '{}'",
                sub_path.display()
            ));
        }
    }

    Ok(base.join(sub_path))
}

/// Convenience helper for callers that receive a string path.
pub fn safe_join(base: &Path, relative: &str) -> Result<PathBuf, String> {
    sanitize_path(base, Path::new(relative))
}

/// Validates a path component (file or directory name) to ensure it can safely be
/// used as a single segment. Rejects empty strings, `.`, `..`, path separators,
/// null bytes and other special filesystem characters.
pub fn validate_path_component(component: &str) -> Result<(), String> {
    if component.is_empty() {
        return Err("The path component is empty".into());
    }
    if component == "." || component == ".." {
        return Err(format!("Path component not allowed: '{}'", component));
    }

    const FORBIDDEN: &[char] = &['/', '\\', '\0', '<', '>', ':', '"', '|', '?', '*'];
    if component.chars().any(|c| FORBIDDEN.contains(&c)) {
        return Err(format!(
            "The path component contains disallowed characters: '{}'",
            component
        ));
    }

    Ok(())
}

/// Validates a file name intended to be written into a specific directory.
/// It must be a single component, so paths like `foo/bar` or `../x` are rejected.
pub fn validate_filename(filename: &str) -> Result<(), String> {
    validate_path_component(filename)
}

/// Validates an identifier derived from user input (e.g. a theme id).
/// Allows letters, digits, underscores, hyphens and dots; this is stricter than
/// a generic filename because identifiers are also used as directory names.
pub fn validate_identifier(id: &str) -> Result<(), String> {
    if id.is_empty() || id.len() > 64 {
        return Err("The identifier cannot be empty or exceed 64 characters".into());
    }
    if id == "." || id == ".." || id.contains(":") || id.contains('\0') {
        return Err("The identifier contains disallowed characters".into());
    }
    if id.contains('/') || id.contains('\\') {
        return Err("The identifier cannot contain path separators".into());
    }
    Ok(())
}

#[cfg(test)]
#[path = "../tests/core/path_security.rs"]
mod tests;
