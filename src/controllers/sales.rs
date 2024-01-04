#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use crate::models::_entities::products::Entity as ProductEntity;
use crate::models::_entities::resources::{
    ActiveModel as ResourceActiveModel, Entity as ResourceEntity, Model as ResourceModel,
};
use crate::models::_entities::sales::{ActiveModel, Entity, Model};
use crate::models::_entities::users::Entity as UsersEntity;
use loco_rs::prelude::*;
use random_word::Lang;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Resource {
    pub sale_id: i32,
    pub hostname: String,
    pub memory: i32,
    pub core: i32,
    pub space: i32,
}

impl Resource {
    fn update(&self, item: &mut ResourceActiveModel) {
        item.hostname = Set(self.hostname.clone());
        item.space = Set(self.space.clone());
        item.memory = Set(self.memory.clone());
        item.core = Set(self.core.clone());
        item.sale_id = Set(self.sale_id.clone());
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct LxcPayload {
    ostemplate: String,
    vmid: String,
    hostname: String,
    password: String,
    memory: String,
    rootfs: String,
    cores: String,
    swap: String,
    net0: String,
    start: bool,
    #[serde(rename = "ssh-public-keys")]
    ssh_public_keys: String,
}

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
    pub password: String,
    pub ssh_keys: String,
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

pub async fn update(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<ParamsUpdate>,
) -> Result<Json<Model>> {
    let status = params.status.as_str();
    match status {
        "PaymentOk" => {
            //create accoount
            let item_load = load_item(&ctx, id).await?;
            let user_id = item_load.user_id.clone();
            let product_id = item_load.product_id.clone();

            let mut item = item_load.into_active_model();
            let product_entity = ProductEntity::find_by_id(product_id).one(&ctx.db).await;

            match product_entity.unwrap() {
                Some(product) => {
                    let user_entity = UsersEntity::find_by_id(user_id).one(&ctx.db).await;

                    match user_entity {
                        Ok(user) => {
                            let word = random_word::gen(Lang::En);
                            let name = str::replace(&user.unwrap().name, " ", "");
                            let host_name = format!("{}{}{}", name, word, product.name);
                            let model = Resource {
                                sale_id: id,
                                hostname: host_name,
                                core: product.cpu,
                                memory: product.memory,
                                space: product.storage_size,
                            };
                            let mut resource = ResourceActiveModel {
                                ..Default::default()
                            };
                            model.update(&mut resource);
                            let resource = resource.insert(&ctx.db).await?;

                            let _ = self::create_lxc(
                                &model,
                                String::from(params.password.clone()),
                                String::from(params.ssh_keys.clone()),
                            )
                            .await;
                            params.update(&mut item);
                            let item = item.update(&ctx.db).await?;
                            format::json(item)
                        }
                        Err(_) => return Err(Error::BadRequest(String::from("Status not allow"))),
                    }
                }
                None => return Err(Error::BadRequest(String::from("Status not allow"))),
            }
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

async fn create_lxc(params: &Resource, password: String, ssh_keys: String) {
    let token_id = "bot-admin@pve!maglev";
    let token_secret = "ca1413cf-42aa-474c-95aa-84643dd77580";
    let base_api = "https://10.10.1.2:8006/api2/json";
    let node_name = "data";
    let lxc_url = format!("{}/nodes/{}/lxc", base_api, node_name);

    let rootfs = format!("local-lvm:{}", &params.space);
    let payload = LxcPayload {
        ostemplate: String::from("local:vztmpl/debian-11-standard_11.7-1_amd64.tar.zst"),
        vmid: String::from("107"),
        hostname: params.hostname.clone(),
        password: password,
        ssh_public_keys: ssh_keys,
        memory: params.memory.to_string(),
        rootfs,
        cores: params.core.to_string(),
        swap: String::from("0"),
        net0: String::from(
            "name=eth0,bridge=vmbr0,firewall=1,ip=10.10.1.20/24,gw=10.10.1.1,ip6=dhcp",
        ),
        start: true,
    };
    let authorization = format!("PVEAPIToken={}={}", &token_id, &token_secret);

    let client_req = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build();
    let json_string = serde_json::to_string(&payload).expect("Failed to serialize");
    println!("data {} ", json_string);
    match client_req {
        Ok(client) => {
            let res = client
                .post(lxc_url)
                .header("Authorization", &authorization)
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .body(json_string)
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
