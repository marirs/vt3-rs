use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Collection<T: Serialize> {
	#[serde(bound(deserialize = "Vec<Data<T>>: Deserialize<'de>"))]
	pub data: Vec<Data<T>>,
	pub links: Option<Links>,
	pub meta: Option<Meta>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Object<T: Serialize> {
	#[serde(bound(deserialize = "Data<T>: Deserialize<'de>"))]
	pub data: Data<T>,
}

impl<T> Object<T>
where
	T: Serialize,
{
	pub fn build(object_type: Option<String>, attributes: Option<T>, id: Option<String>) -> Self {
		Self {
			data: Data {
				object_type,
				attributes,
				id,
				links: None,
			},
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data<T: Serialize> {
	#[serde(rename = "type")]
	pub object_type: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	#[serde(bound(deserialize = "Option<T>: Deserialize<'de>"))]
	pub attributes: Option<T>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub links: Option<Links>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Links {
	#[serde(rename = "self")]
	link_self: Option<String>,

	#[serde(rename = "next")]
	link_next: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
	count: Option<i32>,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct DataId {
	#[serde(rename = "type")]
	pub object_type: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
}
