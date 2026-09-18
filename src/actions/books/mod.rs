mod list;
mod add;
mod delete;
mod switch;

pub use list::execute as list;
pub use add::execute as new;
pub use delete::execute as delete;
pub use switch::execute as switch;
