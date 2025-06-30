use crate::topology_ext::definitions::LWT_ELEMID;

pub struct FaceEdgeState<'a> {
        elems: &'a LWT_ELEMID,
        nelems: usize,
        curr: usize,
}
