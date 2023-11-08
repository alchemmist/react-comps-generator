mod ask;
use ask::AskParams;

mod file_creator;
use file_creator::FileCreator;

mod element;


fn check_os() -> String {
    if cfg!(windows) {
        return "\\".to_string();
    } else if cfg!(unix) {
        return "/".to_string();
    }
    println!("Не знаю такой системы!");
    return "???".to_string()
}


fn main() {
    let asker = AskParams::new(check_os());
    let file_creator = FileCreator::new(asker.ask());

    asker.ask_ok(&file_creator.get_created_files());
    file_creator.create();
}
