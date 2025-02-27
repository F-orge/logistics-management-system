use lib_proto::FileMetadata;

use crate::{file, prelude::*};

// storage file
impl Into<lib_proto::FileMetadata> for file::Model {
    fn into(self) -> lib_proto::FileMetadata {
        FileMetadata {
            id: self.id.to_string(),
            name: self.name,
            r#type: self.r#type,
            size: self.size as u32,
            is_public: self.is_public,
            owner_id: self.owner_id.to_string(),
        }
    }
}
