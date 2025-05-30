use chrono::{NaiveDate, Utc};
use diesel::{PgConnection, RunQueryDsl};
use crate::models::{Burn, BurnDto, NewTenant, Note, NewNote};
use crate::models::Tenant;

pub fn get_weekly_chore(n_tenants: usize) -> Vec<String> {
    let mut all_chores: Vec<String> = vec![
        "Clean the kitchen".to_string(),
        "Clean the toilet".to_string(),
        "Clean the bathroom".to_string(),
        "Clean the hallway".to_string(),
        "Clean the living room".to_string(),
        "Take out all trash".to_string(),
    ];

    let n_missing:usize; //Number of missing chores

    if n_tenants > all_chores.len() {
        n_missing = n_tenants - all_chores.len();
    } else {
        n_missing = 0;
    }

    if n_missing > 0 {
        for _ in 0..(n_missing) {
            all_chores.push("Nothing".to_string());
        }
    }

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

pub fn insert_new_note(
    conn: &mut PgConnection,
    new_note: NewNote,
) -> diesel::QueryResult<Note> {
    use crate::schema::notes::dsl::*;

    diesel::insert_into(notes)
        .values(new_note)
        .get_result(conn)
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

pub fn insert_new_tenant(
    conn: &mut PgConnection,
    new_tenant: NewTenant,
) -> diesel::QueryResult<Tenant> {
    use crate::schema::tenants::dsl::*;

    println!("Inserting new tenant: {:?}", new_tenant); // Debug output

    let query_result = diesel::insert_into(tenants)
        .values(new_tenant)
        .get_result(conn);

    match &query_result {
        Ok(tenant) => println!("Successfully inserted tenant: {:?}", tenant),
        Err(e) => eprintln!("Error inserting tenant: {:?}", e),
    }

    query_result
}

pub fn increase_dishwasher_count(
    conn: &mut PgConnection,
    responsible_tenant_id: i32,
) -> diesel::QueryResult<usize> {
    use diesel::prelude::*;
    use crate::schema::tenants::dsl::*;

    diesel::update(tenants.filter(id.eq(responsible_tenant_id)))
    .set(
        dishwasher_count.eq(dishwasher_count + 1)
    )
    .execute(conn)
}

pub fn decrease_burn_count(
    conn: &mut PgConnection,
    tenant_id: i32,
    decrease: i32,
) -> diesel::QueryResult<()> {
    use diesel::prelude::*;
    use crate::schema::tenants::dsl::*;

    let result = diesel::update(tenants.filter(id.eq(tenant_id)))
        .set(current_burn_count.eq(current_burn_count - decrease))
        .execute(conn);

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn give_burn_to_tenant(
    conn: &mut PgConnection,
    burned_tenant_id: i32,
) -> diesel::QueryResult<()> {
    use diesel::prelude::*;
    use crate::schema::tenants::dsl::*;
    diesel::update(tenants.filter(id.eq(burned_tenant_id)))
    .set((
        burn_count.eq(burn_count + 1),
        current_burn_count.eq(current_burn_count + 1),
    ))
    .execute(conn)?;

    Ok(())
}

pub async fn id_to_name(tenant_id: i32, tenants: &[Tenant]) -> String {
    for tenant in tenants {
        if tenant_id == tenant.id {
            println!("Input tenant id: {}, Found tenant id: {}", tenant_id, tenant.id);
            return tenant.name.clone().to_string();
        }
    }
    return "No name found for that user".to_string();
}
