use loco_rs::testing;
use mag::app::App;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn create_produtct() {
    testing::request::<App, _, _>(|request, _ctx| async move {
        let payload = serde_json::json!({
            "cpu": "1",
            "memory": "512mb",
            "storage_size": "5gb",
            "storage_type": "hdd",
            "name": "Dev"
        });

        let res = request.post("/api/products").json(&payload).await;
        assert_eq!(res.status_code(), 200);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn list_pruducts() {
    testing::request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/api/products").await;
        assert_eq!(res.status_code(), 200);
    })
    .await;
}
