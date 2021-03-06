#![cfg(test)]
use super::*;

#[test]
fn test_json_serialization() {
    // Read in a specific file
    let toml_filepath: PathBuf = ["test_files", "test_games.toml"].iter().collect();
    let games: HashMap<String, Game> = toml_to_hashmap(&toml_filepath).unwrap();

    // serialize as json
    let json_object_touhou = serde_json::to_string(&games.get("touhou_123")).unwrap();
    let json_object_melty_blood = serde_json::to_string(&games.get("melty_blood")).unwrap();

    // test cases separately to get around the nondeterministic order for hashmap
    let test_json_touhou = "{\"name\":\"Touhou\",\
                            \"description\":\"bullet hell with waifus\",\
                            \"genres\":[\"bullet hell\",\"anime\"],\
                            \"thumbnail_path\":\"path/to/touhou/thumbnail\"}";
    let test_json_mb = "{\"name\":\"Melty Blood\",\
                        \"description\":\"fighter with waifus\",\
                        \"genres\":[\"fighter\",\"anime\",\"2d\"],\
                        \"thumbnail_path\":\"path/to/melty_blood/thumbnail\"}";

    assert_eq!(json_object_touhou, test_json_touhou);
    assert_eq!(json_object_melty_blood, test_json_mb);
}

#[test]
fn test_games_serialization() {
    // Read in a specific file
    let toml_filepath: PathBuf = ["test_files", "test_games.toml"].iter().collect();
    let games: HashMap<String, Game> = toml_to_hashmap(&toml_filepath).unwrap();

    let games_clone = games.clone();

    // wrap all the games in a mutex
    // note that this moves games into the mutex
    let games_data = Arc::new(Mutex::new(games));

    assert_eq!(games_clone, *games_data.lock().unwrap());
}

#[test]
fn test_read_toml() {
    // Read in a specific file
    let toml_filepath: PathBuf = ["test_files", "test_games.toml"].iter().collect();
    let games: HashMap<String, Game> = toml_to_hashmap(&toml_filepath).unwrap();

    let mut test_games: HashMap<String, Game> = HashMap::new();
    test_games.insert(
        "touhou_123".to_owned(),
        Game {
            name: "Touhou".to_owned(),
            description: "bullet hell with waifus".to_owned(),
            genres: vec!["bullet hell".to_owned(), "anime".to_owned()],
            thumbnail_path: PathBuf::from(r"path\to\touhou\thumbnail"),
            exe_path: PathBuf::from(r"test_files\touhou_game.exe"),
            exe_args: vec!["arg1".to_owned(), "arg2".to_owned()],
        },
    );

    test_games.insert(
        "melty_blood".to_owned(),
        Game {
            name: "Melty Blood".to_owned(),
            description: "fighter with waifus".to_owned(),
            genres: vec!["fighter".to_owned(), "anime".to_owned(), "2d".to_owned()],
            thumbnail_path: PathBuf::from(r"path\to\melty_blood\thumbnail"),
            exe_path: PathBuf::from(r"test_files\melty_blood_game.exe"),
            exe_args: vec!["arg1".to_owned(), "arg2".to_owned()],
        },
    );
    assert_eq!(games, test_games);
}
