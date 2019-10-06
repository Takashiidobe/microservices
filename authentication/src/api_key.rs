use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use super::schema::api_keys;

#[table_name = "api_keys"]
#[derive(Serialize, Deserialize, Identifiable, Queryable)]
pub struct ApiKey {
    pub id: i32,
    pub public_id: String,
    pub permission: String
}

#[table_name = "api_keys"]
#[derive(Serialize, Deserialize, Insertable)]
pub struct NewApiKey<'a> {
    pub public_id: &'a str,
    pub permission: &'a str
}

#[table_name = "api_keys"]
#[derive(Serialize, Deserialize, AsChangeset)]
pub struct UpdateApiKey<'a> {
    pub permission: &'a str
}

impl ApiKey {
    pub fn create(api_key: NewApiKey, connection: &MysqlConnection) -> bool {
        diesel::insert_into(api_keys::table)
            .values(&api_key)
            .execute(connection)
            .is_ok()
    }

    pub fn read(id: i32, connection: &MysqlConnection) -> ApiKey {
        api_keys::table.find(id).first(connection).expect("Error finding user")
    }

    pub fn read_all(connection: &MysqlConnection) -> Vec<ApiKey> {
        api_keys::table
            .order(api_keys::id.asc())
            .load::<ApiKey>(connection)
            .unwrap()
    }

    pub fn update(id: i32, api_key: UpdateApiKey, connection: &MysqlConnection) -> bool {
        diesel::update(api_keys::table.find(id))
            .set(&api_key)
            .execute(connection)
            .is_ok()
    }

    pub fn delete(id: i32, connection: &MysqlConnection) -> bool {
        diesel::delete(api_keys::table.find(id))
            .execute(connection)
            .is_ok()
    }
}