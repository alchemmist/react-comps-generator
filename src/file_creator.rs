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
        FileCreator { _element: elem }
    }

    pub fn create(&self) {
        let dir_path = self._create_empty_dir();
        self._create_empty_files(&dir_path);
        self._write_file_contents(&dir_path);
        println!("Done!");
    }

    fn _create_empty_dir(&self) -> String {
        let kebab_name: String = self._element.name.chars().fold(String::new(), |mut acc, c| {
            if c.is_uppercase() {
                if !acc.is_empty() {
                    acc.push('-');
                }
                acc.push(c.to_ascii_lowercase());
            } else {
                acc.push(c);
            }
            acc
        });

        let full_dir = format!("{}{}{}", self._element.absolute_path, self._element.spliter, kebab_name);
        if Path::new(&full_dir).exists() {
            println!("{} {}", "Директория уже существует:".yellow().bold(), full_dir);
        } else {
            create_dir(&full_dir).unwrap_or_else(|_| {
                process::exit(1);
            });
        }
        full_dir
    }

    fn _create_empty_files(&self, dir_path: &str) {
        let base = format!("{}{}", dir_path, self._element.spliter);
        let jsx_path = format!("{}{}.jsx", base, self._element.name);
        let scss_path = format!("{}{}.scss", base, self._element.name);

        if Path::new(&jsx_path).exists() || Path::new(&scss_path).exists() {
            eprintln!("{}\n{}", "Есть риск утери информации!".red().bold(), "Файлы с таким именем уже существуют. Удалите их и повторите операцию.");
            process::exit(1);
        }

        File::create(&jsx_path).unwrap_or_else(|e| {
            eprintln!("Ошибка при создании файла {}: {}", jsx_path, e);
            process::exit(1);
        });

        File::create(&scss_path).unwrap_or_else(|e| {
            eprintln!("Ошибка при создании файла {}: {}", scss_path, e);
            process::exit(1);
        });
    }

    fn _write_file_contents(&self, dir_path: &str) {
        let jsx_path = format!("{}{}{}", dir_path, self._element.spliter, self._element.name) + ".jsx";
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&jsx_path)
            .unwrap_or_else(|e| {
                eprintln!("Ошибка при открытии файла для записи {}: {}", jsx_path, e);
                process::exit(1);
            });

        let code = format!(
            "import React from 'react';\nimport './{}.scss';\n\nfunction {}() {{\n    return (\n        <div className=\"{}\">\n            {{/* TODO: implement component */}}\n        </div>\n    );\n}}\n\nexport default {};",
            self._element.name,
            self._element.name,
            self._element.name,
            self._element.name
        );

        file.write_all(code.as_bytes()).unwrap_or_else(|e| {
            eprintln!("Ошибка при записи в файл {}: {}", jsx_path, e);
            process::exit(1);
        });
    }

    pub fn get_created_files(&self) -> [String; 2] {
        let kebab_name: String = self._element.name.chars().fold(String::new(), |mut acc, c| {
            if c.is_uppercase() {
                if !acc.is_empty() {
                    acc.push('-');
                }
                acc.push(c.to_ascii_lowercase());
            } else {
                acc.push(c);
            }
            acc
        });
        let dir = format!("{}{}{}{}", 
            ".",
            self._element.spliter,
            self._element.absolute_path.split(&self._element.spliter).last().unwrap(),
            self._element.spliter,
        );
        let jsx = format!("{}{}{}{}.jsx", dir, kebab_name, self._element.spliter, self._element.name);
        let scss = format!("{}{}{}{}.scss", dir, kebab_name, self._element.spliter, self._element.name);
        [jsx, scss]
    }
}

