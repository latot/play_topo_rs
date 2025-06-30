enum TopoLoadFailMessageFlavor {
        SQL,
        AddPoint,
}

pub struct LwtBeDataT<'a> {
        pub r#char: &'a str,
        pub data_changed: bool,
        pub topo_load_fail_message_flavor: TopoLoadFailMessageFlavor,
}
