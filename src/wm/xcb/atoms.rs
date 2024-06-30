xcb::atoms_struct! {
    #[derive(Copy, Clone, Debug)]
    pub(crate) struct Atoms {
        pub active_window => b"_NET_ACTIVE_WINDOW",
        pub number_of_desktops => b"_NET_NUMBER_OF_DESKTOPS",
        pub desktop_names => b"_NET_DESKTOP_NAMES",
        pub current_desktop => b"_NET_CURRENT_DESKTOP",
        pub client_list => b"_NET_CLIENT_LIST",
        pub shadow => b"_COMPTON_SHADOW",
        pub utf8_string => b"ATOM_UTF8_STRING",
        // pub strut_partial => b"_NET_WM_STRUT_PARTIAL",
        // pub strut => b"_NET_WM_STRUT",
    }
}
