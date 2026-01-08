// Actix-Web 프레임워크와 관련 모듈 가져오기: HTTP 메서드, 웹 데이터, HTTP 응답 등
// Actix-Web을 사용한 이유: Rust에서 가장 빠르고 인기 있는 웹 프레임워크로,
// 높은 성능, 비동기 처리, 풍부한 미들웨어 생태계를 제공함
use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};
// Serde: 직렬화/역직렬화 지원 (JSON <-> 구조체 변환)
// Serde를 사용한 이유: Rust에서 가장 표준적이고 효율적인 직렬화 라이브러리로,
// JSON 처리가 매우 간단하고 타입 안정성을 보장함
use serde::{Deserialize, Serialize};
// UUID: 고유 식별자 생성
// UUID를 사용한 이유: 분산 시스템에서 식별자 충돌을 방지하고,
// 사용자가 직접 ID를 생성하지 않도록 하여 보안성과 일관성 확보
use uuid::Uuid;
// Mutex: 여러 스레드에서 안전하게 공유 데이터에 접근하기 위한 동시성 제어
// Mutex를 사용한 이유: Actix-Web이 요청마다 새로운 스레드나 태스크를 생성하므로,
// 공유 상태 데이터에 대한 안전한 동시 접근이 필요함
use std::sync::Mutex;

// 데이터 모델

// User 구조체: 사용자 정보를 저장 (DB 테이블의 행과 유사)
// Clone을 derive한 이유: 응답으로 사용자 데이터를 반환할 때 소유권 이동 문제를 방지하기 위해
#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: String,      // 고유 식별자 (UUID)
    name: String,    // 사용자 이름
    email: String,   // 이메일 주소
}

// CreateUserRequest: 사용자 생성 요청 바디 구조체 (필수 필드만 포함)
// 별도의 구조체를 사용한 이유: 사용자 생성 시 ID는 서버에서 자동 생성하므로
// 클라이언트가 ID를 제공하지 않도록 API 설계를 명확히 구분하기 위해
#[derive(Debug, Deserialize)]
struct CreateUserRequest {
    name: String,    // 필수: 사용자 이름
    email: String,   // 필수: 이메일 주소
}

// UpdateUserRequest: 사용자 수정 요청 바디 구조체 (필드는 선택적)
// Option 타입을 사용한 이유: 부분 업데이트(PATCH 동작)를 지원하여
// 클라이언트가 필요한 필드만 수정할 수 있도록 유연성 제공
#[derive(Debug, Deserialize)]
struct UpdateUserRequest {
    name: Option<String>,   // 선택적: 이름만 수정할 수 있음
    email: Option<String>,  // 선택적: 이메일만 수정할 수 있음
}

// API 응답

// ApiResponse: 모든 API 엔드포인트의 응답 형식을 통일하는 제네릭 구조체
// 제네릭을 사용한 이유: 모든 엔드포인트에서 일관된 응답 형식을 유지하면서도
// 다양한 데이터 타입을 반환할 수 있도록 재사용성 확보
// Option을 사용한 이유: 성공 시 데이터 없이 메시지만 반환하거나,
// 실패 시 메시지 없이 에러만 반환하는 등 유연한 응답 구성 지원
#[derive(Serialize)]
struct ApiResponse<T> {
    success: bool,           // 요청 성공 여부
    data: Option<T>,         // 반환할 데이터 (제네릭 타입)
    message: Option<String>, // 추가 정보 또는 에러 메시지
}

impl<T> ApiResponse<T> {
    // 성공 응답 생성자: 데이터를 포함한 성공 응답
    // Builder 패턴 대신 생성자 메서드를 사용한 이유:
    // 간단한 사용법과 가독성 향상, 별도의 Builder 구현 없이
    // 타입 안전성을 유지하면서 응답 생성을 간소화하기 위해
    fn success(data: T) -> Self {
        ApiResponse {
            success: true,
            data: Some(data),
            message: None,
        }
    }

    // 실패 응답 생성자: 에러 메시지를 포함한 실패 응답
    // &str을 인자로 받는 이유: 문자열 리터럴이나 소유된 문자열 모두
    // 편리하게 전달할 수 있도록 인터페이스 유연성 확보
    fn error(message: &str) -> Self {
        ApiResponse {
            success: false,
            data: None,
            message: Some(message.to_string()),
        }
    }
}

impl ApiResponse<()> {
    // 메시지만 전달하는 성공 응답 생성자 (데이터 없음)
    // Unit 타입(())에 대한 별도 impl을 사용한 이유:
    // 데이터가 없는 응답(삭제, 업데이트 등)에서 제네릭 타입 지정 불편함을 해소하고,
    // 빈 데이터 필드가 아닌 명시적인 None 처리로 의도를 명확히 하기 위해
    fn ok(message: &str) -> Self {
        ApiResponse {
            success: true,
            data: None,
            message: Some(message.to_string()),
        }
    }
}
// 앱 상태
// AppState를 사용한 이유: 애플리케이션 전반에서 공유되는 상태(데이터베이스, 캐시 등)를
// 관리하기 위한 중앙 집중식 구조로, 모든 핸들러에서 동일한 상태에 접근 가능
struct AppState {
    users: Mutex<Vec<User>>,  // 사용자 목록 (인메모리 DB)
}
 // 핸들러
// web::Data를 사용한 이유: 애플리케이션 상태에 대한 스레드 안전한 참조를 제공하고,
// Actix-Web이 이를 자동으로 관리하여 수명 주기 처리를 간소화
// async fn을 사용한 이유: 비동기 처리를 지원하여 여러 요청을 동시에 효율적으로 처리
// impl Responder를 반환하는 이유: 다양한 HTTP 응답 타입을 유연하게 반환 가능
#[get("/api/users")]
async fn get_users(data: web::Data<AppState>) -> impl Responder {
    let users = data.users.lock().unwrap();
    HttpResponse::Ok().json(ApiResponse::success(users.clone()))
}

