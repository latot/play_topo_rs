use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

use crate::topology_ext::lwt_be_callbacks_t::LwtBeCallbacksT;
use crate::topology_ext::lwt_be_data_t::LwtBeDataT;

static POSTGIS_IFACE: Lazy<Arc<Mutex<Foo>>> = Lazy::new(|| Arc::new(Mutex::new(Foo { hi: 2.0 })));

static PI: Lazy<Arc<Foo>> = Lazy::new(|| Arc::new(Foo { hi: 2.0 }));

struct Foo {
        hi: f64,
}

fn foo() {
        POSTGIS_IFACE.lock().unwrap().hi = 2.0;
}

pub struct LwtBeIface<'a> {
        lwt_be_data: LwtBeDataT<'a>,
        lwt_be_callbacks: LwtBeCallbacksT,
}
