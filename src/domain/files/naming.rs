use std::path::Path;

use uuid::Uuid;

use crate::util::fs::stem;

pub fn parse_uuid_from_path(path: &Path) -> Option<Uuid> {
    let candidate = stem(path);
    if let Ok(uuid) = Uuid::parse_str(&candidate) {
        return Some(uuid);
    }

    if candidate.len() == 32 {
        let with_hyphens = format!(
            "{}-{}-{}-{}-{}",
            &candidate[0..8],
            &candidate[8..12],
            &candidate[12..16],
            &candidate[16..20],
            &candidate[20..32]
        );
        Uuid::parse_str(&with_hyphens).ok()
    } else {
        None
    }
}
