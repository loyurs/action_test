use actix_web::{
    Result,error, middleware, web::{self, Json}, App, Error, HttpRequest, HttpResponse, HttpServer, FromRequest, Responder,
};
use env_logger;
use futures::{StreamExt, future::ready};
use json::JsonValue;
use serde::{Deserialize, Serialize};
use lib1::{self, logy_pop_cards};
use futures::future::{ Ready};

#[derive(Debug, Serialize, Deserialize)]
struct MyObj {
    
    number: i32,
    // name: String,
}


//return errpr
#[derive(Serialize)]
struct Err_return {
    msg: String
}


#[derive(Debug,Serialize,Deserialize)]
struct Stars_Msg {
    FiveStars: String,
    FourStars: String,
    ThreeStars: String,
}


//如果要使用m impl Responder   trait进行返回的话，需要再这个结构体实现此项特性

// impl Responder for Stars_Msg {
//     type Error = Error;
//     type Future = Ready<Result<HttpResponse, Error>>;

//     fn respond_to(self, _req: &HttpRequest) -> Self::Future {
//         let body = serde_json::to_string(&self).unwrap();
//         // Create response and set content type
//         ready(Ok(HttpResponse::Ok()
//             .content_type("application/json")
//             .body(body)))
//     }
// }

//加一个参数认证再执行

//关键点，取MyObj的参数，需要   item.0.(字段名)进行取得,详见源码
// impl<T> Json<T> {
//     /// Deconstruct to an inner value
//     pub fn into_inner(self) -> T {
//         self.0
//     }
// }
async fn ipk(item: web::Json<MyObj>) -> HttpResponse {
    println!("?");
    if item.0.number > 100000 {
        let c_msg = format!("Error,numbers{} out of range, max =100000",item.0.number);
        let pt = Err_return {msg: c_msg.to_string()};
        HttpResponse::Ok().json(pt)
    } else {
        let card_pool = lib1::logy_pop_cards::Cards_pool::new("nonol".to_string(), (90,9,1));
        let ppp = logy_pop_cards::when_get_file(card_pool, item.0.number).1;
        println!("???");
        let tt = Stars_Msg {
        FiveStars: ppp.0.to_string(),//item.number.to_string(),
        FourStars: ppp.1.to_string(),
        ThreeStars: ppp.2.to_string(),
    };
    HttpResponse::Ok().json(tt)    //想要以json()返回，只需要实现   #[derive(Serialize)]
    }
}





/// This handler uses json extractor
//处理json的方式，Myobj需要实现Serialize, Deserialize   item.0指代
async fn index(item: web::Json<MyObj>) -> HttpResponse {
    println!("model: {:?}", &item);

    HttpResponse::Ok().json(item.0) // <- send response
}


/// This handler uses json extractor with limit
async fn extract_item(item: web::Json<MyObj>, req: HttpRequest) -> HttpResponse {
    println!("request: {:?}", req);
    println!("model: {:?}", item);
    HttpResponse::Ok().json(item.0) // <- send json response
}

const MAX_SIZE: usize = 262_144; // max payload size is 256k

/// This handler manually load request payload and parse json object
async fn index_manual(mut payload: web::Payload) -> Result<HttpResponse, Error> {
    // payload is a stream of Bytes objects
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    // body is loaded, now we can deserialize serde-json
    let obj = serde_json::from_slice::<MyObj>(&body)?;
    Ok(HttpResponse::Ok().json(obj)) // <- send response
}

async fn index_mjsonrust(body: web::Bytes) -> Result<HttpResponse, Error> {
    // body is loaded, now we can deserialize json-rust
    let result = json::parse(std::str::from_utf8(&body).unwrap()); // return Result
    let injson: JsonValue = match result {
        Ok(v) => v,
        Err(e) => json::object! {"err" => e.to_string() },
    };
    //json转换为

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(injson.dump()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .app_data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
            .service(web::resource("/extractor").route(web::post().to(index)))
            .service(
                web::resource("/extractor2")
                    .app_data(web::JsonConfig::default().limit(1024)) // <- limit size of the payload (resource level)
                    .route(web::post().to(extract_item)),
            )
            .service(web::resource("/manual").route(web::post().to(index_manual)))
            .service(web::resource("/mjsonrust").route(web::post().to(index_mjsonrust)))
            .service(web::resource("/").route(web::post().to(index)))
            .service(web::resource("/nishi").route(web::post().to(ipk)))
    })
    .bind("0.0.0.0:80")?
    .run()
    .await
}
