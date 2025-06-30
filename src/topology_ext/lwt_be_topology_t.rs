use super::error::Error;
use super::lwt_be_data_t::LwtBeDataT;
use super::topology::Topology;
use pgrx::prelude::*;
struct LwtBeTopologyT<'a> {
        be_data: &'a LwtBeDataT<'a>,
        name: &'a str,
        id: i32,
        srid: i32,
        precision: f64,
        hasz: bool,
        geometry_oid: pgrx::pg_sys::Oid,
}

const TOPOBYNAMEQUERY: &str = "
        SELECT id,srid,precision,null::geometry geometry
        FROM topology.topology WHERE name = $1::varchar
";

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
                        let geometry_oid = match ret.get(3) {
                                Ok(x) => x.unwrap(),
                                Err(err) => return Err(err.into()),
                        };
                        let topology: Topology = ret.try_into()?;
                        //let a: i32 = ret.get(0)?;
                        Ok(LwtBeTopologyT {
                                be_data,
                                name,
                                id: topology.id,
                                srid: topology.srid,
                                precision: topology.precision,
                                hasz: topology.hasz,
                                geometry_oid,
                        })
                })
        }
}
