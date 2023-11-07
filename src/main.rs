use std::fs::File;
use std::io::{prelude::*, self};


struct Element { // Element is a object for component or page
    path: str,
    name: str
}


struct AskParams {
    _element: Element
}

impl AskParams {
    fn ask(&self) -> Element {
        return self._element
    }


    fn _ask_base_folder(&self) -> () {
        loop {
            let mut folder = String::new();
            io::stdin()
                .read_line(&mut folder)
                .expect("Faild to read line");
            match folder.as_str() {
                "c" => return "components",
                "p" => return "pages",
                _ => println!("Не понял, попробуй еще раз"),
            }
        }
    }
}


fn main() {
    // let mut file = File::create("hello.txt")
    //     .expect("Error encountered while creating file!");
    // file.write_all(b"Hi, Welcome to Rust Programming!")
    //    .expect("Error while writing to file");
    asker = AskParams();
    asker.ask()
}
