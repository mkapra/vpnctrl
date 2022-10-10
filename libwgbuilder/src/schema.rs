// @generated automatically by Diesel CLI.

diesel::table! {
    clients (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
        dns_server_id -> Int4,
        keepalive -> Int4,
        keypair_id -> Int4,
        vpn_ip_id -> Int4,
    }
}

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
    servers (id) {
        id -> Int4,
        name -> Text,
        description -> Nullable<Text>,
        forward_interface -> Nullable<Text>,
        keypair_id -> Int4,
        vpn_ip_id -> Int4,
        external_ip -> Varchar,
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

diesel::joinable!(clients -> dns_servers (dns_server_id));
diesel::joinable!(clients -> keypairs (keypair_id));
diesel::joinable!(clients -> vpn_ips (vpn_ip_id));
diesel::joinable!(servers -> keypairs (keypair_id));
diesel::joinable!(servers -> vpn_ips (vpn_ip_id));
diesel::joinable!(vpn_ips -> vpn_networks (vpn_network_id));

diesel::allow_tables_to_appear_in_same_query!(
    clients,
    dns_servers,
    keypairs,
    servers,
    vpn_ips,
    vpn_networks,
);
