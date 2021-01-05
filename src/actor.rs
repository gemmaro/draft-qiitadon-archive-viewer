// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

extern crate serde_derive;

#[derive(Debug, Serialize, Deserialize)]
pub struct Actor {
    #[serde(rename = "@context")]
    pub(crate) context: Vec<ContextElement>,

    #[serde(rename = "id")]
    pub(crate) id: String,

    #[serde(rename = "type")]
    pub(crate) actor_type: String,

    #[serde(rename = "following")]
    pub(crate) following: String,

    #[serde(rename = "followers")]
    pub(crate) followers: String,

    #[serde(rename = "inbox")]
    pub(crate) inbox: String,

    #[serde(rename = "outbox")]
    pub(crate) outbox: String,

    #[serde(rename = "featured")]
    pub(crate) featured: String,

    #[serde(rename = "preferredUsername")]
    pub(crate) preferred_username: String,

    #[serde(rename = "name")]
    pub(crate) name: String,

    #[serde(rename = "summary")]
    pub(crate) summary: String,

    #[serde(rename = "url")]
    pub(crate) url: String,

    #[serde(rename = "manuallyApprovesFollowers")]
    pub(crate) manually_approves_followers: bool,

    #[serde(rename = "publicKey")]
    pub(crate) public_key: PublicKey,

    #[serde(rename = "endpoints")]
    pub(crate) endpoints: Endpoints,

    #[serde(rename = "icon")]
    pub(crate) icon: Icon,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContextClass {
    #[serde(rename = "manuallyApprovesFollowers")]
    pub(crate) manually_approves_followers: String,

    #[serde(rename = "sensitive")]
    pub(crate) sensitive: String,

    #[serde(rename = "movedTo")]
    pub(crate) moved_to: String,

    #[serde(rename = "Hashtag")]
    pub(crate) hashtag: String,

    #[serde(rename = "ostatus")]
    pub(crate) ostatus: String,

    #[serde(rename = "atomUri")]
    pub(crate) atom_uri: String,

    #[serde(rename = "inReplyToAtomUri")]
    pub(crate) in_reply_to_atom_uri: String,

    #[serde(rename = "conversation")]
    pub(crate) conversation: String,

    #[serde(rename = "toot")]
    pub(crate) toot: String,

    #[serde(rename = "Emoji")]
    pub(crate) emoji: String,

    #[serde(rename = "focalPoint")]
    pub(crate) focal_point: FocalPoint,

    #[serde(rename = "featured")]
    pub(crate) featured: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FocalPoint {
    #[serde(rename = "@container")]
    pub(crate) container: String,

    #[serde(rename = "@id")]
    pub(crate) id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Endpoints {
    #[serde(rename = "sharedInbox")]
    pub(crate) shared_inbox: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Icon {
    #[serde(rename = "type")]
    pub(crate) icon_type: String,

    #[serde(rename = "mediaType")]
    pub(crate) media_type: String,

    #[serde(rename = "url")]
    pub(crate) url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicKey {
    #[serde(rename = "id")]
    pub(crate) id: String,

    #[serde(rename = "owner")]
    pub(crate) owner: String,

    #[serde(rename = "publicKeyPem")]
    pub(crate) public_key_pem: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContextElement {
    ContextClass(ContextClass),

    String(String),
}
