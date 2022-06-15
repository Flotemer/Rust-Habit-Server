table! {
    check_ins (id) {
        id -> Int8,
        habit_id -> Int8,
        date -> Date,
        count -> Nullable<Int2>,
    }
}

table! {
    habits (id) {
        id -> Int8,
        user_id -> Int8,
        name -> Varchar,
        frequency -> Nullable<Varchar>,
        days -> Nullable<Array<Int8>>,
        check_ins_per_freq -> Int8,
        start_date -> Date,
    }
}

table! {
    users (id) {
        id -> Int8,
        first_name -> Varchar,
        last_name -> Varchar,
    }
}

joinable!(check_ins -> habits (id));
joinable!(habits -> users (id));

allow_tables_to_appear_in_same_query!(
    check_ins,
    habits,
    users,
);
