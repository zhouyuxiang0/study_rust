use actix_web::{
    web,
    App,
    HttpResponse,
    HttpServer,
    Responder,
    get
};
use listenfd::ListenFd;
use std::sync::Mutex;

fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}

// 使用宏定义路由
#[get("/hello")]
fn index3() -> impl Responder {
    HttpResponse::Ok().body("lalala~")
}

fn app_index() -> impl Responder {
    HttpResponse::Ok().body("appIndex")
}

// 共享状态
struct AppState {
    app_name: String,
}

fn state_index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;

    format!("Hello {}!", app_name)
}

// 共享可变状态
struct AppStateWithCounter {
    counter: Mutex<i32>,
}

fn _index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    format!("Request number: {}", counter)
}

fn main() {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0)
    });
    // 自动重启模式 使用命令 systemfd --no-pid -s http::3000 -- cargo watch -x run
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move|| {
        App::new()
            .register_data(counter.clone())
            .route("/counter", web::get().to(_index))
            .data(AppState {app_name: String::from("Actix-web"),})
            .route("/state", web::get().to(state_index))
            .service(
                web::scope("/app")
                .route("/index.html", web::get().to(app_index))
            )
            .service(index3)
            .route("/", web::get().to(index))
            .route("/again", web::get().to(index2))
    });
    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind("127.0.0.1:3000").unwrap()
    };
    server.run().unwrap()

    // 普通模式
    // HttpServer::new(|| {
    //     App::new()
    //         .service(index3)
    //         .route("/", web::get().to(index))
    //         .route("/again", web::get().to(index2))
    // })
    // .bind("127.0.0.1:8088")
    // .unwrap()
    // .run()
    // .unwrap()
}
