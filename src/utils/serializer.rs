use mongodb::bson::oid::ObjectId;
use serde::Serializer;

pub fn serialize_option_object_id<S>(
    id: &Option<ObjectId>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match id {
        Some(object_id) => serializer.serialize_str(&object_id.to_hex()),
        None => serializer.serialize_none(),
    }
}
