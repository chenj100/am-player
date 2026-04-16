use crate::domain::storage_destination::StorageDestination;

pub fn default_destinations() -> Vec<StorageDestination> {
    vec![
        StorageDestination {
            destination_id: "shared_local".to_string(),
            destination_type: "shared_local".to_string(),
            display_name: "Device storage".to_string(),
            writable: true,
            requires_auth: false,
            capacity_hint_bytes: None,
            availability_state: "ready".to_string(),
        },
        StorageDestination {
            destination_id: "google_drive".to_string(),
            destination_type: "google_drive".to_string(),
            display_name: "Google Drive backup".to_string(),
            writable: true,
            requires_auth: true,
            capacity_hint_bytes: None,
            availability_state: "auth_required".to_string(),
        },
    ]
}
