use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

use std::io::{Cursor, Write};
use zip::ZipWriter;

#[command]
#[description = "Sends a `.zip` archive containing the attachments."]
fn zip(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id.broadcast_typing(&ctx)?;

    let mut zip = ZipWriter::new(Cursor::new(Vec::new()));

    for attachment in &msg.attachments {
        zip.start_file(&*attachment.filename, Default::default())?;
        zip.write_all(&attachment.download()?)?;
    }

    msg.channel_id.send_files(
        &ctx,
        vec![(zip.finish()?.into_inner().as_slice(), "output.zip")],
        |m| m,
    )?;

    Ok(())
}
