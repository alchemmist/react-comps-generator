use colored::Colorize;
use std::env;
use std::io::{stdin,stdout,Write};
use std::process;
use crate::element::Element;


pub struct AskParams {
    _element: Option<Element>,
    spliter: String
}

impl AskParams {
    pub fn new(spliter: String) -> AskParams {
        AskParams { 
            _element: None,
            spliter,
        }
    }

    pub fn ask(&self) -> Element {
        let folder = self._ask_folder();
        let name = self._ask_name(&folder);
        let absolute_path = self._generate_path(&folder);

        Element { absolute_path , name, spliter: self.spliter.clone() }
    }

    fn _ask_name(&self, base_folder: &String) -> String {

        let mut name = String::new();
        print!("What do we call the component? {}/", *base_folder);
        let _= stdout().flush();
        stdin().read_line(&mut name).expect("Failed to read line");
        if let Some('\n')=name.chars().next_back() {
            name.pop();
        }
        if let Some('\r')=name.chars().next_back() {
            name.pop();
        }

        name
    }

    fn _ask_folder(&self) -> String {
        println!("Which folder should we put the component in? ");
        loop {
            let mut folder = String::new();
            print!("c — components, p — pages, m - modules: ");
            let _= stdout().flush();
            stdin().read_line(&mut folder).expect("Failed to read line");
            if let Some('\n')=folder.chars().next_back() {
                folder.pop();
            }
            if let Some('\r')=folder.chars().next_back() {
                folder.pop();
            }

            match folder.as_str() {
                "c" => return "components".to_string(),
                "p" => return "pages".to_string(),
                "m" => return "modules".to_string(),
                _ => println!("Error, try again!"),
            }
        }
    }

    fn _generate_path(&self, base_folder: &String,) -> String {
        env::current_dir()
            .expect("Failed to get current path")
            .to_string_lossy()
            .to_string() + &self.spliter + &base_folder  
    }

    pub fn ask_ok(&self, file_names: &[String]) {
        let _file_names_joined = file_names.join("\n\t");
        loop {
            println!("So I'm creating the files: \n");
            println!("\t{}", _file_names_joined.yellow());

            let mut answer = String::new();
            print!("\nOk? [y]/n: ");
            let _= stdout().flush();
            stdin().read_line(&mut answer).expect("Failed to read line");
            if let Some('\n')=answer.chars().next_back() {
                answer.pop();
            }
            if let Some('\r')=answer.chars().next_back() {
                answer.pop();
            }
            match answer.as_str() {
                "y" | "" => return (),
                "n" => {println!("Nothing created. Exit."); 
                        process::exit(0)},
                _ => {  println!("----------------------------------------------------");
                        println!("Error, try again!");
                    }
            }
        }
    }
}

