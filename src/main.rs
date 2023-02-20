// Import necessary crates and modules
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::time::Instant;

// Define a struct `PaymentParams` that represents the payment parameters
// This struct implements the `Deserialize` trait to allow for deserializing from JSON
#[derive(Debug, Deserialize)]
pub struct PaymentParams {
    pub id: u64,
    pub amount: f64,
    pub payment_info: String,
}

// Define a struct `PaymentResult` that represents the result of the payment
// This struct implements the `Deserialize` trait to allow for deserializing from JSON
#[derive(Deserialize)]
pub struct PaymentResult {
    pub id: u64,
    pub amount: f64,
    pub payment_info: String,
    pub success: bool,
    pub fraud: bool,
    pub duration: f64,
    pub response: String,
}
// Implement the `Display` trait for `PaymentResult`
impl std::fmt::Display for PaymentResult {
    // Define the `fmt` function that formats the `PaymentResult` struct as a string
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "| {:^4} | {:^20} | {:^20} | {:>10.2}$ | {:>10}ns | {:<10} | {:<20} |\n",
            self.id,
            if self.success {
                "✅ Payment successful  "
            } else {
                "❌ Payment failed      "
            },
            if self.fraud {
                "🔔 Fraud detected      "
            } else {
                "👍 No fraud detected   "
            },
            self.amount,
            format!("{:?}", self.duration),
            self.payment_info,
            self.response
        )
    }
}

// Define the `process_payment` function that processes the payment
// This function is marked with the `get` attribute to indicate that it is a GET request handler
// It takes in the payment parameters as a query parameter
#[get("/payment")]
pub async fn process_payment(web::Query(params): web::Query<PaymentParams>) -> impl Responder {
    // Create a `PaymentParams` object from the query parameter
    let payment = PaymentParams {
        id: params.id,
        amount: params.amount,
        payment_info: params.payment_info.clone(),
    };
    // Get the start time
    let start = Instant::now();
    // Predict whether the payment is fraudulent based on the payment amount
    let is_fraud = predict_fraud(&payment);
    // Get the duration of the payment processing
    let duration = start.elapsed();
    // Get the payment information
    let payment_info = params.payment_info;
    // Print the start time and duration to the console
    println!("start: {:?}", start);
    println!("duration: {:?}", duration);
    // Create a `PaymentResult` object from the payment processing results
    let result = PaymentResult {
        id: payment.id,
        success: !is_fraud,
        fraud: is_fraud,
        amount: payment.amount,
        duration: duration.as_nanos() as f64,
        payment_info: payment_info,
        response: format!("Payment {} processed", payment.id),
    };
    // Return an HTTP response with the `PaymentResult` object formatted as a string
    HttpResponse::Ok().body(result.to_string())
}

// Define the `predict_fraud` function that predicts whether the payment is fraudulent
// This function takes in a reference to a `PaymentParams` object
// If the payment amount is greater than 1000.0, it is considered fraudulent
fn predict_fraud(payment: &PaymentParams) -> bool {
    payment.amount > 1000.0
}

// Define the main function that starts the HTTP server
// This function is marked with the `actix_web::main` attribute to indicate that it is the main entry point for the application
#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    // Create an HTTP server and bind it to a specific address and port
    HttpServer::new(|| App::new().service(process_payment))
        .bind("127.0.0.1:8080")?
        .run()
        .await?;

    Ok(())
}
