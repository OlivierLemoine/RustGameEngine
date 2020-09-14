use super::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Object {
    #[serde(default)]
    pub ui: bool,
    pub transform: Option<Transform>,
    pub sprite: Option<Sprite>,
    pub rigidbody: Option<Rigidbody>,
    pub script: Option<Script>,
    #[serde(skip)]
    pub parent: Option<Weak<RefCell<Object>>>,
    #[serde(skip)]
    pub children: Vec<Rc<RefCell<Object>>>,
    #[serde(skip)]
    pub custom: Option<Box<dyn std::any::Any>>,
}
impl Object {
    pub fn global_transform(&self) -> Result<Transform, Box<dyn std::error::Error>> {
        let mut res = self
            .transform
            .clone()
            .ok_or("Object does not have a transform")?;

        if self.parent.has_transform() {
            if let Some(parent) = &self.parent {
                let parent_global_transform = parent
                    .upgrade()
                    .ok_or("Could not upgrade to parent")?
                    .try_borrow()?
                    .global_transform()?;
                res.position = (res.position * parent_global_transform.scale)
                    + parent_global_transform.position;
                res.scale *= parent_global_transform.scale;
            }
        }

        Ok(res)
    }
}
pub trait ComponentChecker {
    fn has_transform(&self) -> bool;
    fn has_sprite(&self) -> bool;
    fn has_rigidbody(&self) -> bool;
    fn has_script(&self) -> bool;
    fn has_parent(&self) -> bool;
}
impl ComponentChecker for Object {
    fn has_transform(&self) -> bool {
        self.transform.is_some()
    }
    fn has_sprite(&self) -> bool {
        self.sprite.is_some()
    }
    fn has_rigidbody(&self) -> bool {
        self.rigidbody.is_some()
    }
    fn has_script(&self) -> bool {
        self.script.is_some()
    }
    fn has_parent(&self) -> bool {
        self.parent.is_some()
    }
}

macro_rules! impl_component_checker_opt_weak_refcell_obj {
    ($($f:ident),*) => {
        impl ComponentChecker for Option<Weak<RefCell<Object>>> {
            $(
                fn $f(&self) -> bool {
                    self.as_ref()
                    .map(|parent| {
                        parent
                            .upgrade()
                            .map(|v| v.try_borrow().ok().map(|v| v.$f()))
                    })
                    .flatten()
                    .flatten()
                    .unwrap_or(false)
                }
            )*
        }
    };
}
macro_rules! impl_component_checker_rc_refcell_obj {
    ($($f:ident),*) => {
        impl ComponentChecker for Rc<RefCell<Object>> {
            $(
                fn $f(&self) -> bool {
                    self.try_borrow()
                        .map(|v| v.$f())
                        .unwrap_or(false)
                }
            )*
        }
    };
}
macro_rules! impl_all_component_checker {
    ($($f:ident),*) => {
        impl_component_checker_opt_weak_refcell_obj!{$($f),*}
        impl_component_checker_rc_refcell_obj!{$($f),*}
    };
}

impl_all_component_checker! {has_transform, has_sprite, has_rigidbody, has_script, has_parent}
