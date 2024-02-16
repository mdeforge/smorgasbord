use crate::account::Account;

pub trait Menu {
    fn prompt(&self, account: &mut Account) -> Option<Box<dyn Menu>>;
}