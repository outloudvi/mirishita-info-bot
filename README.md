# Mirishita Info Bot

[![Bot link](https://img.shields.io/badge/Telegram-%40mirishita__info__bot-gold.svg?style=flat-square)](https://t.me/mirishita_info_bot) [![Crates.io](https://img.shields.io/crates/v/mirishita-info-bot?style=flat-square)](https://crates.io/crates/mirishita_info_bot) [![Bot Usage Guide](https://img.shields.io/docsrs/mirishita_info_bot?label=Bot%20usage%20guide&style=flat-square)](https://docs.rs/mirishita_info_bot/latest/mirishita_info_bot/cmd/index.html)

**[Usage Guide](https://docs.rs/mirishita_info_bot/latest/mirishita_info_bot/cmd/index.html)**

A Mirishita (ミリシタ) information bot based on [Princess by matsurihi.me](https://api.matsurihi.me/docs/).

## Disclaimer

This can be seen as a sample project to write Telegram bots with [`worker-rs`](https://github.com/cloudflare/workers-rs/tree/HEAD/worker) over Cloudflare Workers. However, it should be stressed that `worker-rs` is **NOT** yet production ready:

* [Lack of scheduled events](https://github.com/cloudflare/workers-rs/issues/53)
* [Cannot send requests with FormData](https://github.com/cloudflare/workers-rs/issues/79)

## License

MIT

\* The boilerplate is provided by Cloudflare under Apache License.