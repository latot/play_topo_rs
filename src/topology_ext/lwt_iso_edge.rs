use crate::topology_ext::definitions::LWT_ELEMID;

struct LwtIsoEdge {
        edge_id: LWT_ELEMID,
        start_node: LWT_ELEMID,
        end_node: LWT_ELEMID,
        face_left: LWT_ELEMID,
        face_right: LWT_ELEMID,
        next_left: LWT_ELEMID,
        next_right: LWT_ELEMID,
}
