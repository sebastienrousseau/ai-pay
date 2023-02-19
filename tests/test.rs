// #[cfg(test)]
// mod tests {
//     use actix_web::{http::StatusCode, test, web, App};
//     use aipay::{process_payment, PaymentParams};

//     #[actix_rt::test]
//     async fn test_process_payment_success() {
//         let mut app = test::init_service(
//             App::new().service(web::resource("/payment").route(web::get().to
//         );
//         // let mut app = test::init_service(
//         //     App::new().service(web::resource("/payment").route(web::get().to(process_payment))),
//         // )
//         // .await;

//         let payment_params = PaymentParams {
//             id: 1,
//             amount: 500.0,
//         };

//         let response = test::TestRequest::get()
//             .uri("/payment?id=1&amount=500.0")
//             .send_request(&mut app)
//             .await
//             .expect("Failed to execute request.");

//         assert_eq!(response.status(), StatusCode::OK);

//         let body = test::read_body(response).await;
//         let expected_body =
//             "{\"id\":1,\"amount\":500.0,\"success\":true,\"fraud\":false,\"duration\":0.0,\"response\":\"Payment 1 processed\"}";

//         assert_eq!(body, expected_body);
//     }

//     #[actix_rt::test]
//     async fn test_process_payment_fraud() {
//         let mut app = test::init_service(
//             App::new().service(web::resource("/payment").route(web::get().to(process_payment))),
//         )
//         .await;
//         let payment_params = PaymentParams {
//             id: 2,
//             amount: 1500.0,
//         };

//         let response = test::TestRequest::get()
//             .uri("/payment?id=2&amount=1500.0")
//             .send_request(&mut app)
//             .await
//             .expect("Failed to execute request.");

//         assert_eq!(response.status(), StatusCode::PAYMENT_REQUIRED);

//         let body = test::read_body(response).await;
//         let expected_body =
//             "{\"id\":2,\"amount\":1500.0,\"success\":false,\"fraud\":true,\"duration\":0.0,\"response\":\"Payment 2 processed\"}";

//         assert_eq!(body, expected_body);
//     }

//     #[actix_rt::test]
//     async fn test_process_payment_invalid_input() {
//         let mut app = test::init_service(
//             App::new().service(web::resource("/payment").route(web::get().to(process_payment))),
//         )
//         .await;

//         // Invalid input (negative amount)
//         let response = test::TestRequest::get()
//             .uri("/payment?id=3&amount=-500.0")
//             .send_request(&mut app)
//             .await
//             .expect("Failed to execute request.");

//         assert_eq!(response.status(), StatusCode::BAD_REQUEST);

//         // Invalid input (invalid ID)
//         let response = test::TestRequest::get()
//             .uri("/payment?id=0&amount=500.0")
//             .send_request(&mut app)
//             .await
//             .expect("Failed to execute request.");

//         assert_eq!(response.status(), StatusCode::BAD_REQUEST);
//     }
// }
