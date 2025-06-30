use crate::topology_ext::definitions::{LWT_ELEMID, LWT_POINT};

struct LwtIsoNode<'a> {
        node_id: LWT_ELEMID,
        // None if is isolated
        containing_face: Option<LWT_ELEMID>,
        geom: &'a LWT_POINT,
}
