
#[derive(Debug, Default)]
pub struct Login {
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Default)]   // 增加了 sqlx::FromRow
pub struct Users {
    pub id: i64,
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub avatar: Option<String>,
    pub phone: Option<String>,
    pub status: Option<i32>,    // 1.账号正常; 419.账号冻结（七牛扩展状态码）;
    // 类型：user、admin
    pub user_type: UserType,
}

#[derive(Debug, Default, PartialEq)]   // 增加了 sqlx::FromRow
pub enum UserType {
    Admin,
    #[default]
    User,
}

#[derive(Debug)]
pub struct Password {
    pub id: i64,
    pub new: String,
    pub old: String,
}
