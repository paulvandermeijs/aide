use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Assuming OpenAPI types are defined elsewhere
// use openapi_types::{OpenAPI, OpenAPIV2, OpenAPIV3, OpenAPIV3_1};

// Enum for ThemeId
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ThemeId {
    Alternate,
    Default,
    Moon,
    Purple,
    Solarized,
    BluePlanet,
    DeepSpace,
    Saturn,
    Kepler,
    Mars,
    None,
}

// Struct for ClientInfo
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientInfo {
    pub key: String,
    pub title: String,
    pub link: String,
    pub description: String,
}

// Struct for TargetInfo
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TargetInfo {
    pub key: String, // Assuming TargetId is a String
    pub title: String,
    pub extname: Option<String>,
    pub default: String,
}

// Enum for HiddenClients
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum HiddenClients {
    All(bool),
    Partial(HashMap<String, HiddenClientValue>),
    Keys(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum HiddenClientValue {
    Boolean(bool),
    StringArray(Vec<String>),
}

// Struct for HttpClientState
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HttpClientState {
    pub target_key: String,
    pub client_key: String,
}

// Struct for PathRouting
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PathRouting {
    pub base_path: String,
}

// Struct for ReferenceConfiguration
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReferenceConfiguration {
    pub theme: Option<ThemeId>,
    pub layout: Option<String>, // Can be "modern" or "classic"
    pub spec: Option<SpecConfiguration>,
    pub proxy: Option<String>,
    pub is_editable: Option<bool>,
    pub show_sidebar: Option<bool>,
    pub hide_models: Option<bool>,
    pub hide_download_button: Option<bool>,
    pub hide_test_request_button: Option<bool>,
    pub hide_search: Option<bool>,
    pub dark_mode: Option<bool>,
    pub force_dark_mode_state: Option<String>, // Can be "dark" or "light"
    pub hide_dark_mode_toggle: Option<bool>,
    pub search_hot_key: Option<char>,
    pub meta_data: Option<UseSeoMetaInput>, // Assuming UseSeoMetaInput is defined elsewhere
    pub favicon: Option<String>,
    pub hidden_clients: Option<HiddenClients>,
    pub default_http_client: Option<HttpClientState>,
    pub custom_css: Option<String>,
    pub authentication: Option<AuthenticationState>,
    pub path_routing: Option<PathRouting>,
    pub base_server_url: Option<String>,
    pub servers: Option<Vec<Server>>,
    pub with_default_fonts: Option<bool>,
    pub default_open_all_tags: Option<bool>,
    pub tags_sorter: Option<String>, // Can be "alpha" or a custom function (not directly translatable to Rust)
    pub operations_sorter: Option<String>, // Can be "alpha", "method", or a custom function
    pub _integration: Option<String>, // Enum-like String
}

// Struct for Server
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Server {
    pub url: String,
    pub description: Option<String>,
}

// Struct for BaseParameter
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BaseParameter {
    pub name: String,
    pub description: Option<String>,
    pub value: Value, // Using serde_json::Value for flexibility
    pub required: Option<bool>,
    pub enabled: bool,
}

// Enum for ContentType
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ContentType {
    ApplicationJson(Option<String>),
    ApplicationXml(Option<String>),
    TextPlain(Option<String>),
    TextHtml(Option<String>),
    ApplicationOctetStream(Option<String>),
    ApplicationFormUrlencoded(Option<String>),
    MultipartFormData(Option<String>),
}

// Struct for Cookie
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cookie {
    pub name: String,
    pub value: String,
}

// Struct for CustomRequestExample
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomRequestExample {
    pub lang: String,
    pub label: String,
    pub source: String,
}

// Struct for Header
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Header {
    pub name: String,
    pub value: String,
}

// Struct for Information
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Information {
    pub description: Option<String>,
    pub operation_id: Option<String>,
    pub parameters: Option<Vec<Parameters>>,
    pub responses: Option<HashMap<String, OpenAPI::ResponseObject>>,
    pub security: Option<Vec<OpenAPIV3::SecurityRequirementObject>>,
    pub request_body: Option<RequestBody>,
    pub summary: Option<String>,
    pub tags: Option<Vec<String>>,
    pub deprecated: Option<bool>,
    pub x_custom_examples: Option<Vec<CustomRequestExample>>,
    pub x_code_samples: Option<Vec<CustomRequestExample>>,
}

// Struct for Operation
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Operation {
    pub http_verb: HttpVerb,
    pub path: String,
    pub operation_id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub information: Option<Information>,
}

// Enum for HttpVerb
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum HttpVerb {
    GET,
    HEAD,
    PATCH,
    POST,
    PUT,
    TRACE,
    CONNECT,
    DELETE,
    OPTIONS,
}

// Struct for Parameters
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Parameters {
    pub name: String,
    pub in_field: Option<String>,
    pub description: Option<String>,
    pub required: Option<bool>,
    pub deprecated: Option<bool>,
    pub allow_empty_value: Option<bool>,
    pub style: Option<String>,
    pub explode: Option<bool>,
    pub allow_reserved: Option<bool>,
    pub schema: Option<Schema>,
    pub example: Option<serde_json::Value>,
    pub examples: Option<HashMap<String, serde_json::Value>>,
    pub content: Option<HashMap<ContentType, ContentTypeDetails>>,
    pub headers: Option<HashMap<String, OpenAPI::HeaderObject>>,
}

// Struct for ContentTypeDetails
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContentTypeDetails {
    pub schema: Option<serde_json::Value>,
    pub example: Option<serde_json::Value>,
    pub examples: Option<serde_json::Value>,
}

// Struct for RequestBody
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestBody {
    pub description: Option<String>,
    pub required: Option<bool>,
    pub content: Option<HashMap<ContentType, ContentTypeDetails>>,
}

// Struct for SpecConfiguration
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpecConfiguration {
    pub url: Option<String>,
    pub content: Option<serde_json::Value>,
}

// Struct for Schema
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Schema {
    pub type_field: String,
    pub name: Option<String>,
    pub example: Option<serde_json::Value>,
    pub default: Option<serde_json::Value>,
    pub format: Option<String>,
    pub description: Option<String>,
    pub properties: Option<HashMap<String, Schema>>,
}

// Struct for TransformedOperation
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransformedOperation {
    #[serde(flatten)]
    pub operation: Operation,
    pub path_parameters: Option<Vec<Parameters>>,
}

// Struct for AuthenticationState
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthenticationState {
    pub custom_security: bool,
    pub preferred_security_scheme: Option<String>,
    pub security_schemes: Option<SecuritySchemes>,
    pub http: HttpAuth,
    pub api_key: ApiKeyAuth,
    pub o_auth2: OAuth2Auth,
}

// Enum for SecuritySchemes
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SecuritySchemes {
    V2(OpenAPIV2::SecurityDefinitionsObject),
    V3(HashMap<String, OpenAPIV3::SecuritySchemeObject>),
    V3_1(HashMap<String, OpenAPIV3_1::SecuritySchemeObject>),
}

// Struct for HttpAuth
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HttpAuth {
    pub basic: BasicAuth,
    pub bearer: BearerAuth,
}

// Struct for BasicAuth
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BasicAuth {
    pub username: String,
    pub password: String,
}

// Struct for BearerAuth
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BearerAuth {
    pub token: String,
}

// Struct for ApiKeyAuth
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiKeyAuth {
    pub token: String,
}

// Struct for OAuth2Auth
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OAuth2Auth {
    pub client_id: String,
    pub scopes: Vec<String>,
    pub access_token: String,
    pub state: String,
    pub username: String,
    pub password: String,
}

// Struct for Heading
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Heading {
    pub depth: i32,
    pub value: String,
    pub slug: Option<String>,
}

// Struct for ScalarState
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScalarState {
    pub hash: Option<String>,
    pub authentication: Option<AuthenticationState>,
    pub collapsed_sidebar_items: Option<HashMap<String, bool>>,
    #[serde(flatten)]
    pub other: HashMap<String, serde_json::Value>,
}

// Struct for SSRState
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SSRState {
    pub payload: SSRPayload,
    pub url: String,
}

// Struct for SSRPayload
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SSRPayload {
    pub data: ScalarState,
}

// Struct for Tag
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tag {
    pub name: String,
    pub description: String,
    pub operations: Vec<TransformedOperation>,
    pub x_display_name: Option<String>,
}

// Struct for TagGroup
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TagGroup {
    pub name: String,
    pub tags: Vec<String>,
}

// Type alias for Definitions
pub type Definitions = OpenAPIV2::DefinitionsObject;

// Type alias for Webhooks
pub type Webhooks = HashMap<String, HashMap<String, TransformedOperation>>;

// Struct for Spec
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Spec {
    pub tags: Option<Vec<Tag>>,
    pub info: OpenAPIInfo,
    pub host: Option<String>,
    pub base_path: Option<String>,
    pub schemes: Option<Vec<String>>,
    pub external_docs: Option<ExternalDocs>,
    pub servers: Option<Vec<Server>>,
    pub components: Option<Components>,
    pub webhooks: Option<Webhooks>,
    pub definitions: Option<Definitions>,
    pub swagger: Option<String>,
    pub openapi: Option<String>,
    pub x_tag_groups: Option<Vec<TagGroup>>,
    pub security: Option<Vec<OpenAPIV3::SecurityRequirementObject>>,
}

// Enum for OpenAPIInfo
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OpenAPIInfo {
    V2(OpenAPIV2::InfoObject),
    V3(OpenAPIV3::InfoObject),
    V3_1(OpenAPIV3_1::InfoObject),
}

// Struct for ExternalDocs
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExternalDocs {
    pub url: String,
    pub description: Option<String>,
}

// Enum for Components
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Components {
    V3(OpenAPIV3::ComponentsObject),
    V3_1(OpenAPIV3_1::ComponentsObject),
}
