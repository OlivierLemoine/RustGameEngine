use super::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Object {
    pub transform: Option<Transform>,
    pub sprite: Option<Sprite>,
    pub rigidbody: Option<Rigidbody>,
    pub script: Option<Script>,
    #[serde(skip)]
    pub parent: Option<Weak<RefCell<Object>>>,
    #[serde(skip)]
    pub children: Vec<Rc<RefCell<Object>>>,
}
impl Object {
    pub fn global_transform(&self) -> Option<Transform> {
        let mut res = self.transform.clone()?;

        if let Some(parent) = &self.parent {
            let parent_global_transform = parent.upgrade()?.try_borrow().ok()?.transform.clone()?;
            res.position += parent_global_transform.position;
            res.scale *= parent_global_transform.scale;
        }

        Some(res)
    }
}
