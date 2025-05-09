use colored::Colorize;
use discord_rich_presence::{
    activity::{self, Activity},
    DiscordIpc, DiscordIpcClient,
};
use std::{
    thread, time::{SystemTime, UNIX_EPOCH}, vec
};
use signal_hook::{consts::signal::*, iterator::Signals};

use crate::cli::Cli;

pub fn run(args: Cli) {
    let state = args.state;
    let details = args.details;
    let large_image = args.large_image;
    let large_text = args.large_text;
    let small_image = args.small_image;
    let small_text = args.small_text;
    let mut button_1_text = args.button_1_text;
    let button_1_url = args.button_1_url;
    let mut button_2_text = args.button_2_text;
    let button_2_url = args.button_2_url;
    let enable_timer = args.enable_time;

    let mut client = DiscordIpcClient::new(&args.client_id).expect("failed to create client");

    let activity = Activity::new();

    client
        .connect()
        .expect("failed to connect to Discord, please try again or relaunch Discord app");

    println!("{} {}", "details :".cyan(), details.yellow());

    let mut activity: Activity = activity.details(&details);

    if state != "none" {
        println!("{} {}", "state :".cyan(), state.yellow());

        activity = activity.state(&state);
    }

    let mut assets = activity::Assets::new();

    if large_image != "none" {
        println!("{} {}", "large image :".cyan(), large_image.yellow());

        assets = assets.large_image(&large_image);
    }

    if large_text != "none" {
        println!("{} {}", "large image text :".cyan(), large_text.yellow());

        assets = assets.large_text(&large_text);
    }

    if small_image != "none" {
        println!("{} {}", "small image :".cyan(), small_image.yellow());

        assets = assets.small_image(&small_image);
    }

    if small_text != "none" {
        println!("{} {}", "small image text :".cyan(), small_text.yellow());

        assets = assets.small_text(&small_text);
    }

    activity = activity.assets(assets);

	if button_1_text.len() > 31 {
		println!("{}", "button 1 text is too long, max length is 31 bytes".red());
		println!("{}", "button 1 will not be added".red());
		button_1_text = "none".to_string();
	}

    if button_1_text != "none" && button_1_url != "none" {
        println!("{} {}", "button 1 text :".cyan(), button_1_text.yellow());
        println!("{} {}", "button 1 url :".cyan(), button_1_url.yellow());

        activity = activity.buttons(vec![activity::Button::new(&button_1_text, &button_1_url)]);
    }

	if button_2_text.len() > 31 {
		println!("{}", "button 2 text is too long, max length is 31 bytes".red());
		println!("{}", "button 2 will not be added".red());
		button_2_text = "none".to_string();
	}

    if button_1_text != "none"
        && button_1_url != "none"
        && button_2_text != "none"
        && button_2_url != "none"
    {
        println!("{} {}", "button 2 text :".cyan(), button_2_text.yellow());
        println!("{} {}", "button 2 url :".cyan(), button_2_url.yellow());

        activity = activity.buttons(vec![
            activity::Button::new(&button_1_text, &button_1_url),
            activity::Button::new(&button_2_text, &button_2_url),
        ]);
    }

    if enable_timer {
        let time_unix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        activity = activity.timestamps(activity::Timestamps::new().start(time_unix))
    }

    client.set_activity(activity).expect("client set activity");

	thread::scope(|s| {
		s.spawn(|| {
			let mut signals = Signals::new(&[SIGINT, SIGTERM]).unwrap();
			for _ in signals.forever() {
				println!("{}", "Stopping due to signal...".blue());
				let _ = &client.close().expect("failed to close client");
				std::process::exit(0);
			}
		});
	});
}
