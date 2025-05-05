use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::http::Status;
use rocket::State;
use std::sync::Mutex;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

// 创建唯一用户ID
static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none")]
    pub id: Option<usize>,
    pub username: String,
    pub password: String,  // 实际应用中应该使用哈希值存储
    pub is_student: bool,
    pub student_id: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
    pub user: Option<UserInfo>,
}

#[derive(Serialize)]
pub struct UserInfo {
    pub id: usize,
    pub username: String,
    pub is_student: bool,
    pub student_id: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

// 使用Mutex创建线程安全的用户存储
pub type UserStore = Mutex<HashMap<usize, User>>;

// 注册用户
#[post("/register", format = "json", data = "<user>")]
pub async fn register_user(user_store: &State<UserStore>, mut user: Json<User>) -> Result<Json<UserInfo>, Status> {
    let mut store = user_store.lock().unwrap();
    
    // 检查用户名是否已存在
    if store.values().any(|u| u.username == user.username) {
        return Err(Status::Conflict);
    }
    
    // 验证学生信息
    if user.is_student && user.student_id.is_none() {
        return Err(Status::BadRequest);
    }
    
    let id = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed);
    
    let mut user_inner = user.into_inner();
    user_inner.id = Some(id);
    
    // 创建用户信息响应
    let user_info = UserInfo {
        id,
        username: user_inner.username.clone(),
        is_student: user_inner.is_student,
        student_id: user_inner.student_id.clone(),
        email: user_inner.email.clone(),
        phone: user_inner.phone.clone(),
    };
    
    // 保存用户
    store.insert(id, user_inner);
    
    Ok(Json(user_info))
}

// 用户登录
#[post("/login", format = "json", data = "<login>")]
pub async fn login(user_store: &State<UserStore>, login: Json<LoginRequest>) -> Json<LoginResponse> {
    let store = user_store.lock().unwrap();
    
    // 在实际应用中，应该使用哈希比较而不是明文比较密码
    for (id, user) in store.iter() {
        if user.username == login.username && user.password == login.password {
            return Json(LoginResponse {
                success: true,
                message: "登录成功".to_string(),
                user: Some(UserInfo {
                    id: *id,
                    username: user.username.clone(),
                    is_student: user.is_student,
                    student_id: user.student_id.clone(),
                    email: user.email.clone(),
                    phone: user.phone.clone(),
                }),
            });
        }
    }
    
    // 如果存储为空，添加默认用户
    if store.is_empty() {
        // 我们这里不能修改存储，因为已经锁定为不可变的
        drop(store);  // 释放锁
        
        // 添加一些示例用户
        let mut store = user_store.lock().unwrap();
        if store.is_empty() {
            store.insert(1, User {
                id: Some(1),
                username: "admin".to_string(),
                password: "admin123".to_string(),
                is_student: false,
                student_id: None,
                email: Some("admin@example.com".to_string()),
                phone: None,
            });
            
            store.insert(2, User {
                id: Some(2),
                username: "student".to_string(),
                password: "student123".to_string(),
                is_student: true,
                student_id: Some("20240001".to_string()),
                email: Some("student@example.com".to_string()),
                phone: None,
            });
            
            // 检查登录信息是否匹配
            for (id, user) in store.iter() {
                if user.username == login.username && user.password == login.password {
                    return Json(LoginResponse {
                        success: true,
                        message: "登录成功".to_string(),
                        user: Some(UserInfo {
                            id: *id,
                            username: user.username.clone(),
                            is_student: user.is_student,
                            student_id: user.student_id.clone(),
                            email: user.email.clone(),
                            phone: user.phone.clone(),
                        }),
                    });
                }
            }
        }
    }
    
    // 登录失败
    Json(LoginResponse {
        success: false,
        message: "用户名或密码错误".to_string(),
        user: None,
    })
}

// 获取用户
#[get("/users/<id>")]
pub async fn get_user(user_store: &State<UserStore>, id: usize) -> Result<Json<UserInfo>, Status> {
    let store = user_store.lock().unwrap();
    match store.get(&id) {
        Some(user) => Ok(Json(UserInfo {
            id,
            username: user.username.clone(),
            is_student: user.is_student,
            student_id: user.student_id.clone(),
            email: user.email.clone(),
            phone: user.phone.clone(),
        })),
        None => Err(Status::NotFound),
    }
}

// 初始化示例用户
pub fn init_users() -> HashMap<usize, User> {
    let mut users = HashMap::new();
    
    users.insert(1, User {
        id: Some(1),
        username: "admin".to_string(),
        password: "admin123".to_string(),
        is_student: false,
        student_id: None,
        email: Some("admin@example.com".to_string()),
        phone: None,
    });
    
    users.insert(2, User {
        id: Some(2),
        username: "student".to_string(),
        password: "student123".to_string(),
        is_student: true,
        student_id: Some("20240001".to_string()),
        email: Some("student@example.com".to_string()),
        phone: None,
    });
    
    NEXT_USER_ID.store(3, Ordering::Relaxed);
    
    users
} 