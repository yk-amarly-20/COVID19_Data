table! {
    all_prefectures (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    num_infected (id) {
        id -> Int4,
        date -> Varchar,
        prefecture_id -> Int4,
        num_of_infected -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    all_prefectures,
    num_infected,
);
