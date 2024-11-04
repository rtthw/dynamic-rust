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

    pub fn current_zone_mut(&mut self) -> Option<&mut Zone> {
        self.root.get_mut(self.current)
    }
}

pub enum ZoneNode {
    Branch(Vec<ZoneNode>),
    Leaf(Zone),
}

impl ZoneNode {
    pub fn get_mut(&mut self, id: u32) -> Option<&mut Zone> {
        match self {
            ZoneNode::Branch(leaves) => {
                for leaf in leaves {
                    if let Some(zone) = leaf.get_mut(id) {
                        return Some(zone);
                    }
                }
                None
            }
            ZoneNode::Leaf(zone) => {
                if zone.id == id {
                    Some(zone)
                } else {
                    None
                }
            }
        }
    }

    pub fn topmost_mut(&mut self) -> Option<&mut Zone> {
        match self {
            ZoneNode::Branch(leaves) => {
                for leaf in leaves {
                    if let Some(zone) = leaf.topmost_mut() {
                        return Some(zone);
                    }
                }
                None
            }
            ZoneNode::Leaf(zone) => Some(zone),
        }
    }
}

/// A zone is a portion of the screen.
/// See [make_zone] for details on how to create zones.
pub struct Zone {
    id: u32,
}

impl Zone {
    pub fn id(&self) -> u32 {
        self.id
    }

}
