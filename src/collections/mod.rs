//! Collections and workspaces for organizing requests

pub mod collection;
pub mod folder;
pub mod request_item;
pub mod storage;
pub mod workspace;

pub use collection::{Collection, CollectionInfo};
pub use folder::Folder;
pub use request_item::RequestItem;
pub use storage::CollectionStorage;
pub use workspace::{Workspace, WorkspaceStorage};
