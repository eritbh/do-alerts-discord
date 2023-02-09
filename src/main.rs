use chrono::TimeZone;
use dotenv::dotenv;
use regex_macro::regex;

#[derive(Debug)]
struct ReceivedMessage {
	date: i64,
	from: String,
	subject: String,
	body: String,
}

fn something(
	domain: String,
	port: u16,
	username: String,
	password: String,
) -> Vec<ReceivedMessage> {
	let client = imap::ClientBuilder::new(domain, port)
		.native_tls()
		.expect("Client should be created");

	let mut session = client
		.login(username, password)
		.map_err(|e| e.0)
		.expect("Session should log in");

	// Work in the inbox only
	session
		.select("INBOX")
		.expect("Session should select inbox");

	// Get sequence numbers for messages that match our query
	let seqs = session
		.search(r#"UNDELETED FROM "digitalocean.com" SUBJECT "monitoring triggered""#)
		.expect("Message search should return seqs");

	// If we have no messages matching the query, there's nothing to do
	if seqs.is_empty() {
		return vec![];
	}

	// Map the sequence numbers into a comma-separated string for future use
	let seqs_str = seqs
		.into_iter()
		.map(|n| n.to_string())
		.collect::<Vec<String>>()
		.join(",");

	println!("seqs: {}", seqs_str);

	// Fetch info about the messages
	let messages = session
		.fetch(&seqs_str, "(INTERNALDATE ENVELOPE BODY[TEXT])")
		.expect("there should be messages");

	// Delete stuff
	session
		.store(&seqs_str, r#"+FLAGS (\Deleted)"#)
		.expect("the messages should be marked deleted");

	// Map messages into their bodies
	messages
		.iter()
		.filter_map(|message| {
			let date = message.internal_date()?;

			let envelope = message.envelope().expect("message should have envelope");

			// let from = envelope
			// 	.from
			// 	.as_ref()
			// 	.expect("message should have a from field");
			// let from = from.first().expect("there should be at least one sender");

			let subject = envelope
				.subject
				.as_ref()
				.expect("message should have a subject");
			let subject = std::str::from_utf8(&subject)
				.expect("message subject should be utf-8")
				.to_string();

			let body = message.text().expect("message should have a text body");
			let body = std::str::from_utf8(body)
				.expect("body should be utf-8")
				.to_string()
				// handle weird MIME line continuiation bullshit
				.replace("\r\n", "\n")
				.replace("=\n", "");

			println!("{}", body);

			Some(ReceivedMessage {
				date: date.timestamp(),
				from: "".into(),
				subject,
				body,
			})
		})
		.collect::<Vec<ReceivedMessage>>()
}

fn main() {
	dotenv().ok();

	let domain = std::env::var("IMAP_DOMAIN").expect("IMAP_DOMAIN is required");
	let port = std::env::var("IMAP_PORT")
		.expect("IMAP_PORT is required")
		.parse::<u16>()
		.expect("IMAP_PORT must be a valid port number");
	let username = std::env::var("IMAP_USER").expect("IMAP_USER is required");
	let password = std::env::var("IMAP_PASS").expect("IMAP_PASSWORD is required");

	let webhook_url =
		std::env::var("DISCORD_WEBHOOK_URL").expect("DISCORD_WEBHOOK_URL is required");

	println!("Fetching messages...");
	let messages = something(domain, port, username, password);

	if messages.is_empty() {
		println!("No messages");
		return;
	}

	let client = reqwest::blocking::Client::new();
	println!("Listing...");
	for message in messages {
		// formatting specific to DO and Discord
		let subject = message
			.subject
			.replace("DigitalOcean monitoring triggered: ", "");
		let body = regex!(r"\n([^:\[\]]+): (https?://[^\n]+)\n")
			.replace_all(&message.body, "\n[$1]($2)\n");
		let datetime = chrono::Utc
			.timestamp_opt(message.date, 0)
			.unwrap()
			.to_rfc3339();

		client
			.post(&webhook_url)
			.json(&serde_json::json!({
				"embeds": [
					{
						"title": subject,
						"description": body,
						"timestamp": datetime
					},
				],
			}))
			.send()
			.expect("webhook request should succeed");
		println!("{:?}\n", message);
	}
}
