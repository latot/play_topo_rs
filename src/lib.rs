use pgrx::*;
use std::ffi::CString;

pgrx::pg_module_magic!();

#[pg_extern]
fn create_topology(
    topology_name: &str,
    srid: i32,
    tolerance: Option<f64>,
    hasz: Option<bool>,
) -> Result<i32, String> {
    if topology_name.is_empty() {
        return Err("Topology name cannot be empty".to_string());
    }

    let _tol = tolerance.unwrap_or(0.0);
    let _z = hasz.unwrap_or(false);

    Spi::run("SELECT topology.CreateTopology(...)")
        .map_err(|e| format!("SPI error: {:?}", e))?;

    Ok(1) // Simulación: retorna ID ficticio
}

#[pg_extern]
fn drop_topology(topology_name: &str) -> Result<(), String> {
    if topology_name.is_empty() {
        return Err("Topology name cannot be empty".to_string());
    }

    Spi::run(&format!("SELECT topology.DropTopology('{}')", topology_name))
        .map_err(|e| format!("SPI error: {:?}", e))?;

    Ok(())
}

extension_sql!(
    r#"
    CREATE SCHEMA IF NOT EXISTS topology;
    "#,
    name = "create_topology_schema",
    requires = [],
);