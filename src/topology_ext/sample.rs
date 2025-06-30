use std::f32::MAX;
use std::fmt::Display;

use pgrx::datum::DatumWithOid;
use pgrx::prelude::*;
use pgrx::spi::{SpiError, SpiTupleTable};
use pgrx::*;

const MAXERRLEN: usize = 256;

enum TopoLoadFailMessageFlavor {
        SQL,
        AddPoint,
}

struct LwtBeDataT<'a> {
        r#char: &'a str,
        data_changed: bool,
        topo_load_fail_message_flavor: TopoLoadFailMessageFlavor,
}

enum TODO {}

struct LwtBeTopologyT<'a> {
        be_data: &'a LwtBeDataT<'a>,
        name: &'a str,
        id: i32,
        srid: i32,
        precision: f64,
        has_z: bool,
        geometry_oid: pgrx::pg_sys::Oid,
}

const TOPOBYNAMEQUERY: &str = "
        SELECT id,srid,precision,null::geometry geometry
        FROM topology.topology WHERE name = $1::varchar
";

#[derive(Debug, PartialEq)]
enum Error {
        SpiError(SpiError),
        NotOneRow,
        NoRows,
}

impl From<SpiError> for Error {
        fn from(value: SpiError) -> Self {
                Error::SpiError(value)
        }
}

impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                        f,
                        "{}",
                        match self {
                                Error::NoRows => String::from("No rows found"),
                                Error::NotOneRow => String::from("More than one row found"),
                                Error::SpiError(x) => format!("{:?}", x),
                        }
                )
        }
}

impl std::error::Error for Error {}

struct Foo {}

impl LwtBeTopologyT<'_> {
        fn try_load_from_name<'a>(
                be_data: &'a LwtBeDataT<'a>,
                name: &'a str,
        ) -> Result<LwtBeTopologyT<'a>, Error> {
                Spi::connect(|client: &spi::SpiClient<'_>| {
                        let ret = match client.select(TOPOBYNAMEQUERY, None, &[name.into()]) {
                                Ok(x) => x,
                                Err(x) => return Err(x.into()),
                        };
                        let ret = match ret.len() {
                                0 => return Err(Error::NoRows),
                                1 => ret.first(),
                                _ => return Err(Error::NotOneRow),
                        };
                        //let a: i32 = ret.get(0)?;
                        Ok(LwtBeTopologyT {
                                be_data,
                                name,
                                id: match ret.get(0) {
                                        Ok(x) => x.unwrap(),
                                        Err(err) => return Err(err.into()),
                                },
                                srid: match ret.get(1) {
                                        Ok(x) => x.unwrap(),
                                        Err(err) => return Err(err.into()),
                                },
                                precision: match ret.get(2) {
                                        Ok(x) => x.unwrap(),
                                        Err(err) => return Err(err.into()),
                                },
                                has_z: false,
                                geometry_oid: match ret.get(3) {
                                        Ok(x) => x.unwrap(),
                                        Err(err) => return Err(err.into()),
                                },
                        })
                })
        }
}
