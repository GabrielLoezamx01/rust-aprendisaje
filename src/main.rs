// Serialize se encarga de serializar una estructura de datos a un formato JSON
// por ejemplo si tenemos una estructura de datos llamada MyData que contiene dos campos nombre y contraseña de tipo String esto hace que se pueda convertir a un formato JSON
// Recuerda tener todo lo que tiene en use en cargo.toml
// PARA ACTUALIZAR EL CARGO.TIOML EJECUTA EL SIGUIENTE COMANDO: cargo check
// cargo build sirve para compilar el proyecto
// cargo update sirve para actualizar las dependencias del proyecto
// cargo fetch sirve para descargar las dependencias del proyecto pero sin compilar

use serde::{Deserialize, Serialize};

/*
*
*   MI APRENDIZAJE DE RUST BY: GABRIEL LOEZA - SOFTREK
*
*   Importamos actrix_web que es un framework web para Rust
*   Importamos post, web, App, HttpResponse, HttpServer, Responder
*   Post es un macro que nos permite definir una ruta POST
*   Web es un módulo que nos permite definir un tipo de dato Json
*   Un ejemplo para web es Json<MyData> que es un tipo de dato Json que contiene un dato de tipo MyData que se mandará por POST, PUT, DELETE, etc.
*   otro dato de web es JsonValue que es un tipo de dato Json que contiene un dato de tipo Value que se mandará por POST, PUT, DELETE, etc.
*   ejemplo de JsonValue: JsonValue::String("Hola Mundo".to_string()) y JsonValue::Number(10) y JsonValue::Bool(true) y JsonValue::Array(vec![JsonValue::String("Hola Mundo".to_string()), JsonValue::Number(10), JsonValue::Bool(true)]) y JsonValue::Object(vec![("nombre".to_string(), JsonValue::String("Gabriel".to_string())), ("edad".to_string(), JsonValue::Number(10)), ("activo".to_string(), JsonValue::Bool(true))]) y JsonValue::Null y JsonValue::Undefined y JsonValue::Error y JsonValue::from("Hola Mundo") y JsonValue::from(10) y JsonValue::from(true) y JsonValue::from(vec![JsonValue::from("Hola Mundo"), JsonValue::from(10), JsonValue::from(true)]) y JsonValue::from(vec![("nombre".to_string(), JsonValue::from("Gabriel")), ("edad".to_string(), JsonValue::from(10)), ("activo".to_string(), JsonValue::from(true))]) y JsonValue::from(())
*   App es un módulo que nos permite definir una aplicación basicamente es un contenedor de rutas y el middleware y el estado de la aplicación
*   HttpResponse es un tipo de dato que nos permite devolver una respuesta HTTP
*   HttpServer es un tipo de dato que nos permite crear un servidor HTTP y ejecutarlo por ejemplo con el método run()
*/

use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};

/*
*
*   Definimos una estructura de datos llamada MyData que contiene dos campos nombre y contraseña de tipo String
*   que nos servirá para recibir los datos que se manden por POST y devolverlos en la respuesta, tambien para el metodo GET, PUT, DELETE, etc.
*/

// Se agrega #[derive(Serialize, Deserialize)] para que la estructura de datos MyData se pueda serializar y deserializar

#[derive(Serialize, Deserialize)]
struct MyData {
    nombre: String,
        : String,
}

// Al escribir el #[post("/auth")] estamos definiendo una ruta POST que se llamará /auth
// y que se ejecutará el método echo que recibe un dato de tipo Json<MyData> y devuelve un dato de tipo impl Responder
// tambien el impl tiene otros valores como impl Future<Output = Result<HttpResponse, Error>> y impl Fn() -> Future<Output = Result<HttpResponse, Error>> y impl Fn() -> Result<HttpResponse, Error> y impl Fn() -> HttpResponse y impl Fn() -> Result<HttpResponse, Error

#[post("/auth")]
async fn echo(data: web::Json<MyData>) -> impl Responder {
    HttpResponse::Ok().json(data.0)
}
// este #actix_web::main es un macro que nos permite definir la función main que se ejecutará al iniciar la aplicación

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(echo))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
