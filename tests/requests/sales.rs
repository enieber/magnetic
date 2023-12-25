use loco_rs::testing;
use mag::app::App;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn create_sales() {
    testing::request::<App, _, _>(|request, _ctx| async move {
        let payload_product = serde_json::json!({
            "cpu": "1",
            "memory": "512mb",
            "storage_size": "5gb",
            "storage_type": "hdd",
            "name": "Dev"
        });

        //Creating a new product
        request.post("/api/products").json(&payload_product).await;

        let register_payload = serde_json::json!({
            "name": "loco",
            "email": "loco@test.com",
            "password": "loco_test123"
        });

        //Creating a new user
        request
            .post("/api/auth/register")
            .json(&register_payload)
            .await;

        let payload = serde_json::json!({
            "status": "PaymentOK",
            "user_id": 1,
            "product_id": 1
        });

        let res = request.post("/api/sales").json(&payload).await;
        assert_eq!(res.status_code(), 200);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn list_pruducts() {
    testing::request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/api/sales").await;
        assert_eq!(res.status_code(), 200);
    })
    .await;
}
