//! Zones



static mut ZONES: u32 = 0;

// SAFETY: This must never be called concurrently.
pub fn make_zone() -> Zone {
    unsafe {
        let id = ZONES;
        ZONES += 1;
        Zone {
            id
        }
    }
}

pub struct ZoneTree {
    root: ZoneNode,
    current: u32,
}

impl ZoneTree {
    pub fn new() -> Self {
        let root_zone = make_zone();
        let current = root_zone.id;
        
        Self {
            root: ZoneNode::Leaf(root_zone),
            current,
        }
    }
}

pub enum ZoneNode {
    Branch(Vec<ZoneNode>),
    Leaf(Zone),
}

/// A zone is a portion of the screen.
/// See [make_zone] for details on how to create zones.
pub struct Zone {
    id: u32,
}
