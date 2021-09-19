table! {
    movies (id) {
        id -> Varchar,
        title -> Varchar,
        genre -> Varchar,
        plot -> Varchar,
        poster -> Varchar,
        runtime -> Varchar,
    }
}

table! {
    users (id) {
        id -> Varchar,
        username -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(movies, users,);
