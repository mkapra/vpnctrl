// @generated automatically by Diesel CLI.

diesel::table! {
    keypairs (id) {
        id -> Int4,
        private_key -> Text,
        public_key -> Text,
    }
}
