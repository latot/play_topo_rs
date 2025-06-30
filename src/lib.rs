use pgrx::prelude::*;
mod topology_ext;
use once_cell::sync::Lazy;
use pgrx::shmem;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::Mutex;

#[pg_extern]
fn hello_my_extension() -> f64 {
        1.0
}

use pgrx::*;
use std::sync::atomic::{AtomicUsize, Ordering};

pgrx::pg_module_magic!();

// Tamaño del segmento compartido
const SHMEM_SIZE: usize = std::mem::size_of::<AtomicUsize>();
static mut FOO_PTR: *mut AtomicUsize = std::ptr::null_mut();

#[pg_guard]
unsafe extern "C-unwind" fn init_foo_shmem() {
        let found = &mut false;
        let ptr = shmem::shmem_init("shared_foo", SHMEM_SIZE, found) as *mut AtomicUsize;

        if !*found {
                // Inicializar si es la primera vez
                ptr.write(AtomicUsize::new(0));
        }

        FOO_PTR = ptr;
}

#[pg_guard]
pub extern "C-unwind" fn _PG_init() {
        unsafe {
                init_foo_shmem();
        }
}

#[pg_extern]
fn add() -> i32 {
        unsafe {
                if FOO_PTR.is_null() {
                        init_foo_shmem();
                }
                (*FOO_PTR).fetch_add(1, Ordering::SeqCst) as i32 + 1
        }
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
        use pgrx::prelude::*;

        #[pg_test]
        fn test_hello_my_extension() {
                assert_eq!(2.0, crate::hello_my_extension());
        }
}

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
        pub fn setup(_options: Vec<&str>) {
                // perform one-off initialization when the pg_test framework starts
        }

        #[must_use]
        pub fn postgresql_conf_options() -> Vec<&'static str> {
                // return any postgresql.conf settings that are required for your tests
                vec![]
        }
}
