use std::collections::HashMap;
use std::fs;

const LANG: &str = "Pazu";
const LANG_EXTENTION: &str = "pz";

enum Type {
    i(isize),
    u(u8),
    c(char),
    b(bool),
}

struct Interpretor {
    memory: HashMap<String, Type>,
}

impl Interpretor {
    fn interpretor(&mut self, code: &String) {
        let instructions = code.split(";");
        for (index, instr) in instructions.enumerate() {
            self.interpret(index, instr);
        }
    }

    fn interpret(&mut self, index: usize, instr: &str) -> Result<String, String> {
        let words: Vec<&str> = instr.split(" ").collect::<Vec<&str>>();
        if check_syntax(&words) {
            match words[0] {
                "v" => self.declare(words[1], enumify(words[3])),
                other => {
                    return Err(format!(
                        "Syntaxe \"{}\" non reconnue {}",
                        other,
                        stackTrace(index)
                    ));
                }
            }
        }
        Ok(format!("Ok"))
    }

    fn declare(&mut self, name: &str, value: Type) {
        self.memory.insert(name.to_string(), value);
    }
}

fn main() {
    let mut interpretor = Interpretor {
        memory: HashMap::new(),
    };
    println!("{}Lang", LANG);
    interpretor(&read_file("test.pz").unwrap());
}

fn check_syntax(instr: &Vec<&str>) -> bool {
    true
}

fn stackTrace(index: usize) -> String {
    format!("ligne {}", index)
}

fn read_file(file_name: &str) -> Result<String, String> {
    if file_name.ends_with(format!(".{}", LANG_EXTENTION).as_str()) {
        Ok(fs::read_to_string(file_name).expect("Erreur lecture fichier"))
    } else {
        Err(format!(
            "Seul les fichiers .{} peuvent être interprétés.",
            LANG_EXTENTION
        ))
    }
}
