use serenity::model::prelude::{Channel, PartialGuild, Role};

use crate::Todo;

#[derive(Clone)]
pub struct Group {
    pub name: String,
    pub role: Option<Role>,
    pub channel: Channel,
    pub todos: Vec<Todo>,
    pub guild: PartialGuild,
}

impl Group {
    // returns if two groups aren't allowed to coexist
    pub fn overlap(&self, other: &Group) -> bool {
        // as far as I'm aware, if lets don't
        // allow for logical and, so we get this
        let mut role_overlap = false;
        if let Some(role1) = &self.role {
            if let Some(role2) = &other.role {
                if role1.id == role2.id {
                    role_overlap = false;
                }
            }
        }

        (self.channel.id() == other.channel.id() || role_overlap) && self.guild.id == other.guild.id
    }
}
