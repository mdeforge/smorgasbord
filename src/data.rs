use crate::user::User;

pub mod data {
    use crate::user::User;

    pub static mut USER_DATA: User = User::default();
}