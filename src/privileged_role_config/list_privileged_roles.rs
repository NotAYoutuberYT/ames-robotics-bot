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

    // get admin roles
    let bot_data = ctx.data.read().await;
    let admin_roles = bot_data
        .get::<AdminRoles>()
        .expect("no AdminRoles in TypeMap");

    let mut builder = MessageBuilder::new();
    builder.push("The roles with elevated privileges are the following:\n");

    for role in &admin_roles.admin_roles {
        builder.push_safe("\t").push(role);
    }

    message = builder.build();

    if admin_roles.admin_roles.is_empty() {
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
