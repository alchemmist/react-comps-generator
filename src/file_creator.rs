use std::fs::{File, create_dir, OpenOptions};
use colored::Colorize;
use std::path::Path;
use std::io::Write;
use crate::element::Element;
use std::process;


pub struct FileCreator {
    _element: Element,
}

impl FileCreator {
    pub fn new(elem: Element) -> FileCreator {
        FileCreator  { 
            _element: elem,
        }
    }

    pub fn create(&self) {
        self._create_empty_file();
        self._write_file_contents();
    }

    fn _create_empty_file(&self) {
        if !Path::new(&self._element.absolute_path).is_dir() {
            create_dir(self._element.absolute_path.clone()).expect("Error while create dir");
        }

        let file = self._element.absolute_path.clone() + &self._element.spliter + &self._element.name;
        if !Path::new(&(file.clone() + ".jsx")).is_file() && !Path::new(&(file.clone() + ".scss")).is_file(){
            File::create(
                file + ".jsx",
            ).expect("Error encountered while creating file!");
            File::create(
                self._element.absolute_path.clone() + &self._element.spliter + &self._element.name + ".scss",
            ).expect("Error encountered while creating file!");
        } else {
            println!("{}\n{}\n{}", 
                "Есть риск утери информации!".red().bold(), 
                "Такие файлы уже существуют. Если вы убедились, \nчто в этих файлах нет важной информации,\nудалите их, и повторите операцию создания компонетна.",  
                "Завершаюсь!"); 
            process::exit(0)
        }

    }

    fn _write_file_contents(&self) {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(
                    self._element.absolute_path.clone() + &self._element.spliter + &self._element.name + ".jsx",
                )
            .unwrap();
        let code = format!("import React from 'react';\nimprt '{}.scss';\n\n\nfunction {}() \n\n\nexport default {};", 
            self._element.name, self._element.name, self._element.name);
        // writeln!(file, "ahahahhaha").expect("Error whlie writing to file");
        file.write_all(code.as_bytes()).expect("Error whlie writing to file");
    }

    pub fn get_created_files(&self) -> [String; 2] {
        let spliter = self._element.spliter.clone();
        let binding = self._element.absolute_path.clone();
        let path_as_vec = binding.split(&spliter).collect::<Vec<_>>();
        [
            ".".to_string() + &spliter.to_string() + &String::from(path_as_vec[path_as_vec.len() - 1]) + &spliter + &self._element.name + ".jsx",
            ".".to_string() + &spliter.to_string() + &String::from(path_as_vec[path_as_vec.len() - 1]) + &self._element.spliter + &self._element.name + ".scss",
        ]    
    }
}

