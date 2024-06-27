xcb::atoms_struct! {
    #[derive(Copy, Clone, Debug)]
    pub(crate) struct Atoms {
        pub active_window => b"_NET_ACTIVE_WINDOW",
        pub shadow => b"_COMPTON_SHADOW",
        pub utf8_string => b"ATOM_UTF8_STRING",
        // pub strut_partial => b"_NET_WM_STRUT_PARTIAL",
        // pub strut => b"_NET_WM_STRUT",
    }
}
