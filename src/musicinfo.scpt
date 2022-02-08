tell application "Music"
    get {name, artist, album, year, duration} of current track & {player position}
end tell
