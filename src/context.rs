use std::env;
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CommandContext {
    pub(crate) current_dir: Option<String>,
    pub(crate) workspace_root: Option<String>,
}

impl CommandContext {
    pub(crate) fn capture() -> Self {
        let current_dir = env::current_dir().ok();
        let workspace_root = current_dir
            .as_deref()
            .and_then(workspace_root_for)
            .map(path_to_string);

        Self {
            current_dir: current_dir.map(path_to_string),
            workspace_root,
        }
    }
}

pub(crate) fn workspace_root_for(path: &Path) -> Option<PathBuf> {
    let output = Command::new("git")
        .arg("-C")
        .arg(path)
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8(output.stdout).ok()?;
    let root = stdout.trim();
    if root.is_empty() {
        None
    } else {
        Some(PathBuf::from(root))
    }
}

#[cfg(test)]
pub(crate) fn path_is_same_or_under(path: &str, root: &str) -> bool {
    let path_components = normalized_path_components(path);
    let root_components = normalized_path_components(root);

    components_are_same_or_under(&path_components, &root_components)
}

#[cfg(test)]
pub(crate) fn paths_are_same(left: &str, right: &str) -> bool {
    normalized_path_components(left) == normalized_path_components(right)
}

pub(crate) fn normalized_path_components(path: &str) -> Vec<String> {
    normalized_components(&comparable_path(Path::new(path)))
}

pub(crate) fn components_are_same_or_under(path: &[String], root: &[String]) -> bool {
    !root.is_empty()
        && path.len() >= root.len()
        && path
            .iter()
            .zip(root.iter())
            .all(|(path_component, root_component)| path_component == root_component)
}

fn comparable_path(path: &Path) -> PathBuf {
    fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
}

fn normalized_components(path: &Path) -> Vec<String> {
    path.components()
        .filter_map(|component| match component {
            Component::Prefix(prefix) => Some(prefix.as_os_str().to_string_lossy().into_owned()),
            Component::RootDir => Some("/".to_string()),
            Component::Normal(value) => Some(value.to_string_lossy().into_owned()),
            Component::CurDir => None,
            Component::ParentDir => Some("..".to_string()),
        })
        .collect()
}

fn path_to_string(path: PathBuf) -> String {
    path.to_string_lossy().into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_is_same_or_under_matches_component_boundaries() {
        assert!(path_is_same_or_under("/repo", "/repo"));
        assert!(path_is_same_or_under("/repo/crate/src", "/repo"));
        assert!(!path_is_same_or_under("/repo-other", "/repo"));
        assert!(!path_is_same_or_under("/tmp/repo", "/repo"));
        assert!(paths_are_same("/repo", "/repo"));
        assert!(!paths_are_same("/repo/api", "/repo"));
    }
}
