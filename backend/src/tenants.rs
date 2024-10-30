use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use crate::establish_connection;
use crate::models::Tenant;
use crate::schema::tenants;

pub(crate) fn create_tenant(new_tenant: Tenant) {
    let connection = &mut establish_connection();

    diesel::insert_into(tenants::table)
        .values(&new_tenant)
        .execute(connection)
        .expect("Error saving new tenant");
}

pub(crate) fn get_all_tenants() -> Vec<Tenant> {
    use crate::schema::tenants::dsl::tenants;
    let connection = &mut establish_connection();
    tenants
        .select(Tenant::as_select())
        .load(connection)
        .expect("error loading tenants")
}








