# nuntius

*nuntius* (Latin: "messenger") is a personal Discord bot written in Rust.

Its first job is to read the email newsletters I subscribe to, keep only the
items I actually care about, and deliver them to my Discord server. More
features will follow, but everything stays inside this one bot.

## Goals

- Filter incoming email newsletters by my own rules and post the survivors
  to Discord channels.
- Grow into a small personal assistant bot: one deployable binary, multiple
  features as internal modules.
- Keep every piece of personal information out of this repository.

## Architecture Principles

- **One repo, modular inside.** Features live as modules (later: workspace
  crates such as `ingest`, `filter`, `bot`) rather than separate
  repositories. A feature graduates to its own repo only when it becomes
  independently useful or needs a private boundary.
- **Code is public, configuration is private.** The public/private line is
  drawn between code and data, not between features. Filtering logic is
  code; *what* I filter is data.
- **Clean module boundary for mail access.** The mail-reading component
  (ingester) is isolated so it can later be split into a separate private
  process that only hands results to the bot, if credential isolation ever
  becomes necessary.

## Privacy Rules

- `config.toml`, `.env`, and `data/` are gitignored. They hold mail
  accounts, API credentials, the Discord token, channel IDs, and filter
  rules (filter keywords reveal personal interests — treat them as data,
  not code).
- Only `config.example.toml` with placeholder values is committed.
- No real newsletter content or email addresses in test fixtures.
- A token that ever lands in a commit is considered leaked and gets
  reissued.

## Planned Stack

- [serenity](https://github.com/serenity-rs/serenity) +
  [poise](https://github.com/serenity-rs/poise) for the Discord side
- [tokio](https://tokio.rs) async runtime
- Gmail API or `async-imap` for newsletter ingestion

## TODO

- [ ] Decide mail source: Gmail API vs IMAP
- [ ] Define `config.example.toml` schema (mail account, filter rules,
      channel routing)
- [ ] Bot skeleton: login, health check command
- [ ] Newsletter ingester module
- [ ] Filter engine (rule-driven, rules loaded from config/data)
- [ ] Discord delivery: formatting, channel routing
- [ ] Scheduling: periodic fetch loop
- [ ] Deployment target (local daemon vs small server)
- [ ] Next features (TBD)
