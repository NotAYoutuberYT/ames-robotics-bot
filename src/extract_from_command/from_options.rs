use serenity::{
    json::Value,
    model::prelude::{
        interaction::application_command::ApplicationCommandInteraction, Channel, Role,
    },
    prelude::Context,
    utils::ArgumentConvert,
    Error,
};

pub async fn extract_role(
    command: &ApplicationCommandInteraction,
    ctx: &Context,
    option_index: usize,
) -> Result<Role, Error> {
    if let Some(Value::String(role_id)) = &command.data.options[option_index].value {
        // successfully extracted id
        if let Ok(role) =
            Role::convert(ctx, command.guild_id, Some(command.channel_id), role_id).await
        {
            // successfully extracted role
            Ok(role)
        } else {
            // couldn't get role from id
            Err(Error::Other(
                "failed to get a role from an id supplied from a command",
            ))
        }
    } else {
        // couldn't get id
        Err(Error::Other(
            "failed to extract role id from command options",
        ))
    }
}

pub async fn extract_channel(
    command: &ApplicationCommandInteraction,
    ctx: &Context,
    option_index: usize,
) -> Result<Channel, Error> {
    if let Some(Value::String(channel_id)) = &command.data.options[option_index].value {
        // successfully extracted id
        if let Ok(channel) =
            Channel::convert(ctx, command.guild_id, Some(command.channel_id), channel_id).await
        {
            // successfully extracted channel
            Ok(channel)
        } else {
            // couldn't get channel from id
            Err(Error::Other(
                "failed to get a channel from an id supplied from a command",
            ))
        }
    } else {
        // couldn't get id
        Err(Error::Other(
            "failed to extract channel id from command options",
        ))
    }
}
