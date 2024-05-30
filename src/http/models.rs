use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub command: String,
    pub error: i64,
    pub message: String,
    pub data: Data,
    pub connection: Connection,
    pub token: String,
    pub fromdomain: String,
    pub worktime: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    #[serde(rename = "user_fullname")]
    pub user_fullname: String,
    pub user: User,
    pub front: Front,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "user_fullname")]
    pub user_fullname: String,
    #[serde(rename = "user_name")]
    pub user_name: String,
    #[serde(rename = "user_surname")]
    pub user_surname: String,
    #[serde(rename = "user_patronymic")]
    pub user_patronymic: String,
    #[serde(rename = "user_mail")]
    pub user_mail: String,
    #[serde(rename = "user_phone")]
    pub user_phone: String,
    #[serde(rename = "user_avatar")]
    pub user_avatar: String,
    #[serde(rename = "user_balance")]
    pub user_balance: i64,
    #[serde(rename = "user_type")]
    pub user_type: i64,
    #[serde(rename = "user_status")]
    pub user_status: i64,
    #[serde(rename = "user_role_id")]
    pub user_role_id: i64,
    #[serde(rename = "user_gender")]
    pub user_gender: i64,
    #[serde(rename = "user_birthday")]
    pub user_birthday: Value,
    #[serde(rename = "user_search_disable")]
    pub user_search_disable: String,
    #[serde(rename = "user_search_disable_text")]
    pub user_search_disable_text: String,
    #[serde(rename = "user_search_disable_fulltext")]
    pub user_search_disable_fulltext: String,
    pub token: String,
    #[serde(rename = "ghost_token")]
    pub ghost_token: String,
    #[serde(rename = "user_uid")]
    pub user_uid: String,
    pub auth_key: String,
    #[serde(rename = "is_demo")]
    pub is_demo: i64,
    #[serde(rename = "show_tabbar")]
    pub show_tabbar: bool,
    pub apartment: Vec<Apartment>,
    pub stage: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Apartment {
    pub id: String,
    pub uid: String,
    pub title: String,
    #[serde(rename = "custom_title")]
    pub custom_title: String,
    #[serde(rename = "user_title")]
    pub user_title: String,
    pub here: i64,
    #[serde(rename = "user_token")]
    pub user_token: String,
    #[serde(rename = "complex_id")]
    pub complex_id: String,
    #[serde(rename = "complex_title")]
    pub complex_title: String,
    #[serde(rename = "building_id")]
    pub building_id: String,
    #[serde(rename = "building_address")]
    pub building_address: String,
    #[serde(rename = "buildings_properties_rent_available")]
    pub buildings_properties_rent_available: bool,
    #[serde(rename = "show_tabbar")]
    pub show_tabbar: bool,
    pub stage: i64,
    #[serde(rename = "accept_enabled")]
    pub accept_enabled: i64,
    pub sip: String,
    #[serde(rename = "apartment_request")]
    pub apartment_request: Value,
    #[serde(rename = "user_search_disable")]
    pub user_search_disable: String,
    #[serde(rename = "accept_data")]
    pub accept_data: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Front {
    pub title: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub app: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Connection {
    #[serde(rename = "server_real_ip")]
    pub server_real_ip: String,
    #[serde(rename = "user_ip")]
    pub user_ip: String,
}
