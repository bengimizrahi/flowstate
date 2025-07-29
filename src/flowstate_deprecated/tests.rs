use super::*;

#[test]
fn test_create_delete_team() {
    let mut flow_state = FlowState::new();

    let team_name: String = "Test Team".to_string();

    assert!(flow_state.create_team(team_name.clone()).is_ok());
    assert_eq!(flow_state.teams.len(), 1);
    assert!(flow_state.teams.contains(&team_name));

    assert!(flow_state.delete_team(&team_name).is_ok());
    assert_eq!(flow_state.teams.len(), 0);
    assert!(!flow_state.teams.contains(&team_name));
}

#[test]
fn test_create_delete_team_undo_undo_redo_redo() {
    let mut flow_state = FlowState::new();

    let team_name = "Test Team".to_string();
    
    assert!(flow_state.create_team(team_name.clone()).is_ok());
    assert_eq!(flow_state.teams.len(), 1);
    assert!(flow_state.teams.contains(&team_name));
    println!("After create: {flow_state:#?}");

    assert!(flow_state.delete_team(&team_name).is_ok());
    assert_eq!(flow_state.teams.len(), 0);
    assert!(!flow_state.teams.contains(&team_name));
    println!("After delete: {flow_state:#?}");
    
    assert!(flow_state.undo().is_ok());
    assert_eq!(flow_state.teams.len(), 1);
    assert!(flow_state.teams.contains(&team_name));
    println!("After undo: {flow_state:#?}");

    assert!(flow_state.undo().is_ok());
    assert_eq!(flow_state.teams.len(), 0);
    assert!(!flow_state.teams.contains(&team_name));
    println!("After undo: {flow_state:#?}");

    assert!(flow_state.undo().is_err());
    println!("After undo: {flow_state:#?}");
    
    assert!(flow_state.redo().is_ok());
    assert_eq!(flow_state.teams.len(), 1);
    assert!(flow_state.teams.contains(&team_name));
    println!("After redo: {flow_state:#?}");

    assert!(flow_state.redo().is_ok());
    assert_eq!(flow_state.teams.len(), 0);
    assert!(!flow_state.teams.contains(&team_name));
    println!("After redo: {flow_state:#?}");

    assert!(flow_state.redo().is_err());
    println!("After redo: {flow_state:#?}");
}

#[test]
fn test_rename_team_undo_redo() {
    let mut flow_state = FlowState::new();
    let team_name = "Test Team".to_string();
    assert!(flow_state.create_team(team_name.clone()).is_ok());

    let new_team_name = "Renamed Team".to_string();
    assert!(flow_state.rename_team(&team_name, &new_team_name).is_ok());
    assert_eq!(flow_state.teams.len(), 1);
    assert!(flow_state.teams.contains(&new_team_name));
    assert!(!flow_state.teams.contains(&team_name));
    println!("After rename: {flow_state:#?}");

    assert!(flow_state.undo().is_ok());
    assert_eq!(flow_state.teams.len(), 1);
    assert!(flow_state.teams.contains(&team_name));
    assert!(!flow_state.teams.contains(&new_team_name));
    println!("After undo: {flow_state:#?}");

    assert!(flow_state.redo().is_ok());
    assert_eq!(flow_state.teams.len(), 1);
    assert!(flow_state.teams.contains(&new_team_name));
    assert!(!flow_state.teams.contains(&team_name));
    println!("After redo: {flow_state:#?}");
}

#[test]
fn test_save_load_yaml() {
    let mut flow_state = FlowState::new();
    let team_name = "Test Team".to_string();
    assert!(flow_state.create_team(team_name.clone()).is_ok());
    assert_eq!(flow_state.teams.len(), 1);
    assert!(flow_state.teams.contains(&team_name));
    assert!(flow_state.save_as_yaml().is_ok());
    println!("After save: {flow_state:#?}");

    let mut loaded_flow_state = FlowState::new();
    assert!(loaded_flow_state.load_from_yaml().is_ok());
    assert_eq!(loaded_flow_state.teams.len(), 1);
    assert!(loaded_flow_state.teams.contains(&team_name));
    assert_eq!(loaded_flow_state.teams.get(&team_name).unwrap(), &team_name);
    println!("After load: {loaded_flow_state:#?}");
}

#[test]
fn test_create_rename_delete_resource() {
    let mut flow_state = FlowState::new();

    let team_name: String = "Test Team".to_string();
    assert!(flow_state.create_team(team_name.clone()).is_ok());

    let resource_name: String = "Test Resource".to_string();
    assert!(flow_state.create_resource(Resource {
        name: resource_name.clone(),
        team_name: team_name.clone(),
    }).is_ok());
    assert_eq!(flow_state.resources.len(), 1);
    assert!(flow_state.resources.contains_key(&resource_name));

    let updated_resource_name: String = "Updated Resource".to_string();
    assert!(flow_state.rename_resource(&resource_name, &updated_resource_name).is_ok());
    assert!(!flow_state.resources.contains_key(&resource_name));
    assert!(flow_state.resources.contains_key(&updated_resource_name));

    assert!(flow_state.delete_resource(&updated_resource_name).is_ok());
    assert_eq!(flow_state.resources.len(), 0);
    assert!(!flow_state.resources.contains_key(&updated_resource_name));
}