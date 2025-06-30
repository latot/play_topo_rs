use crate::topology_ext::definitions::{GBOX, LWT_ELEMID};

pub struct LwtIsoFace<'a> {
        face_id: LWT_ELEMID,
        mbr: &'a GBOX,
}
