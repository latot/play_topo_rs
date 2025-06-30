use crate::topology_ext::{definitions::LWT_ELEMID, lwt_topo_err_type_t::LwtTopoErrType};

pub struct LwtTopoErrT {
        pub err: LwtTopoErrType,
        /// Element affected
        pub elem1: LWT_ELEMID,
        /// Secondary element affected
        pub elem2: Option<LWT_ELEMID>,
}
