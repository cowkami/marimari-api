// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Bpchar,
        name -> Nullable<Text>,
    }
}
