use serenity::{
    json::Value,
    model::{
        application::interaction::application_command::ApplicationCommandInteraction, prelude::Role,
    },
    prelude::Context,
    utils::ArgumentConvert,
    Error,
};

pub async fn extract_role(
    command: &ApplicationCommandInteraction,
    ctx: &Context,
) -> Result<Role, Error> {
    if let Some(Value::String(role_id)) = &command.data.options[0].value {
        if let Ok(role) =
            Role::convert(ctx, command.guild_id, Some(command.channel_id), &role_id).await
        {
            // successfully extracted role
            return Ok(role);
        } else {
            // couldn't get role from id
            return Err(Error::Other("invalid role id"));
        }
    } else {
        // couldn't get id
        return Err(Error::Other("failed to extract role id"));
    }
}
