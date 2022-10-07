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

diesel::allow_tables_to_appear_in_same_query!(dns_servers, keypairs,);
