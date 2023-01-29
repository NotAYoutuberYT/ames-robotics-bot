use serenity::{
    model::prelude::{
        interaction::application_command::ApplicationCommandInteraction, PartialGuild, Role,
    },
    prelude::Context,
    Error,
};

// simple struct that contains a vector of roles
pub struct AdminRoleList {
    // because roles (id's and all) are unique across guilds,
    // we can store admin roles for all guilds in a single vector
    // and everything just... works
    pub admin_roles: Vec<Role>,
}

impl AdminRoleList {
    pub fn new() -> AdminRoleList {
        AdminRoleList {
            admin_roles: Vec::new(),
        }
    }

    // if this returns an error, it should be delivered to the user instead of
    // actually being considered and error and logged
    pub fn add_role(&mut self, role: &Role) -> Result<(), &'static str> {
        // if the role already has permissions, return an error
        if self.admin_roles.contains(role) {
            return Err("That role already has elevated privileges!");
        }

        self.admin_roles.push(role.clone());

        Ok(())
    }

    // if this returns an error, it should be delivered to the user instead of
    // actually being considered and error and logged
    pub fn remove_role(&mut self, role: &Role) -> Result<(), &'static str> {
        // if this role doesn't already have perms, return an error
        if !self.admin_roles.contains(role) {
            return Err("That role doesn't have elevated privileges!");
        }

        // remove the element
        self.admin_roles.retain(|i| i != role);

        Ok(())
    }

    // checks if the person who
    // sent the command has admin
    pub async fn command_author_has_admin(
        &self,
        command: &ApplicationCommandInteraction,
        ctx: &Context,
        guild: &PartialGuild,
    ) -> Result<bool, Error> {
        // see if the user is the server owner
        let is_server_owner = command.user.id == guild.owner_id;

        // see if the user has any admin roles
        let mut has_admin = false;

        for admin_role in &self.admin_roles {
            let has_admin_role = command
                .user
                .has_role(&ctx.http, guild.clone(), admin_role)
                .await?;

            // if we successfully checked if the user has a role,
            // if they do have the admin role, set has_admin_role
            // to true. otherwise, return the error
            if has_admin_role {
                has_admin = true;
            }
        }

        Ok(is_server_owner || has_admin)
    }
}
