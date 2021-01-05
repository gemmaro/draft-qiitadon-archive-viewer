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
pub struct Outbox {
    #[serde(rename = "@context")]
    pub(crate) context: Vec<ContextElement>,

    #[serde(rename = "id")]
    pub(crate) id: String,

    #[serde(rename = "type")]
    pub(crate) outbox_type: String,

    #[serde(rename = "totalItems")]
    pub(crate) total_items: i64,

    #[serde(rename = "orderedItems")]
    pub(crate) ordered_items: Vec<OrderedItem>,
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
pub struct OrderedItem {
    #[serde(rename = "id")]
    pub(crate) id: String,

    #[serde(rename = "type")]
    pub(crate) ordered_item_type: OrderedItemType,

    #[serde(rename = "actor")]
    pub(crate) actor: String,

    #[serde(rename = "published")]
    pub(crate) published: String,

    #[serde(rename = "to")]
    pub(crate) to: Vec<String>,

    #[serde(rename = "cc")]
    pub(crate) cc: Vec<String>,

    #[serde(rename = "object")]
    pub(crate) object: ObjectUnion,

    #[serde(rename = "atomUri")]
    pub(crate) atom_uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectClass {
    #[serde(rename = "id")]
    pub(crate) id: String,

    #[serde(rename = "type")]
    pub(crate) object_type: ObjectType,

    #[serde(rename = "summary")]
    pub(crate) summary: Option<String>,

    #[serde(rename = "content")]
    pub(crate) content: String,

    #[serde(rename = "inReplyTo")]
    pub(crate) in_reply_to: Option<String>,

    #[serde(rename = "published")]
    pub(crate) published: String,

    #[serde(rename = "url")]
    pub(crate) url: String,

    #[serde(rename = "attributedTo")]
    pub(crate) attributed_to: String,

    #[serde(rename = "to")]
    pub(crate) to: Vec<String>,

    #[serde(rename = "cc")]
    pub(crate) cc: Vec<String>,

    #[serde(rename = "sensitive")]
    pub(crate) sensitive: bool,

    #[serde(rename = "atomUri")]
    pub(crate) atom_uri: String,

    #[serde(rename = "inReplyToAtomUri")]
    pub(crate) in_reply_to_atom_uri: Option<String>,

    #[serde(rename = "conversation")]
    pub(crate) conversation: String,

    #[serde(rename = "attachment")]
    pub(crate) attachment: Vec<Attachment>,

    #[serde(rename = "tag")]
    pub(crate) tag: Vec<Tag>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attachment {
    #[serde(rename = "type")]
    pub(crate) attachment_type: AttachmentType,

    #[serde(rename = "mediaType")]
    pub(crate) media_type: MediaType,

    #[serde(rename = "url")]
    pub(crate) url: String,

    #[serde(rename = "name")]
    pub(crate) name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    #[serde(rename = "type")]
    pub(crate) tag_type: TagType,

    #[serde(rename = "href")]
    pub(crate) href: Option<String>,

    #[serde(rename = "name")]
    pub(crate) name: String,

    #[serde(rename = "id")]
    pub(crate) id: Option<String>,

    #[serde(rename = "updated")]
    pub(crate) updated: Option<String>,

    #[serde(rename = "icon")]
    pub(crate) icon: Option<Attachment>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContextElement {
    ContextClass(ContextClass),

    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ObjectUnion {
    ObjectClass(ObjectClass),

    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AttachmentType {
    #[serde(rename = "Document")]
    Document,

    #[serde(rename = "Image")]
    Image,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MediaType {
    #[serde(rename = "image/jpeg")]
    ImageJpeg,

    #[serde(rename = "image/png")]
    ImagePng,

    #[serde(rename = "video/mp4")]
    VideoMp4,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ObjectType {
    #[serde(rename = "Note")]
    Note,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TagType {
    #[serde(rename = "Emoji")]
    Emoji,

    #[serde(rename = "Hashtag")]
    Hashtag,

    #[serde(rename = "Mention")]
    Mention,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OrderedItemType {
    #[serde(rename = "Announce")]
    Announce,

    #[serde(rename = "Create")]
    Create,
}
