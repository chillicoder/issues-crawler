table! {
    issues (id) {
        id -> Int8,
        author -> Varchar,
        title -> Varchar,
        url -> Varchar,
    }
}

table! {
    projects (id) {
        id -> Int8,
        name -> Varchar,
        url -> Varchar,
        watchers -> Nullable<Int8>,
        forks -> Nullable<Int8>,
        stars -> Nullable<Int8>,
    }
}

allow_tables_to_appear_in_same_query!(
    issues,
    projects,
);
