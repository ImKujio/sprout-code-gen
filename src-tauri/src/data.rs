use std::env;
use once_cell::sync::Lazy;
use redb::{AccessGuard, Database, Error, ReadableTable, TableDefinition};

const TB: TableDefinition<&str, &str> = TableDefinition::new("data");
static DB: Lazy<Database> = Lazy::new(|| {
    let exe = env::current_exe().unwrap();
    let dir = exe.parent().unwrap();
    // let cur_dir = env::current_dir().unwrap();
    // let dir = cur_dir.parent().unwrap();
    let db = Database::create(dir.to_str().unwrap().to_owned() + "\\data.redb").unwrap();
    return db;
});

pub struct Data;

impl Data {
    pub fn set(key: &str, val: &str) -> Result<(), Error> {
        let writer = DB.begin_write()?;
        {
            let mut table = writer.open_table(TB)?;
            table.insert(key, val)?;
        }
        writer.commit()?;
        Ok(())
    }

    pub fn get(key: &str) -> Result<Option<String>, Error> {
        let reader = DB.begin_read()?;
        let table = reader.open_table(TB)?;
        let val = table.get(key)?;
         match val {
            None => Ok(None) ,
            Some(d) => {
                let rst = d.value();
                Ok(Some(rst.to_owned())) }
        }
    }
}