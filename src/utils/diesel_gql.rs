use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, Value};
use diesel::serialize::ToSql;
use diesel::{deserialize::FromSql, sql_types::BigInt};

#[derive(Debug, Clone, Copy, FromSqlRow, AsExpression, PartialEq, Eq, Hash)]
#[sql_type = "BigInt"]
pub struct StringNumber(pub i64);

#[Scalar]
impl ScalarType for StringNumber {
    fn parse(value: Value) -> InputValueResult<Self> {
        match value {
            Value::String(s) => {
                let n = i64::from_str_radix(&s, 10)
                    .map_err(|err| InputValueError::custom(err.to_string()))?;
                Ok(StringNumber(n))
            }
            _ => Err(InputValueError::expected_type(value)),
        }
    }

    fn is_valid(value: &Value) -> bool {
        matches!(value, Value::String(_))
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.to_string())
    }
}

impl<DB: diesel::backend::Backend> ToSql<BigInt, DB> for StringNumber {
    fn to_sql<W: std::io::Write>(
        &self,
        out: &mut diesel::serialize::Output<W, DB>,
    ) -> Result<diesel::serialize::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        ToSql::<BigInt, DB>::to_sql(&(*self as StringNumber).0, out)
    }
}

impl<DB: diesel::backend::Backend<RawValue = [u8]>> FromSql<BigInt, DB> for StringNumber {
    fn from_sql(
        bytes: Option<&DB::RawValue>,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        Ok(StringNumber(<i64 as FromSql<BigInt, DB>>::from_sql(bytes)?))
    }
}
