#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use std::str::FromStr;

use crate::models::_entities::products::Entity as ProductEntity;
use crate::models::_entities::resources::{
    ActiveModel as ResourceActiveModel, Entity as ResourceEntity, Model as ResourceModel,
};
use crate::models::_entities::sales::{ActiveModel, Entity, Model};
use crate::models::_entities::users::Entity as UsersEntity;
use loco_rs::prelude::*;
use random_word::Lang;
use reqwest::{header, Client, Response};
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
                            let _resource = resource.insert(&ctx.db).await?;

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

struct ProxMoxApi {
    pub base_api: String,
    pub client: Option<Client>,
    pub header_token_value: String,
    pub header_token_key: String,
}

fn config_proxmox() -> ProxMoxApi {
    let token_id = "bot-admin@pve!maglev";
    let token_secret = "ca1413cf-42aa-474c-95aa-84643dd77580";
    let base_api = String::from("https://10.10.1.2:8006/api2/json");
    let token = format!("{}={}", &token_id, &token_secret);
    let name_token = String::from("PVEAPIToken");
    let header_token_key = String::from("Authorization");
    let header_token_value = format!("{}={}", name_token, token);

    let client_req = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build();

    match client_req {
        Ok(client) => {
            return ProxMoxApi {
                base_api,
                client: Some(client),
                header_token_key,
                header_token_value,
            };
        }
        Err(_) => {
            return ProxMoxApi {
                base_api,
                client: None,
                header_token_key,
                header_token_value,
            };
        }
    }
}

struct ResourceItem {
    vmid: Option<i32>,
}

#[derive(Debug, Deserialize)]
enum ResourceType {
    Qemu {
        cpu: f64,
        diskwrite: i64,
        node: String,
        netin: i64,
        tags: String,
        maxcpu: i32,
        maxdisk: i64,
        uptime: i64,
        mem: i64,
        id: String,
        diskread: i64,
        maxmem: i64,
        netout: i64,
        template: i64,
        #[allow(dead_code)]
        type_field: String,
        vmid: i32,
        disk: i64,
        status: String,
        name: String,
    },
    Lxc {
        diskread: i64,
        maxmem: i64,
        netout: i64,
        template: i64,
        vmid: i32,
        #[allow(dead_code)]
        type_field: String,
        status: String,
        name: String,
        disk: i64,
        node: String,
        diskwrite: i64,
        cpu: f64,
        netin: i64,
        tags: String,
        maxcpu: i32,
        maxdisk: i64,
        uptime: i64,
        mem: i64,
        id: String,
    },
    Node {
        #[allow(dead_code)]
        type_field: String,
        status: String,
        disk: i64,
        maxmem: i64,
        maxdisk: i64,
        uptime: i64,
        mem: i64,
        id: String,
        node: String,
        cpu: f64,
        cgroup_mode: i32,
        level: String,
        maxcpu: i32,
    },
    Storage {
        #[allow(dead_code)]
        type_field: String,
        maxdisk: i64,
        plugintype: String,
        status: String,
        disk: i64,
        id: String,
        node: String,
        shared: i64,
        content: String,
        storage: String,
    },
    Sdn {
        id: String,
        sdn: String,
        status: String,
        node: String,
        #[allow(dead_code)]
        type_field: String,
    },
}

#[derive(Debug, serde::Deserialize)]
struct ResponseClusterResource {
    data: Vec<ResourceType>,
}

async fn get_resource() -> std::result::Result<Response, reqwest::Error> {
    let proxmox_api = config_proxmox();
    let url = format!("{}/cluster/resources", proxmox_api.base_api);

    match proxmox_api.client {
        Some(client) => {
            let res = client
                .get(url)
                .header(proxmox_api.header_token_key, proxmox_api.header_token_value)
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .send()
                .await;
            return res;
        }
        None => panic!("Not found client"),
    }
}

async fn get_last_vmid() -> ResourceItem {
    let res = get_resource().await;
    match res {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<ResponseClusterResource>().await {
                    Ok(body) => {
                        tracing::info!("response-body: {:?}", body);
                        let max_vmid = body
                            .data
                            .iter()
                            .filter_map(|item| match item {
                                ResourceType::Qemu { vmid, .. } => Some(vmid),
                                ResourceType::Lxc { vmid, .. } => Some(vmid),
                                _ => None,
                            })
                            .filter_map(|vmid| Some(vmid))
                            .max();
                        return ResourceItem {
                            vmid: max_vmid.copied(),
                        };
                    }
                    Err(_) => {
                        println!("Hm, the response didn't match the shape we expected.");
                        return ResourceItem { vmid: Some(109) };
                    }
                }
            } else {
                panic!("Not connect ok resource: {}", response.status())
            }
        }
        Err(_) => ResourceItem { vmid: None },
    }
}

struct LxcConfig {
    hostname: String,
    space: i32,
    ssh_keys: String,
    password: String,
    vmid: String,
    memory: i32,
    core: i32,
}

fn config_lxc(configs: LxcConfig) -> LxcPayload {
    let net0 =
        String::from("name=eth0,bridge=vmbr0,firewall=1,ip=10.10.1.20/24,gw=10.10.1.1,ip6=dhcp");
    let rootfs = format!("local-lvm:{}", &configs.space);
    let cores = configs.core.to_string();
    let memory = configs.memory.to_string();
    let payload = LxcPayload {
        ostemplate: String::from("local:vztmpl/debian-11-standard_11.7-1_amd64.tar.zst"),
        vmid: configs.vmid,
        hostname: configs.hostname,
        password: configs.password,
        ssh_public_keys: configs.ssh_keys,
        memory,
        rootfs,
        cores,
        swap: String::from("0"),
        net0,
        start: true,
    };
    payload
}

async fn create_lxc(params: &Resource, password: String, ssh_keys: String) {
    let proxmox_api = config_proxmox();
    let node_name = "data";
    let lxc_url = format!("{}/nodes/{}/lxc", proxmox_api.base_api, node_name);
    let resource_vmid = get_last_vmid().await;
    tracing::info!("resource_vmid: {:?}", resource_vmid.vmid);
    let vmid = if resource_vmid.vmid.is_some() {
        let mut next_vmid = resource_vmid.vmid.unwrap();
        next_vmid = next_vmid + 1;
        next_vmid.to_string()
    } else {
        String::from("100")
    };
    let configs = LxcConfig {
        vmid,
        space: params.space.clone(),
        hostname: params.hostname.clone(),
        core: params.core.clone(),
        memory: params.memory.clone(),
        password,
        ssh_keys,
    };
    let payload = config_lxc(configs);
    let json_string = serde_json::to_string(&payload).expect("Failed to serialize");
    tracing::info!("json body {}", json_string);
    match proxmox_api.client {
        Some(client) => {
            let res = client
                .post(lxc_url)
                .header(proxmox_api.header_token_key, proxmox_api.header_token_value)
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .body(json_string)
                .send()
                .await;
            match res {
                Ok(response) => {
                    if response.status().is_success() {
                        tracing::info!("resource created");
                    } else {
                        tracing::error!("error to create resource: {:?}", response);
                    }
                }
                Err(err) => {
                    tracing::error!("error in request: {:?}", err);
                }
            }
        }
        None => println!("Not found client"),
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
