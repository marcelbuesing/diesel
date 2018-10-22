use byteorder::{NetworkEndian, ReadBytesExt, WriteBytesExt};
use std::error::Error;
use std::io::prelude::*;

use deserialize::{self, FromSql};
use pg::{Pg, PgValue};
use serialize::{self, IsNull, Output, ToSql};
use sql_types;

impl FromSql<sql_types::SmallInt, Pg> for i16 {
    fn from_sql(value: Option<PgValue>) -> deserialize::Result<Self> {
        let value = not_none!(value);
        let mut bytes = value.bytes();
        debug_assert!(
            bytes.len() <= 2,
            "Received more than 2 bytes decoding i16. \
             Was an Integer expression accidentally identified as SmallInt?"
        );
        debug_assert!(
            bytes.len() >= 2,
            "Received fewer than 2 bytes decoding i16. \
             Was an expression of a different type accidentally identified \
             as SmallInt?"
        );
        bytes
            .read_i16::<NetworkEndian>()
            .map_err(|e| Box::new(e) as Box<Error + Send + Sync>)
    }
}

impl FromSql<sql_types::Integer, Pg> for i32 {
    fn from_sql(value: Option<PgValue>) -> deserialize::Result<Self> {
        let value = not_none!(value);
        let mut bytes = value.bytes();
        debug_assert!(
            bytes.len() <= 4,
            "Received more than 4 bytes decoding i32. \
             Was a BigInteger expression accidentally identified as Integer?"
        );
        debug_assert!(
            bytes.len() >= 4,
            "Received fewer than 4 bytes decoding i32. \
             Was a SmallInteger expression accidentally identified as Integer?"
        );
        bytes
            .read_i32::<NetworkEndian>()
            .map_err(|e| Box::new(e) as Box<Error + Send + Sync>)
    }
}

impl FromSql<sql_types::BigInt, Pg> for i64 {
    fn from_sql(value: Option<PgValue>) -> deserialize::Result<Self> {
        let value = not_none!(value);
        let mut bytes = value.bytes();
        debug_assert!(
            bytes.len() <= 8,
            "Received more than 8 bytes decoding i64. \
             Was an expression of a different type misidentified as BigInteger?"
        );
        debug_assert!(
            bytes.len() >= 8,
            "Received fewer than 8 bytes decoding i64. \
             Was an Integer expression misidentified as BigInteger?"
        );
        bytes
            .read_i64::<NetworkEndian>()
            .map_err(|e| Box::new(e) as Box<Error + Send + Sync>)
    }
}

impl FromSql<sql_types::Oid, Pg> for u32 {
    fn from_sql(value: Option<PgValue>) -> deserialize::Result<Self> {
        let value = not_none!(value);
        let mut bytes = value.bytes();
        bytes.read_u32::<NetworkEndian>().map_err(|e| e.into())
    }
}

impl ToSql<sql_types::Oid, Pg> for u32 {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        out.write_u32::<NetworkEndian>(*self)
            .map(|_| IsNull::No)
            .map_err(|e| e.into())
    }
}

#[test]
fn i16_to_sql() {
    let mut bytes = Output::test();
    ToSql::<sql_types::SmallInt, Pg>::to_sql(&1i16, &mut bytes).unwrap();
    ToSql::<sql_types::SmallInt, Pg>::to_sql(&0i16, &mut bytes).unwrap();
    ToSql::<sql_types::SmallInt, Pg>::to_sql(&-1i16, &mut bytes).unwrap();
    assert_eq!(bytes, vec![0, 1, 0, 0, 255, 255]);
}

#[test]
fn i32_to_sql() {
    let mut bytes = Output::test();
    ToSql::<sql_types::Integer, Pg>::to_sql(&1i32, &mut bytes).unwrap();
    ToSql::<sql_types::Integer, Pg>::to_sql(&0i32, &mut bytes).unwrap();
    ToSql::<sql_types::Integer, Pg>::to_sql(&-1i32, &mut bytes).unwrap();
    assert_eq!(bytes, vec![0, 0, 0, 1, 0, 0, 0, 0, 255, 255, 255, 255]);
}

#[test]
fn i64_to_sql() {
    let mut bytes = Output::test();
    ToSql::<sql_types::BigInt, Pg>::to_sql(&1i64, &mut bytes).unwrap();
    ToSql::<sql_types::BigInt, Pg>::to_sql(&0i64, &mut bytes).unwrap();
    ToSql::<sql_types::BigInt, Pg>::to_sql(&-1i64, &mut bytes).unwrap();
    assert_eq!(
        bytes,
        vec![
            0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255,
        ]
    );
}
