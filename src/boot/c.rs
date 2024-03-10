use std::collections::HashMap;
use std::sync::Arc;

use actix_web::dev::ServiceRequest;
use notify::event::CreateKind;
use once_cell::sync::OnceCell;
use rbatis::core::db::DBPoolOptions;
use rbatis::rbatis::Rbatis;
use regex::Regex;

use crate::boot::global;
use crate::module::ifile;

lazy_static! {
    pub static ref RB: Rbatis = Rbatis::new();
    pub static ref RE: Regex = Regex::new(r"(?x)//(.+):(?P<anchor>[^@\s]+)@").unwrap();
    pub static ref HOME: String = global().watch_path.as_str().to_string();
}

/// 脱敏处理
fn desensitive(input: &str) -> String {
    RE.captures(input).and_then(|cap| {
        cap.name("anchor").map(|anchor| {
            input.replace(anchor.as_str(), "*******")
        })
    }).unwrap()
}

pub async fn init_rbatis() {
    if let Some(db) = &global().postgres {
        let db_pool_options = DBPoolOptions {
            max_connections: db.max,
            min_connections: db.min,
            ..Default::default()
        };
        RB.link_opt(&db.dsn, db_pool_options).await.unwrap();
        log::info!("rbatis::datasource {}   {} ~ {}", desensitive(&db.dsn), db.min, db.max)
    }
}

pub async fn init_db_schema() {
    let tables: i64 = RB.fetch("select count(*) from pg_tables where tablename = 'files';", vec![]).await.unwrap();
    if tables == 0 {
        // 1.建表
        let sql = std::fs::read_to_string("scripts/create.sql").unwrap();
        let _ = RB.exec(sql.as_str(), vec![]).await;
        // 2.记录管理路径
        let watch_path = global().watch_path.as_str();
        ifile::bs::create(CreateKind::Folder, watch_path).await;
    }
    // let files: i64 = RB.fetch("select count(*) from files;", vec![]).await.unwrap();
    // if files < 2 {
    let root_path = ifile::bs::check_path(global().watch_path.as_str()).await;
    if let None = root_path {
        // 3.初始建立索引
        let watch_path = global().watch_path.as_str();
        ifile::bs::index(watch_path).await;
    }
}

// Rbatis new_addition ------------------------------------------------------
pub static RB_OLD: OnceCell<Arc<Rbatis>> = OnceCell::new();

pub async fn init_rbatis_old() {
    if let Some(db) = &global().postgres {
        let rbatis = Rbatis::new();
        let db_pool_options = DBPoolOptions {
            max_connections: db.max,
            min_connections: db.min,
            ..Default::default()
        };
        rbatis.link_opt(&db.dsn, db_pool_options).await.unwrap();
        assert!(RB_OLD.set(Arc::new(rbatis)).is_ok());
        log::info!("rbatis_old::datasource {}   {} ~ {}", desensitive(&db.dsn), db.min, db.max)
    }
}

pub fn rb<'a>() -> &'a Rbatis {
    RB_OLD.get().unwrap()
}

/// 是否白名单请求
pub fn is_with_list(req: &ServiceRequest) -> bool {
    // Some({"/login": {"POST": 1}, "/category/warehouse": {"GET": 1}, "/userdocs/warehouse": {"GET": 1}})
    static WHITELIST_MAP: OnceCell<Arc<HashMap<String, HashMap<String, i8>>>> = OnceCell::new();
    let whitelist_map = WHITELIST_MAP.get_or_init(|| {
        let conf_white_list = global().white_list.clone();
        Arc::new(conf_white_list.into_iter().map(|(path, methods)|
            (path, methods.into_iter().map(|method| (method, 1)).collect())).collect())
    });

    let result = whitelist_map.iter().map(|(path, _)| {
        if path.ends_with("*") {
            let whitelist_path = path.split("*").collect::<Vec<&str>>();
            if req.path().starts_with(whitelist_path.first().unwrap()) { 1 } else { 0 }
        } else {
            if req.path() == path { 1 } else { 0 }
        }
    }).collect::<Vec<_>>().iter().sum::<i32>();

    result > 0
}


/// -------------------------- Test: The following --------------------------------------------------
/// CTX测试模块
#[cfg(test)]
mod ctx_mod_test {
    use rbatis::crud::CRUD;
    use rbatis::plugin::page::PageRequest;
    use rbatis::rbatis::Rbatis;

    use crate::boot::c::rb;
    use crate::boot::start;
    use crate::module::user::Users;

    /// 测试 - rbatis - 池化
    #[tokio::test]
    async fn test_rbatis_pool() {
        // log::info!("{:?}", "中华人民共和国");
        //
        // // rbatis::core::runtime::task::block_on(async {
        // // init_rbatis().await;
        // start().await;
        // let rbatis = rb();
        // // 连接数据库
        // let page_req = PageRequest::new(1, 10);
        // let data = rbatis
        //     .fetch_page_by_wrapper::<Users>(
        //         rbatis
        //             .new_wrapper()
        //             .is_not_null("username")
        //             .order_by(true, &["id"]),
        //         &page_req,
        //     )
        //     .await;
        // println!("{:?}", data)
        // });
    }

    /// 测试 - rbatis - 基本
    #[tokio::test]
    async fn test_rbatis_basic() {
        // // rbatis::core::runtime::task::block_on(async {
        // let rbatis = Rbatis::new();
        // // 连接数据库
        // rbatis.link("postgres://favorites_rw:agYx0DzkYsRASuQT@182.92.107.221:30432/favorites").await.unwrap();
        // let page_req = PageRequest::new(1, 10);
        // let data = rbatis
        //     .fetch_page_by_wrapper::<Users>(
        //         rbatis
        //             .new_wrapper()
        //             .is_not_null("username")
        //             .order_by(true, &["id"]),
        //         &page_req,
        //     )
        //     .await;
        // println!("{:?}", data)
        // // });
    }
}


