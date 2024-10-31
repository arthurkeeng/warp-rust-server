use rusqlite::Connection;

pub fn get_db_conn() -> Connection{
    let conn = Connection::open("restaurant.db").expect("failed to open sqllite connection");
    conn

}

pub fn initialize_db(){
    println!("initializing the database...");
    let conn = Connection::open("restaurant.db").expect("failed to open sqllite connection");
    conn.execute("PRAGMA foreign_keys;", []).expect("failed to enable foreign key support");
    println!("create Table table");
    create_table_table_if_not_exist(&conn);
    println!("create Menu table");
    create_table_table_if_not_exist(&conn);
    println!("create Order table");
    create_table_table_if_not_exist(&conn);
    println!("create OrderItem table");
    create_table_table_if_not_exist(&conn);
}

fn create_table_table_if_not_exist(conn: &Connection)->rusqlite::Result<()>{
    conn.execute("CREATE TABLE IF NOT EXISTS tables(
    id INTEGER PRIMARY KEY , code TEXT NOT NULL UNIQUE
    )", [])?;
    Ok(())
}
fn create_menu_table_if_not_exist(conn: &Connection)->rusqlite::Result<()>{
    conn.execute("CREATE TABLE IF NOT EXISTS menus(
        id INTEGER PRIMARY KEY , name TEXT NOT NULL 
        )", [])?;
        Ok(())
}
fn create_order_table_if_not_exist(conn: &Connection)->rusqlite::Result<()>{
    conn.execute("CREATE TABLE IF NOT EXISTS orders(
        id INTEGER PRIMARY KEY ,table_id INTEGER NOT NULL , FOREIGN KEY(table_id) REFERENCES tables(id), UNIQUE(table_id)
        )", [])?;
        Ok(())
}
fn create_order_item_table_if_not_exist(conn: &Connection)->rusqlite::Result<()>{
    conn.execute("CREATE TABLE IF NOT EXISTS orderItems(
        id INTEGER PRIMARY KEY , 
        menu_id INTEGER NOT NULL, 
        cooking_time INTEGER NOT NULL , 
        quantity INTEGER NOT NULL default 1, 
        order_id INTEGER NOT NULL , FOREIGN KEY(order_id) REFERENCES orders(id), 
        FOREIGN KEY(menu_id) REFERENCES menus(id),
        )", [])?;
        Ok(())
}