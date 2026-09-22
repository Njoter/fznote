mod list;
mod add;
mod delete;
mod switch;
mod rename;

pub use list::execute as list;
pub use add::execute as new;
pub use delete::execute as delete;
pub use switch::execute as switch;
pub use rename::execute as rename;
