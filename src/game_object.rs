
// public interface for GameObject, only an id so that the scene can know which
// components to retrieve
pub struct GameObject {
    pub is_active: bool,
    pub is_loaded: bool,
    // using an int might not be the best idea
    pub(in crate) id: i64,
}


impl GameObject {
    pub fn new() -> Self {
        GameObject {
            is_active: true,
            is_loaded: false,
            // negative int for objects which haven't been added to a scene
            id: -1
        }
    }

    pub fn is_active(&self) -> bool {
        return self.is_active;
    }

    // would like this function to only be accessible from scene
    pub fn get_id(&self) -> i64 {
        return self.id;
    }

    pub fn set_id(&mut self, new_id: i64) -> () {
        self.id = new_id;
    }
}

