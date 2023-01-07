use serenity::{
    builder::CreateApplicationCommand,
    json::Value,
    model::{
        application::interaction::application_command::ApplicationCommandInteraction,
        prelude::{command::CommandOptionType, interaction::InteractionResponseType, Role},
    },
    prelude::Context,
    utils::ArgumentConvert,
    Error,
};

use crate::AdminRoles;

//
// run
//

pub async fn run(command: &ApplicationCommandInteraction, ctx: &Context) -> Result<(), Error> {
    let role: Role;

    // extract the role
    if let Some(Value::String(role_id)) = &command.data.options[0].value {
        if let Ok(r) =
            Role::convert(ctx, command.guild_id, Some(command.channel_id), &role_id).await
        {
            // successfully extracted role
            role = r;
        } else {
            // couldn't get role from id
            return Err(Error::Other("invalid role id"));
        }
    } else {
        // couldn't get id
        return Err(Error::Other("failed to extract role id"));
    }

    let mut message: String = std::format!(
        "Successfully gave {} the ability to configure the bot.",
        role.name
    );

    // in group for rwlock safety
    {
        // get admin roles
        let mut bot_data = ctx.data.write().await;
        let admin_roles = bot_data
            .get_mut::<AdminRoles>()
            .expect("no AdminRoles in TypeMap");

        let has_perms: bool;

        // figure out if the user has perms (looks complicated, but just has a bunch of error handling)
        // the error handling can be moved elsewhere in the future
        if let Some(guild_id) = command.guild_id {
            match guild_id.to_partial_guild(&ctx.http).await {
                Ok(guild) => match admin_roles.user_has_admin(&command, &ctx, &guild).await {
                    Ok(result) => has_perms = result,
                    Err(e) => return Err(e),
                },
                Err(e) => return Err(e),
            }
        } else {
            // we failed to get a guild id
            return Err(Error::Other("failed to get a guild id"));
        }

        match has_perms {
            true => {
                if let Err(e) = admin_roles.add_role(&role) {
                    message = e.to_owned();
                }
            }
            false => message = "You are not allowed to do that!".to_owned(),
        }
    }

    // interact to the command with a response containing the supplied messasge
    command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                // message defined here
                .interaction_response_data(|response| response.content(message))
        })
        .await
}

//
// create
//

pub fn create(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("add-configuration-permissions")
        .description("allows the given role to configure the bot (any roles with administrator are allowed by default)")
        .create_option(|option| {
            // the option for what the bot will say
            option
                .kind(CommandOptionType::Role)
                .name("role")
                .description("the role to let configure the bot")
                .default_option(false)
                .required(true)
        })
}
