no_arg_sql_function!(last_insert_id, diesel::types::Bigint);

pub mod schema;

mod club;
pub use club::*;

mod asset;
pub use asset::*;

mod user;
pub use user::*;
