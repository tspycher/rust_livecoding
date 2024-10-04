// @generated automatically by Diesel CLI.

diesel::table! {
    aircraft (id) {
        id -> Nullable<Integer>,
        display_name -> Text,
        registration -> Text,
    }
}
