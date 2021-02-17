#[macro_export]
macro_rules! define_enum {
    (
        $(#[$meta:meta])*
        pub enum $name:ident { $($variant:ident = $val:expr,)* }
    ) => {
        use diesel::sql_types::Integer;
        use diesel::deserialize::FromSql;
        use diesel::serialize::ToSql;

        // 元の enum を必要な derive とともに定義
        $(#[$meta])*
        #[derive(FromSqlRow, AsExpression)]
        #[sql_type = "Integer"]
        pub enum $name {
            $($variant = $val,)*
        }

        // `ToSql`を定義
        impl<DB: diesel::backend::Backend> ToSql<Integer, DB> for $name {
            fn to_sql<W: std::io::Write>(
                &self,
                out: &mut diesel::serialize::Output<W, DB>,
            ) -> Result<diesel::serialize::IsNull, Box<dyn std::error::Error + Send + Sync>> {
                ToSql::<Integer, DB>::to_sql(&(*self as i32), out)
            }
        }

        // `FromSql`を定義
        impl<DB: diesel::backend::Backend> FromSql<Integer, DB> for $name
        where
            i32: FromSql<Integer, DB>,
        {
            fn from_sql(
                bytes: Option<&DB::RawValue>,
            ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
                use self::$name::*;

                match <i32 as FromSql<Integer, DB>>::from_sql(bytes)? {
                    $($val => Ok($variant),)*
                    s => Err(format!("invalid {} value: {}", stringify!($name), s).into()),
                }
            }
        }
    }
}
