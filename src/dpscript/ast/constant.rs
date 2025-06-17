use crate::util::Location;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ConstantNode {
    pub name: String,
    pub ty: Option<String>,
    pub location: Location,
    pub value: Vec<Node>,
}
