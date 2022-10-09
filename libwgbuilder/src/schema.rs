// @generated automatically by Diesel CLI.

diesel::table! {
    dns_servers (id) {
        id -> Int4,
        name -> Text,
        ip -> Text,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    keypairs (id) {
        id -> Int4,
        private_key -> Text,
        public_key -> Text,
    }
}

diesel::table! {
    vpn_ips (id) {
        id -> Int4,
        address -> Varchar,
        vpn_network_id -> Int4,
    }
}

diesel::table! {
    vpn_networks (id) {
        id -> Int4,
        network -> Varchar,
        subnetmask -> Int4,
        interface -> Int4,
        port -> Int4,
    }
}

diesel::joinable!(vpn_ips -> vpn_networks (vpn_network_id));

diesel::allow_tables_to_appear_in_same_query!(dns_servers, keypairs, vpn_ips, vpn_networks,);
