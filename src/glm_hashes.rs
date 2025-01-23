use std::hash::{Hash, Hasher};


pub struct IVec2Hashable(pub glm::IVec2);
pub struct IVec3Hashable(pub glm::IVec3);


impl Hash for IVec2Hashable {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.x.hash(state);
        self.0.y.hash(state);
    }
}

impl Hash for IVec3Hashable {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.x.hash(state);
        self.0.y.hash(state);
        self.0.z.hash(state);
    }
}




impl PartialEq for IVec2Hashable {
    fn eq(&self, other: &Self) -> bool {
        return self.0.eq(&other.0);
    }
}
impl Eq for IVec2Hashable {}

impl PartialEq for IVec3Hashable {
    fn eq(&self, other: &Self) -> bool {
        return self.0.eq(&other.0);
    }
}
impl Eq for IVec3Hashable {}


impl From<glm::IVec2> for IVec2Hashable {
    fn from(item: glm::IVec2) -> Self {
        IVec2Hashable(item)
    }
}

impl From<glm::IVec3> for IVec3Hashable {
    fn from(item: glm::IVec3) -> Self {
        IVec3Hashable(item)
    }
}

