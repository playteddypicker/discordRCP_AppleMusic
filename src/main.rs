use discord_rich_presence::{activity, new_client, DiscordIpc};
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = new_client("940199722138890250")?;
    loop {
        if client.connect().is_ok() {
            println!("logged in");
            break;
        } else {
            println!("login failed");
        }
    }

    loop {
        //노래가 현재 재생중인지 체크
        let status_command = Command::new("osascript")
            .args(["./src/curstatus.scpt"])
            .output()
            .expect("failed to execute process");
        let status = String::from_utf8(status_command.stdout).expect("nope");

        if status == "playing\n" {
            //현재 플레이 정보 가져오기
            //제목 아티스트 앨범
            let get_current_playing_info = Command::new("osascript")
                .args(["./src/musicinfo.scpt"])
                .output()
                .expect("failed to execute process");

            let to_current_playing_info =
                String::from_utf8(get_current_playing_info.stdout).expect("nope");

            let current_playing_info = to_current_playing_info.split(", ").collect::<Vec<&str>>();
            let full_info = format!("{} - {}", current_playing_info[0], current_playing_info[1]);

            let start = &current_playing_info[5][0..&current_playing_info[5].len() - 2]
                .parse::<f64>()
                .unwrap()
                .round();
            let end = &current_playing_info[4][0..current_playing_info[4].len()]
                .parse::<f64>()
                .unwrap()
                .round();
            println!("{:?} : {:?} - {:?}", current_playing_info[0], start, end);
            let payload = activity::Activity::new()
                .state(current_playing_info[2])
                .details(full_info.as_str())
                .assets(
                    activity::Assets::new()
                        .large_image("./src/logo.png")
                        .large_text(current_playing_info[1]),
                )
                .timestamps(
                    activity::Timestamps::new()
                        .start(*start as i64)
                        .end(*end as i64),
                );

            if client.set_activity(payload).is_err() && client.reconnect().is_ok() {
                continue;
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    #[allow(unreachable_code)]
    Ok(())
}
