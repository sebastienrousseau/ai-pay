extern crate rustlearn;

use actix_web::{get, web, App, HttpServer};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::prelude::*, time::Instant};

use rustlearn::prelude::*;

// A struct to represent a payment transaction
#[derive(Debug, Deserialize)]
struct PaymentTransaction {
    id: u64,
    amount: f64,
}

// Struct to represent the URL parameters for the payment transaction API
#[derive(Debug, Deserialize)]
struct PaymentParams {
    id: u64,
    amount: f64,
}

// Struct to represent the features of a payment transaction
#[derive(Debug, Serialize)]
struct PaymentFeatures {
    id: f64,
    amount: f64,
}

// Handler function for the payment transaction API
#[get("/payment")]
async fn process_payment(
    web::Query(params): web::Query<PaymentParams>,
) -> impl actix_web::Responder {
    let payment = PaymentTransaction {
        id: params.id,
        amount: params.amount,
    };
    let start = Instant::now();
    let is_fraud = predict_fraud(&payment, &nb);
    let duration = start.elapsed();
    let response = if is_fraud {
        format!(
            "Payment transaction {} is fraudulent! Execution time: {:?}",
            payment.id, duration
        )
    } else {
        format!(
            "Payment transaction {} processed successfully. Execution time: {:?}",
            payment.id, duration
        )
    };
    actix_web::HttpResponse::Ok().body(response)
}

// Train a machine learning model to predict fraud
fn train_model() -> (SparseRowArray, Array) {
    // Load the training data from a CSV file
    let mut file = File::open("transactions.csv").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let mut reader = csv::Reader::from_reader(data.as_bytes());
    let mut features: Vec<SparseRowArray> = vec![];
    let mut targets: Vec<f32> = vec![];
    for result in reader.records() {
        let record = result.unwrap();
        let id = record[0].parse().unwrap();
        let amount = record[1].parse().unwrap();
        let mut row = SparseRowArray::zeros(1, 2);
        row.set(0, id, 1.0);
        row.set(1, amount, 1.0);
        features.push(row);
        let target = record[2].parse().unwrap();
        targets.push(target);
    }

    let x = SparseRowArray::from(&features);

    let y = Array::from(targets);
    (x, y)
}

// Predict fraud using a trained machine learning model
pub fn predict_fraud(payment: &PaymentTransaction) -> bool {
    // Convert the payment transaction to features
    let features =
        SparseRowArray::from_row_iter(1, 2, vec![payment.id, payment.amount].into_iter().cloned())
            .unwrap();
    // Predict whether the payment is fraudulent using the trained model
    let prediction = nb.predict(&features).unwrap();
    prediction[0] > 0.5
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Train a machine learning model to predict fraud
    let nb = train_model().expect("Error loading training data");
    // Start the web server
    HttpServer::new(|| App::new().service(process_payment))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
