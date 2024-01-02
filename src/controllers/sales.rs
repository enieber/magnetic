#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use std::fmt::format;

use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::_entities::sales::{ActiveModel, Entity, Model};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub status: String,
    pub user_id: i32,
    pub product_id: i32,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.status = Set(self.status.clone());
        item.user_id = Set(self.user_id.clone());
        item.product_id = Set(self.product_id.clone());
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ParamsUpdate {
    pub status: String,
}

impl ParamsUpdate {
    fn update(&self, item: &mut ActiveModel) {
        item.status = Set(self.status.clone());
    }
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

pub async fn list(State(ctx): State<AppContext>) -> Result<Json<Vec<Model>>> {
    format::json(Entity::find().all(&ctx.db).await?)
}

pub async fn add(State(ctx): State<AppContext>, Json(params): Json<Params>) -> Result<Json<Model>> {
    let status = params.status.as_str();
    if status == "PendingPayment" {
        let mut item = ActiveModel {
            ..Default::default()
        };
        params.update(&mut item);
        let item = item.insert(&ctx.db).await?;
        format::json(item)
    } else {
        return Err(Error::BadRequest(String::from("Status not allow")));
    }
}

async fn login_proxmox(_ctx: &AppContext) {
    let token_id = "bot-admin@pve!maglev";
    let token_secret = "ca1413cf-42aa-474c-95aa-84643dd77580";
    let base_api = "https://10.10.1.2:8006/api2/json";
    let node_name = "data";
    let lxc_url = format!("{}/nodes/{}/lxc", base_api, node_name);

    let payload = r#"{
        "ostemplate": "local:vztmpl/debian-11-standard_11.7-1_amd64.tar.zst",
        "vmid": "103",
        "hostname": "Sample",
        "password": "1ZY2i9VsD20shVVo",
        "memory": "512",
        "rootfs": "local-lvm:6",
        "cores": "1",
        "swap": "0",
        "net0": "name=eth0,bridge=vmbr0,firewall=1,ip=10.10.1.16/24,gw=10.10.1.1,ip6=dhcp",
        "start": true
    }"#;
    let authorization = format!("PVEAPIToken={}={}", &token_id, &token_secret);

    let client_req = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build();

    match client_req {
        Ok(client) => {
            let res = client
                .post(lxc_url)
                .header("Authorization", &authorization)
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .body(payload)
                .send()
                .await;
            match res {
                Ok(response) => {
                    println!("body = {:?}", response);
                }
                Err(err) => {
                    println!("fail error: {:?}", err);
                }
            }
        }
        Err(err) => {
            println!("fail error: {:?}", err);
        }
    }
}

pub async fn update(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<ParamsUpdate>,
) -> Result<Json<Model>> {
    let status = params.status.as_str();
    match status {
        "PaymentOk" => {
            //create accoount
            let item = load_item(&ctx, id).await?;
            let mut item = item.into_active_model();
            params.update(&mut item);
            let item = item.update(&ctx.db).await?;
            let _ = self::login_proxmox(&ctx).await;
            format::json(item)
        }
        _ => {
            let item = load_item(&ctx, id).await?;
            let mut item = item.into_active_model();
            params.update(&mut item);
            let item = item.update(&ctx.db).await?;
            format::json(item)
        }
    }
}

pub async fn remove(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<()> {
    load_item(&ctx, id).await?.delete(&ctx.db).await?;
    format::empty()
}

pub async fn get_one(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Json<Model>> {
    format::json(load_item(&ctx, id).await?)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("sales")
        .add("/", get(list))
        .add("/", post(add))
        .add("/:id", get(get_one))
        .add("/:id", delete(remove))
        .add("/:id", post(update))
}
