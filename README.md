# do-alerts-discord

A shitty program to bridge DO resource alerts to Discord webhooks via email. Also the first """practical""" thing I've built in Rust.

![Screenshot of a Discord message containing an embed which reads "CPU is running high - CPU Utilization Percent is currently at 5.99%, above setting of 1.00% for the last 5m. View droplet (link). IP: [redacted]. Edit monitor (link). Today at 2:48 AM](image.png)

## Usage

Set up your DO monitoring alerts to notify via email. Provide IMAP credentials for the mailbox they'll be delivered to in the environment variables, along with the webhook URL from Discord where you want notifications delivered to.

`cargo run` is a thing. or you can use [the docker image](https://github.com/eritbh/do-alerts-discord/pkgs/container/do-alerts-discord) hey look i finally learned dockerfile enough to do some stupid unoptimized bullshit.

The program connects to your mail server via IMAP, looks for received DO notification emails, parses them and sends the information on to Discord, then permanently deletes the emails. It performs one check, forwards any notifications it finds, and exits. Run it periodically via `cron` or similar.

I run this via Docker on my unRAID box using the configuration in [the `examples` directory](/examples).

## Environment

`IMAP_DOMAIN`, `IMAP_PORT`, `IMAP_USER`, `IMAP_PASS` set authentication to your mail server. `DISCORD_WEBHOOK_URL` is obtained by creating a Discord webhook in the channel where you want messages to be delivered.

also all the code sux sorry. i gave myself one day to make this work and i set it up on my server and now im never touching it again
