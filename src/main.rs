mod tab_session_manager;
use color_eyre::eyre::Context;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    // Read STG JSON file as input
    let text = read_input()?;

    let result = serde_json::from_str::<tab_session_manager::de::TSMConfig>(&text)?;

    for session in result {
        if session.check_valid_tabs() {
            println!("Valid window");
            println!("Session: {:?}", session);
        } else {
            println!("Inalid window")
        }
    }
    Ok(())
}

fn read_input() -> color_eyre::Result<String> {
    let input_file = "src/demo_data/tsm.json";
    let input =
        std::fs::read_to_string(input_file).wrap_err(format!("Error reading {}", input_file))?;

    // Ignore BOM
    // let char1 = input.chars().next().unwrap();
    // if char1 == '\u{0306}' {
    //     input = input.strip_prefix('\u{FEFF}').unwrap().to_string()
    // }

    Ok(input)
}
