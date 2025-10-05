use actix_web::{delete, get, post, web::Json, HttpResponse, Responder};

use crate::inputs::{CreateOrederInput, DeleteOrder};
use crate::outputs::{CreateOrderResponse, Depth};

#[post("/order")]

pub async fn create_order(body: Json<CreateOrederInput>) -> impl Responder {
    let _price = body.0.price;
    let _quantity = body.0.quantity;
    let _user_id = body.0.user_id;

    return HttpResponse::Ok().json(CreateOrderResponse {
        order_id: String::from("ads"),
    });
}

#[delete("/order")]
pub async fn delete_order(Json(body): Json<DeleteOrder>) -> impl Responder {
    let order_id = body.order_id;
    HttpResponse::Ok().json(DeleteOrderResponse {
        filled_qty: 0,
        average_price: 100,
    })
}

#[get("/depth")]
pub async fn get_depth() -> impl Responder {
    HttpResponse::Ok().json(Depth {
        bids: vec![],
        asks: vec![],
        lastUpdateId: String::from("adsa"),
    })
}
