use serenity::{
    builder::CreateApplicationCommand,
    model::{
        application::interaction::application_command::ApplicationCommandInteraction,
        prelude::{interaction::InteractionResponseType, PartialGuild},
    },
    prelude::Context,
    utils::MessageBuilder,
    Error,
};

use crate::{extract_from_command::from_command_data::extract_partial_guild, Groups};

use super::group::Group;

//
// run
//

pub async fn run(command: &ApplicationCommandInteraction, ctx: &Context) -> Result<(), Error> {
    // attempts to extract the command's guild and tells the user to
    // not use this command in dms if it can't find one
    let message_guild: PartialGuild = match extract_partial_guild(&command, &ctx).await {
        Ok(guild) => guild,
        Err(_) => {
            return command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|response| {
                            response.content("This command must be used in a server!")
                        })
                })
                .await
        }
    };

    // get groups (because we're in a function,
    // the rwlock will close before any issues arise)
    let bot_data = ctx.data.read().await;
    let groups = bot_data.get::<Groups>().expect("no Groups in TypeMap");

    // this is what will be used to construct the message
    let mut builder = MessageBuilder::new();
    builder.push("The bot has the following groups:\n");

    let relevant_groups: Vec<&Group> = groups
        .iter()
        .filter(|group| group.guild.id == message_guild.id)
        .collect();

    for g in &relevant_groups {
        let group = (*g).clone();

        builder.push_safe(group.name);
        builder.push(": ");
        builder.push_safe(group.channel);

        match group.role {
            Some(role) => {
                builder.push_safe(", ");
                builder.push_safe(role);
            }
            None => (),
        }

        builder.push("\n");
    }

    // interact to the command with a response containing the supplied messasge
    let mut message = builder.build();

    if relevant_groups.len() == 0 {
        message = "There are no groups!".to_owned();
    }

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
    command.name("list-groups").description("lists all groups")
}
