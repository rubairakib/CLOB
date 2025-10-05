use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CreateOrederInput {
    pub price: u32,
    pub quantity: u32,
    pub user_id: u32,
}

#[derive(Deserialize, Debug)]
pub enum Option {
    Buy,
    Sell,
}

#[derive(Deserialize, Debug)]
pub struct DeleteOrder {
    pub order_id: String,
}
