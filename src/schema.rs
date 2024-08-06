// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    pet_tag (pet_id, tag_id) {
        pet_id -> Int4,
        tag_id -> Int4,
    }
}

diesel::table! {
    pets (id) {
        id -> Int4,
        name -> Varchar,
        category_id -> Int4,
        #[max_length = 100]
        status -> Varchar,
    }
}

diesel::table! {
    tags (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::joinable!(pet_tag -> pets (pet_id));
diesel::joinable!(pet_tag -> tags (tag_id));
diesel::joinable!(pets -> categories (category_id));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    pet_tag,
    pets,
    tags,
);
