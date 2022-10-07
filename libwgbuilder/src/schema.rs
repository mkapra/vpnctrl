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
    vpn_networks (id) {
        id -> Int4,
        network -> Varchar,
        subnetmask -> Int4,
        interface -> Int4,
        port -> Int4,
    }
}

diesel::allow_tables_to_appear_in_same_query!(dns_servers, keypairs, vpn_networks,);
