use serenity::{
    builder::CreateApplicationCommand,
    model::{
        application::interaction::application_command::ApplicationCommandInteraction,
        prelude::{command::CommandOptionType, interaction::InteractionResponseType, Role},
    },
    prelude::Context,
    utils::MessageBuilder,
    Error,
};

use crate::extract_from_command::{
    from_command_data::extract_partialguild, from_options::extract_role,
};
use crate::AdminRoles;

//
// run
//

pub async fn run(command: &ApplicationCommandInteraction, ctx: &Context) -> Result<(), Error> {
    // extract the role
    let role: Role = match extract_role(command, ctx, 0).await {
        Ok(r) => r,
        Err(e) => return Err(e),
    };

    // if no errors are encountered, this "default" message will
    // be the one sent at the end of the command
    let mut message: String = MessageBuilder::new()
        .push("Successfully removed ")
        .push_safe(&role)
        .push("'s privileges.")
        .build();

    // get admin roles
    let mut bot_data = ctx.data.write().await;
    let admin_roles = bot_data
        .get_mut::<AdminRoles>()
        .expect("no AdminRoles in TypeMap");

    let has_perms: bool;

    // figure out if the user has perms (looks complicated, but just has a bunch of error handling)
    match extract_partialguild(&command, &ctx).await {
        Ok(guild) => match admin_roles
            .command_author_has_admin(&command, &ctx, &guild)
            .await
        {
            Ok(result) => has_perms = result,
            Err(e) => return Err(e),
        },
        Err(e) => return Err(e),
    }

    // remove the roll from admin roles if the user has perms
    match has_perms {
        true => {
            if let Err(e) = admin_roles.remove_role(&role) {
                message = e.to_owned();
            }
        }
        false => message = "You are not allowed to do that!".to_owned(),
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
    command
        .name("remove-privileges")
        .description("removes elevated privileges from the given role")
        .create_option(|option| {
            // the option for what the bot will say
            option
                .kind(CommandOptionType::Role)
                .name("role")
                .description("the role to remove the privileges from")
                .default_option(false)
                .required(true)
        })
}
