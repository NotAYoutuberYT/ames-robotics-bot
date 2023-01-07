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

    // in group for rwlock safety
    {
        // get admin roles
        let bot_data = ctx.data.read().await;
        let admin_roles = bot_data
            .get::<AdminRoles>()
            .expect("no AdminRoles in TypeMap");

        let mut builder = MessageBuilder::new();
        builder.push("The roles with configuration permissions are the following:\n");

        for role in &admin_roles.admin_roles {
            builder.push_safe("\t").push(role);
        }

        message = builder.build();

        if admin_roles.admin_roles.len() == 0 {
            message = "There are no roles with configuration permissions!".to_owned();
        }
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
    command.name("list-configuration-roles").description(
        "lists all roles with configuration permissions (server owner can always configure bot)",
    )
}
