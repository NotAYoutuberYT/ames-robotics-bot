use serenity::{
    builder::CreateApplicationCommand,
    model::{
        application::interaction::application_command::ApplicationCommandInteraction,
        prelude::interaction::InteractionResponseType,
    },
    prelude::Context,
    utils::MessageBuilder,
    Error,
};

use crate::AdminRoles;

//
// run
//

pub async fn run(command: &ApplicationCommandInteraction, ctx: &Context) -> Result<(), Error> {
    let mut message: String;

    // attempts to extract the command's guild id and tells the user to
    // not use this command in dms if it can't find one
    let command_guild_id = match command.guild_id {
        Some(id) => id,
        None => return command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|response| response.content("This command must be used in a server!"))
        })
        .await
    };

    // get admin roles (because we're in a function,
    // the rwlock will close before any issues arise)
    let bot_data = ctx.data.read().await;
    let admin_roles = bot_data
        .get::<AdminRoles>()
        .expect("no AdminRoles in TypeMap");

    // this is what will be used to construct the message
    let mut builder = MessageBuilder::new();
    builder.push("The roles with elevated privileges are the following:\n");

    // this keeps track of if there are any admin roles in this guild
    let mut listed_roles = false;

    // iterate over all admin roles and add
    // all ones applicable to this guild to
    // the message being built
    for role in &admin_roles.admin_roles {
        if role.guild_id == command_guild_id {
            builder.push_safe("\t").push(role);
            listed_roles = true;
        }
    }

    message = builder.build();

    // if no roles have admin in this guild, change the message to reflect that
    if !listed_roles {
        message = "There are no roles with elevated privileges!".to_owned();
    }

    // interact to the command with a response containing the supplied messasge
    command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|response| response.content(message))
        })
        .await
}

//
// create
//

pub fn create(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("list-privileged-roles").description(
        "lists all roles with elevated privileges (server owner always has all privileges)",
    )
}
