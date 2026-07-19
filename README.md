# nuntius

*nuntius* (Latin: 'messenger') is a personal Discord bot written in Rust.

It watches the streams I care about — my inbox, my newsletters, and the
sites I follow — and delivers filtered summaries to my Discord server.

## Goals

Three feature lanes, all delivering into Discord channels:

1. **Important mail digest** — detect mail that matters (by my own rules)
   and post a short summary instead of letting it drown in the inbox.
2. **Newsletter summaries** — filter incoming newsletters by my rules and
   summarize the surviving items.
3. **Watched-site updates** — track sites of interest (RSS/Atom where
   available, scraping as a fallback) and summarize what changed.

Beyond that:

- Grow into a small personal assistant bot: one deployable binary, multiple
  features as internal modules.
- Keep every piece of personal information out of this repository.

## Workspace Layout

The repository stays as one Rust workspace while Discord ingress, sessions,
permissions, and local-model adapters are shared. A domain crate can move to
another repository later if it needs an independent release cycle, deployment,
credential boundary, or reusable public API.

```text
nuntius/
├── src/                         deployable application entry point
└── crates/
    ├── lingua/                  implemented language-learning policy
    ├── litterae/                reserved literature boundary
    ├── ratio/                   reserved mathematics, science, engineering boundary
    └── fabrica/                 reserved hardware-software systems boundary
```

### English foundation

`lingua` currently implements backend-independent policy for:

- OPIc and integrated-English channel definitions without numeric Discord
  name prefixes;
- equal initial interest bands for the primary robotics and
  hardware-firmware-software domain and general STEM;
- privacy-bounded cross-space interest reuse;
- OPIc scope filtering before interest-based example selection; and
- local-first pronunciation stages, focused feedback, auxiliary-only ASR
  evidence, and session-scoped raw audio retention.

`litterae`, `ratio`, and `fabrica` contain no domain implementation yet. Their
minimal crates reserve workspace boundaries only. Discord transport,
persistence, audio capture, speech models, and LLM adapters are intentionally
outside `lingua` and remain upcoming integration work.

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
- Summarization sends content to an external LLM API. Decide per lane what
  may leave the machine: newsletters and public sites are fine; important
  personal mail needs an explicit opt-in list or local-only summarization.

## Planned Stack

- [serenity](https://github.com/serenity-rs/serenity) +
  [poise](https://github.com/serenity-rs/poise) for the Discord side
- [tokio](https://tokio.rs) async runtime
- Gmail API or `async-imap` for mail ingestion
- An LLM API for summarization (candidate: Anthropic API)
- RSS/Atom feed parsing for watched sites

## TODO

Foundation:

- [x] Convert the repository to a Rust workspace
- [x] Add backend-independent English-learning policy crate
- [ ] Decide mail source: Gmail API vs IMAP
- [ ] Define `config.example.toml` schema (mail account, filter rules,
      watched sites, channel routing)
- [ ] Bot skeleton: login, health check command
- [ ] Discord delivery: formatting, channel routing
- [ ] Scheduling: periodic fetch loop
- [ ] Summarizer module (shared by all lanes) + per-lane privacy policy

Lane 1 — important mail digest:

- [ ] Mail ingester module
- [ ] Importance rules (sender/subject/keyword driven, loaded from config)
- [ ] Digest formatting and delivery

Lane 2 — newsletter summaries:

- [ ] Newsletter detection and filter engine
- [ ] Item extraction and summary delivery

Lane 3 — watched-site updates:

- [ ] Feed/site watcher with change detection
- [ ] Update summaries and delivery

Later:

- [ ] Connect `lingua` to Discord threads and voice attachments
- [ ] Add SQLite persistence for English sessions, interests, and progress
- [ ] Add structured local speech and pronunciation adapters
- [ ] Deployment target (local daemon vs small server)
- [ ] Next features (TBD)
