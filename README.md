# naoto-bot [![guild-badge][]][guild] [![invite-badge][]][invite]

<img align="right" src="./assets/naoto.png">

A Discord bot written in Rust using [Serenity].

This bot is a work-in-progress.

## Config
To set the token (and other settings), you need to create a file named `config.toml`.

Inside of it you put the following line:
```toml
token = "<bot token>"
```

Other settings are currently unsupported.

Self hosting isn't advised. Custom bot emojis won't appear as intended.

[guild]: https://discord.gg/ZJvqBK7
[guild-badge]: https://img.shields.io/discord/516256587694866452.svg?style=flat-square&colorB=7289DA

[invite]: https://discordapp.com/oauth2/authorize?client_id=494235198582423552&permissions=8&scope=bot
[invite-badge]: https://img.shields.io/badge/naoto-invite-black.svg?style=flat-square&colorB=7289DA

[Serenity]: https://github.com/serenity-rs/serenity
