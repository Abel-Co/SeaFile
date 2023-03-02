use std::cell::RefCell;
use std::rc::Rc;

use rbatis::Value;

use crate::boot::c::RB;
use crate::module::daisy::Daisy;

pub async fn get(ids: &Vec<i64>) -> Vec<Rc<RefCell<Daisy>>> {
    let ids = ids.iter().map(|x| x.to_string()).collect::<Vec<_>>().join("");
    let sql = format!("select id, name, size as value, parent from files where parent in ({})", ids);
    RB.fetch(sql.as_str(), vec![]).await.unwrap()
}

// Fail: Database("error returned from database: operator does not exist: bigint = bigint[]")
async fn _get(ids: &Vec<i64>) -> Vec<Rc<RefCell<Daisy>>> {
    let sql = "select id, name, size as value, parent from files where parent in ($1)";
    let ids = ids.iter().map(|x| Value::Int64(x.clone())).collect();
    RB.fetch(sql, vec![ids]).await.unwrap()
}