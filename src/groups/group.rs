use serenity::model::prelude::{Channel, Role};

pub struct Group {
    pub role: Role,
    pub channel: Option<Channel>,
}
