use serenity::model::prelude::{Channel, Role};

pub struct Group {
    role: Role,
    channel: Option<Channel>,
}
