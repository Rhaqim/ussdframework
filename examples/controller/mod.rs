use crate::{USSDMenu, USSDRequest, UssdApp};
use actix_web::{get, web, HttpResponse, Responder};

#[get("/")]
async fn health_check() -> impl Responder {
    format!("Welcome to the USSD server")
}

pub async fn handle_ussd(
    req: web::Json<USSDRequest>,
    app: web::Data<UssdApp>,
    menus: web::Data<USSDMenu>,
) -> HttpResponse {
    let request = req.into_inner();
    let response = app.run(request, menus.get_ref().clone());
    print!("Response: {:?}", response);
    HttpResponse::Ok().body(response.message)
}
