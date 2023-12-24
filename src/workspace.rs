use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Workspace {
    pub name: String,
    pub path: PathBuf,
}

#[derive(Debug)]
pub struct WorkspaceManager {
    pub workspaces: HashMap<String, Workspace>,
    current_workspace: String,
}

#[derive(Debug)]
pub struct WorkspaceNotFound {
    pub workspace: String,
}

impl<'a> WorkspaceManager {
    /// Creates a new workspace manager with a single workspace, setting it as the default.
    ///
    /// * `workspace`: The single workspace to use
    pub fn from_single_workspace(workspace: Workspace) -> WorkspaceManager {
        let name = workspace.name.clone();
        WorkspaceManager {
            current_workspace: name.clone(),
            workspaces: HashMap::from([(name, workspace)]),
        }
    }

    /// Creates a new workspace manager from a list of workspaces.
    /// If the default workspace is not found, an error is returned.
    ///
    /// * `workspaces`: A list of workspaces to add to the workspace manager.
    /// * `default_workspace`: The name of the default workspace.
    pub fn new(
        workspaces: Vec<Workspace>,
        default_workspace: String,
    ) -> Result<WorkspaceManager, WorkspaceNotFound> {
        if !workspaces.iter().any(|w| w.name == default_workspace) {
            Err(WorkspaceNotFound {
                workspace: default_workspace.to_string(),
            })
        } else {
            Ok(WorkspaceManager {
                current_workspace: default_workspace,
                workspaces: workspaces
                    .into_iter()
                    .map(|w| (w.name.clone(), w))
                    .collect::<HashMap<_, _>>(),
            })
        }
    }

    /// Returns a workspace with the given name, or None if it doesn't exist.
    ///
    /// * `name`: The name of the workspace.
    pub fn get_workspace(&self, name: &String) -> Option<&Workspace> {
        self.workspaces.get(name)
    }

    /// Sets the current workspace to the workspace with the given name.
    /// Returns unit if the workspace was set, else returns a WorkspaceNotFound error.
    ///
    /// * `name`: The name of the workspace to set as the current workspace.
    pub fn set_current_workspace(&mut self, name: String) -> Result<(), WorkspaceNotFound> {
        if self.workspaces.get(&name).is_none() {
            Err(WorkspaceNotFound {
                workspace: name.to_string(),
            })
        } else {
            self.current_workspace = name;
            Ok(())
        }
    }

    /// Returns the current workspace.
    pub fn get_current_workspace(&self) -> &Workspace {
        self.workspaces.get(&self.current_workspace).unwrap()
    }

    /// Adds a workspace to the list of workspaces.
    /// Overwrites any existing workspace with the same name.
    ///
    /// * `workspace`: The workspace to add to the list of workspaces.
    pub fn add_workspace(&mut self, workspace: Workspace) {
        self.workspaces.insert(workspace.name.clone(), workspace);
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_workspace_manager_from_single_workspace() {
        let workspace = Workspace {
            name: "example name".to_string(),
            path: "~/some/path".into(),
        };

        let workspace_manager = WorkspaceManager::from_single_workspace(workspace);
        assert_eq!(&workspace_manager.current_workspace, "example name");
    }

    #[test]
    fn test_workspace_manager_new() {
        let workspace1 = Workspace {
            name: "example name".to_string(),
            path: "~/some/path".into(),
        };

        let workspace2 = Workspace {
            name: "another example name".to_string(),
            path: "~/another/path".into(),
        };

        let workspace_manager =
            WorkspaceManager::new(vec![workspace1, workspace2], "example name".to_string())
                .unwrap();
        assert_eq!(&workspace_manager.current_workspace, "example name");
    }

    #[test]
    fn test_workspace_manager_current_workspace() {
        let workspace1 = Workspace {
            name: "example name".to_string(),
            path: "~/some/path".into(),
        };

        let workspace2 = Workspace {
            name: "another example name".to_string(),
            path: "~/another/path".into(),
        };

        let mut workspace_manager =
            WorkspaceManager::new(vec![workspace1, workspace2], "example name".to_string())
                .expect("Workspace 1 not found!");
        workspace_manager
            .set_current_workspace("another example name".to_string())
            .expect("Unable to set the current workspace");

        assert_eq!(
            &workspace_manager.get_current_workspace().name,
            "another example name"
        );
    }
}
