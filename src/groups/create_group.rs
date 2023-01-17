use serenity::{
    builder::CreateApplicationCommand,
    json::Value,
    model::{
        application::interaction::application_command::ApplicationCommandInteraction,
        prelude::{
            command::CommandOptionType, interaction::InteractionResponseType, Channel,
            PartialGuild, Role, ChannelType,
        },
    },
    prelude::Context,
    utils::MessageBuilder,
    Error,
};

use crate::{
    extract_from_command::{
        from_command_data::extract_partialguild,
        from_options::{extract_channel, extract_role},
    },
    AdminRoles, Groups,
};

use super::group::Group;

//
// run
//

pub async fn run(command: &ApplicationCommandInteraction, ctx: &Context) -> Result<(), Error> {
    let mut bot_data = ctx.data.write().await;

    // put the permission handling inside a group in order
    // to close the bot data rwlock when done with it
    {
        let admin_roles = bot_data
            .get::<AdminRoles>()
            .expect("no AdminRoles in TypeMap");

        // figure out if the user has perms (looks complicated, but just has a bunch of error handling)
        let has_perms: bool;

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

        // attempts to extract the command's guild id and tells the user to
        // not use this command in dms if it can't find one
        if !has_perms {
            return command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|response| {
                            response.content("You do not have permission to use this command!")
                        })
                })
                .await;
        };
    }

    // borrow the bot's groups at mutable
    // rwlock will close on function exit
    let groups = bot_data.get_mut::<Groups>().expect("no Groups in TypeMap");

    // extract and store the role and channel
    let name: String;
    let channel: Channel;
    let role: Option<Role>;
    let guild: PartialGuild;

    match &command.data.options[0].value {
        Some(Value::String(n)) => name = n.to_owned(),
        _ => return Err(Error::Other("didn't recieve a name for a new group")),
    }

    match extract_channel(&command, &ctx, 1).await {
        Ok(c) => channel = c,
        Err(e) => return Err(e),
    }

    if command.data.options.len() > 2 {
        match extract_role(&command, &ctx, 2).await {
            Ok(r) => role = Some(r),
            Err(e) => return Err(e),
        }
    } else {
        role = None;
    }

    match extract_partialguild(&command, &ctx).await {
        Ok(g) => guild = g,
        Err(e) => return Err(e),
    }

    // create a new group and insert it into the groups struct
    let group = Group {
        name: name,
        role: role,
        channel: channel.clone(),
        todos: Vec::new(),
        guild: guild,
    };

    let mut builder = MessageBuilder::new();

    builder.push("Successfully created group \"");
    builder.push_safe(&group.name);
    builder.push("\"!");

    let mut message = builder.build();

    // ensures that the new group doesn't overlap
    // with any old groups
    let mut can_exist = true;
    for g in groups.clone() {
        if group.overlap(&g) {
            message = "Another group is using the same role or channel!".to_owned();
            can_exist = false;
        }
    }

    // makes sure the channel the group uses is a text channel
    if let Some(guild_channel) = channel.guild() {
        if guild_channel.kind != ChannelType::Text {
            message = "Groups must be created on text channels!".to_owned();
            can_exist = false;
        }
    } else {
        message = "That's not a valid channel!".to_owned();
        can_exist = false;
    }

    // adds the group to the bot's saved groups if allowed
    if can_exist {
        groups.push(group.clone());
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
        .name("create-group")
        .description("creates a new group")
        .create_option(|option| {
            option
                .kind(CommandOptionType::String)
                .name("name")
                .description("the name of the group")
                .default_option(false)
                .required(true)
        })
        .create_option(|option| {
            // channel option
            option
                .kind(CommandOptionType::Channel)
                .name("channel")
                .description("the channel todos for this group must be created and checked from")
                .default_option(false)
                .required(true)
        })
        .create_option(|option| {
            // role option
            option
                .kind(CommandOptionType::Role)
                .name("role")
                .description("the role the group will be attached to (defaults to everyone)")
                .default_option(false)
                .required(false)
        })
}
