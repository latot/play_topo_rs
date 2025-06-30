pub enum LwtTopoErrType {
        EdgeCrossesNode,
        EdgeInvalid,
        EdgeNotSimple,
        EdgeCrossesEdge,
        EdgeStartNodeMismatch,
        EdgeEndNodeMismatch,
        FaceWithoutEdges,
        FaceHasNoRings,
        FaceOverlapsFace,
        FaceWithinFace,
}
