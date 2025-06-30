use super::error::Error;
use pgrx::spi::SpiTupleTable;
use struct_iterable::Iterable;

/// Represent each row of topology.topology
#[derive(Iterable, Debug)]
pub struct Topology {
        pub id: i32,
        pub name: String,
        pub srid: i32,
        pub precision: f64,
        pub hasz: bool,
}

fn read_field<T: pgrx::IntoDatum + pgrx::FromDatum>(
        value: &SpiTupleTable,
        name: &str,
) -> Result<T, Error> {
        value.get_by_name::<T, &str>(name)
                .map_err(Error::SpiError)?
                .ok_or(Error::MissingField(String::from(name)))
}

impl TryFrom<SpiTupleTable<'_>> for Topology {
        type Error = Error;
        fn try_from(value: SpiTupleTable) -> Result<Self, Error> {
                let id = read_field(&value, "id")?;
                let name = read_field(&value, "name")?;
                let srid = read_field(&value, "srid")?;
                let precision = read_field(&value, "precision")?;
                let hasz = read_field(&value, "hasz")?;
                Ok(Topology {
                        id,
                        name,
                        srid,
                        precision,
                        hasz,
                })
        }
}
