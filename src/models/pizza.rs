use serde::{Deserialize, Serialize};
use validator::Validate;


#[derive(Validate, Deserialize, Serialize)]
pub struct BuyPizzaRequest{
    #[validate(length(min = 1, message = "Pizza name required"))]
    pub pizza_name: String,

}

#[derive(Validate, Deserialize, Serialize)]
pub struct UpdatePizzaURL{
    pub uuid: String,
}
