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

    pub fn root_node_mut(&mut self) -> &mut ZoneNode {
        &mut self.root
    }

    pub fn current_zone_mut(&mut self) -> Option<&mut Zone> {
        self.root.zone_mut(self.current)
    }

    pub fn current_node_mut(&mut self) -> Option<&mut ZoneNode> {
        self.root.node_mut(self.current)
    }
}

impl ZoneTree {
    pub fn split_current(&mut self) {
        if let Some(node) = self.current_node_mut() {
            let owned_node = node.clone();
            let new_node = ZoneNode::Leaf(make_zone());
            // TODO: If ZoneNode stops being clonable, then work some magic here.
            let _old = std::mem::replace(node, ZoneNode::Branch(vec![owned_node, new_node]));
        }
    }
}

#[derive(Clone)]
pub enum ZoneNode {
    Branch(Vec<ZoneNode>),
    Leaf(Zone),
}

impl ZoneNode {
    pub fn zone_mut(&mut self, id: u32) -> Option<&mut Zone> {
        match self {
            ZoneNode::Branch(leaves) => {
                for leaf in leaves {
                    if let Some(zone) = leaf.zone_mut(id) {
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

    pub fn node_mut(&mut self, id: u32) -> Option<&mut ZoneNode> {
        match self {
            ZoneNode::Branch(leaves) => {
                for leaf in leaves {
                    if let Some(zone) = leaf.node_mut(id) {
                        return Some(zone);
                    }
                }
                None
            }
            ZoneNode::Leaf(zone) => {
                if zone.id == id {
                    Some(self)
                } else {
                    None
                }
            }
        }
    }

    pub fn topmost_zone_mut(&mut self) -> Option<&mut Zone> {
        match self {
            ZoneNode::Branch(leaves) => {
                for leaf in leaves {
                    if let Some(zone) = leaf.topmost_zone_mut() {
                        return Some(zone);
                    }
                }
                None
            }
            ZoneNode::Leaf(zone) => Some(zone),
        }
    }

    pub fn is_branch(&self) -> bool {
        matches!(self, ZoneNode::Branch(_))
    }
}

/// A zone is a portion of the screen.
/// See [make_zone] for details on how to create zones.
#[derive(Clone)]
pub struct Zone {
    id: u32,
}

impl Zone {
    pub fn id(&self) -> u32 {
        self.id
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn zone_splitting_works() {
        let mut tree = ZoneTree::new();
        assert!(tree.current_zone_mut().is_some_and(|z| z.id() == 0));
        tree.split_current();
        assert!(tree.root_node_mut().is_branch());
    }
}