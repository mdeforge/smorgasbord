use crate::user::User;

pub trait Menu {
    fn prompt(&self, user: &mut User) -> Option<Box<dyn Menu>>;
}