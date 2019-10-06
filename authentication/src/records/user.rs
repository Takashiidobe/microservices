use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use super::super::schema::users;

#[table_name = "users"]
#[derive(Serialize, Deserialize, Identifiable, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String
}

#[table_name = "users"]
#[derive(Serialize, Deserialize, Insertable)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str
}

#[table_name = "users"]
#[derive(Serialize, Deserialize, AsChangeset)]
pub struct UpdateUser<'a> {
    pub password: &'a str
}

impl User {
    pub fn create(user: NewUser, connection: &MysqlConnection) {
        diesel::insert_into(users::table)
            .values(&user)
            .execute(connection)
            .expect("Error creating new user");
    }

    pub fn read(id: i32, connection: &MysqlConnection) -> User {
        users::table.find(id).first(connection).expect("Error finding user")
    }

    pub fn read_all(connection: &MysqlConnection) -> Vec<User> {
        users::table
            .order(users::id.asc())
            .load::<User>(connection)
            .unwrap()
    }

    pub fn update(id: i32, user: UpdateUser, connection: &MysqlConnection) -> bool {
        diesel::update(users::table.find(id))
            .set(&user)
            .execute(connection)
            .is_ok()
    }

    pub fn delete(id: i32, connection: &MysqlConnection) -> bool {
        diesel::delete(users::table.find(id))
            .execute(connection)
            .is_ok()
    }
}