#[get("/api/users/{id}")]
async fn get_user(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let id = path.into_inner();
    let users = data.users.lock().unwrap();

    // match를 사용한 이유: Rust의 패턴 매칭을 활용하여
    // 명시적이고 안전하게 존재하는 경우와 없는 경우를 처리
    match users.iter().find(|u| u.id == id) {
        Some(user) => HttpResponse::Ok().json(ApiResponse::success(user.clone())),
        None => HttpResponse::NotFound().json(ApiResponse::<()>::error("사용자를 찾을 수 없습니다")),
    }
}

#[post("/api/users")]
async fn create_user(
    data: web::Data<AppState>,
    body: web::Json<CreateUserRequest>,
) -> impl Responder {
    // 유효성 검사
    // 서버 사이드 검증을 하는 이유: 클라이언트 검증은 우회 가능하므로
    // 데이터 무결성과 보안을 위해 서버에서 반드시 검증 수행
    if body.name.trim().is_empty() {
        return HttpResponse::BadRequest()
            .json(ApiResponse::<()>::error("이름은 필수입니다"));
    }

    // 간단한 이메일 형식 검증 (실제로는 더 정교한 regex 사용 권장)
    // early return 패턴을 사용한 이유: 중첩 if문을 피하고
    // 실패 시 즉시 응답하여 코드 가독성과 유지보수성 향상
    if !body.email.contains('@') {
        return HttpResponse::BadRequest()
            .json(ApiResponse::<()>::error("유효한 이메일을 입력하세요"));
    }

    let mut users = data.users.lock().unwrap();

    // 이메일 중복 체크
    // 중복 검사를 하는 이유: 데이터 일관성을 유지하고
    // 비즈니스 로직 요구사항(고유 이메일)을 강제하기 위해
    if users.iter().any(|u| u.email == body.email) {
        return HttpResponse::Conflict()
            .json(ApiResponse::<()>::error("이미 존재하는 이메일입니다"));
    }

    // Uuid::new_v4()를 사용한 이유: 버전 4 UUID는 랜덤 생성 방식으로
    // 충돌 가능성이 극히 낮고 예측 불가능하여 보안성 우수
    let user = User {
        id: Uuid::new_v4().to_string(),
        name: body.name.clone(),
        email: body.email.clone(),
    };

    users.push(user.clone());

    // 201 Created 상태 코드를 반환하는 이유: 새 리소스가 성공적으로 생성되었음을 명시
    HttpResponse::Created().json(ApiResponse::success(user))
}

#[put("/api/users/{id}")]
async fn update_user(
    data: web::Data<AppState>,
    path: web::Path<String>,
    body: web::Json<UpdateUserRequest>,
) -> impl Responder {
    let id = path.into_inner();
    let mut users = data.users.lock().unwrap();

    // iter_mut()를 사용한 이유: 벡터 내 요소를 직접 수정하기 위해
    // 가변 참조(mutable reference)를 얻어야 함
    match users.iter_mut().find(|u| u.id == id) {
        Some(user) => {
            // if let을 사용한 이유: Option 타입의 값을 안전하게 언래핑하고
            // Some인 경우에만 실행하여 불필요한 분기 제거
            if let Some(name) = &body.name {
                user.name = name.clone();
            }
            if let Some(email) = &body.email {
                user.email = email.clone();
            }

            HttpResponse::Ok().json(ApiResponse::success(user.clone()))
        }
        None => HttpResponse::NotFound()
            .json(ApiResponse::<()>::error("사용자를 찾을 수 없습니다")),
    }
}

#[delete("/api/users/{id}")]
async fn delete_user(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let id = path.into_inner();
    let mut users = data.users.lock().unwrap();

    // retain을 사용한 이유: 필터링된 요소만 남기고 나머지를 제거하는
    // 함수형 프로그래밍 스타일을 적용하여 코드 간결성과 명확성 확보
    let len_before = users.len();
    users.retain(|u| u.id != id);

    // 길이 비교 방식을 사용한 이유: 삭제 성공 여부를 판단하는 가장
    // 효율적인 방법으로, 별도의 플래그 변수나 find 연산 없이 확인 가능
    if users.len() < len_before {
        HttpResponse::Ok().json(ApiResponse::<()>::ok("삭제되었습니다"))
    } else {
        HttpResponse::NotFound()
            .json(ApiResponse::<()>::error("사용자를 찾을 수 없습니다"))
    }
}
#[get("/api/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "health",
        "version": "1.0.0",
    }))
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 초기 데이터
    let initial_users = vec![
        User {
            id: Uuid::new_v4().to_string(),
            name: String::from("Alice"),
            email: String::from("alice@example.com"),
        },
        User {
            id: Uuid::new_v4().to_string(),
            name: String::from("Bob"),
            email: String::from("bob@example.com"),
        },
    ];

    let app_state = web::Data::new(AppState {
        users: Mutex::new(initial_users),
    });

    println!("╔═══════════════════════════╗");
    println!("║  REST API Server v1.0.0   ║");
    println!("╚═══════════════════════════╝");
    println!("서버: http://localhost:8080");
    println!();
    println!("엔드포인트:");
    println!("  GET    /api/users");
    println!("  GET    /api/users/:id");
    println!("  POST   /api/users");
    println!("  PUT    /api/users/:id");
    println!("  DELETE /api/users/:id");
    println!("  GET    /api/health");
    println!();

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(health)
            .service(get_users)
            .service(get_user)
            .service(create_user)
            .service(update_user)
            .service(delete_user)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}