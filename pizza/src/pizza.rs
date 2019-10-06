use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use super::schema::pizzas;

#[table_name = "pizzas"]
#[derive(Serialize, Deserialize, Identifiable, Queryable)]
pub struct Pizza {
  pub id: i32,
      pub name: String,
      pub price: f64
}

#[table_name = "pizzas"]
#[derive(Serialize, Deserialize, Insertable)]
pub struct NewPizza<'a> {
  pub name: &'a str,
      pub price: f64
}

#[table_name = "pizzas"]
#[derive(Serialize, Deserialize, AsChangeset)]
pub struct IgnoreNoneFieldsUpdatePizza<'a> {
  pub name: &'a str,
}

impl Pizza {
  pub fn create(pizza: NewPizza, connection: &MysqlConnection) -> Pizza {
    diesel::insert_into(pizzas::table)
      .values(&pizza)
      .execute(connection)
      .expect("Error creating new pizza");

    pizzas::table.order(pizzas::id.desc()).first(connection).unwrap()
  }

  pub fn read(connection: &MysqlConnection) -> Vec<Pizza> {
    pizzas::table.order(pizzas::id.asc()).load::<Pizza>(connection).unwrap()
  }

  pub fn update(id: i32, pizza: IgnoreNoneFieldsUpdatePizza, connection: &MysqlConnection) -> bool {
    diesel::update(pizzas::table.find(id)).set(&pizza).execute(connection).is_ok()
  }

  pub fn delete(id: i32, connection: &MysqlConnection) -> bool {
    diesel::delete(pizzas::table.find(id)).execute(connection).is_ok()
  }
}
