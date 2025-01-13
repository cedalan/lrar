use chrono::{NaiveDate, Utc};
use diesel::{PgConnection, RunQueryDsl};
use crate::models::{Burn, BurnDto};

pub fn get_weekly_chore() -> Vec<String> {
    let all_chores: Vec<String> = vec![
        "Clean the kitchen".to_string(),
        "Clean the toilet".to_string(),
        "Clean the bathroom".to_string(),
        "Clean the hallway".to_string(),
        "Clean the living room".to_string(),
        "Take out all trash".to_string(),
    ];

    //Arbritrary start date
    let start_date = NaiveDate::from_ymd_opt(2024, 12, 9).expect("Invalid date");
    let current_date = Utc::now().naive_utc().date();

    let weeks_since_start = (current_date.signed_duration_since(start_date).num_weeks()) as usize;

    let num_chores = all_chores.len();

    //Calculates the "rotation index"
    let chore_index = weeks_since_start % num_chores;

    // Rotate the list to the right by 'chore_index'
    let rotated_chores: Vec<_> = all_chores
        .iter()
        .cycle()
        .skip(num_chores - chore_index) // Shift to the right by the calculated index
        .take(num_chores)
        .cloned()
        .collect();

    return rotated_chores;
}

pub fn insert_new_burn(
    conn: &mut PgConnection,
    new_burn: BurnDto,
) -> diesel::QueryResult<Burn> {
    use crate::schema::burn::dsl::*;

    let query_result = diesel::insert_into(burn)
        .values(new_burn)
        .get_result(conn)
        .expect("Error inserting burn");

    Ok(query_result)
}