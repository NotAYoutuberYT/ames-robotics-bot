use serenity::{
    model::prelude::{
        interaction::application_command::ApplicationCommandInteraction, PartialGuild, Role,
    },
    prelude::Context,
    Error,
};

pub struct AdminRoleList {
    pub admin_roles: Vec<Role>,
}

impl AdminRoleList {
    pub fn new() -> AdminRoleList {
        AdminRoleList {
            admin_roles: Vec::new(),
        }
    }

    pub fn add_role(&mut self, role: &Role) -> Result<(), &'static str> {
        // if the role already has permissions, return an error
        if self.admin_roles.contains(role) {
            return Err("That role already has configuration privileges!");
        }

        self.admin_roles.push(role.clone());

        Ok(())
    }

    pub fn remove_role(&mut self, role: &Role) -> Result<(), &'static str> {
        // if this role doesn't already have perms, return an error
        if !self.admin_roles.contains(role) {
            return Err("That role doesn't have configuration privileges!");
        }

        // remove the element
        self.admin_roles.retain(|i| i != role);

        Ok(())
    }

    pub async fn user_has_admin(
        &self,
        command: &ApplicationCommandInteraction,
        ctx: &Context,
        guild: &PartialGuild,
    ) -> Result<bool, Error> {
        let is_server_owner: bool;
        let mut has_admin_role = false;

        // see if the user is the server owner
        is_server_owner = command.user.id == guild.owner_id;

        // see if the user is in the admin roles list
        for admin_role in &self.admin_roles {
            let has_role = command
                .user
                .has_role(&ctx.http, guild.clone(), admin_role)
                .await;

            // if we successfully checked if the user has a role,
            // if they do have the admin role, set has_admin_role
            // to true. otherwise, return the error
            match has_role {
                Ok(result) => {
                    if result == true {
                        has_admin_role = true;
                    }
                }
                Err(e) => return Err(e),
            }
        }

        Ok(is_server_owner || has_admin_role)
    }
}
