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
    fn interpret(&mut self, code: &String) {
        let instructions = code.split(";");
        for (index, instr) in instructions.enumerate() {
            self.interpretLine(index, instr);
        }
    }

    fn interpretLine(&mut self, index: usize, instr: &str) -> Result<String, String> {
        let words: Vec<&str> = instr.split(" ").collect::<Vec<&str>>();
        match check_syntax(&words) {
            Ok(_) => match words[0] {
                "v" => {
                    self.declare(
                        words[1],
                        makeValue(words[1].chars().next().unwrap(), words[3])?,
                    );
                    return Ok(format!("Ok"));
                }
                other => {
                    return Err(format!(
                        "Syntaxe \"{}\" non reconnue {}",
                        other,
                        stackTrace(index)
                    ));
                }
            },
            Err(e) => Err(e),
        }
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
    interpretor.interpret(&read_file("test.pz").unwrap());
}

fn check_syntax(instr: &Vec<&str>) -> Result<(), String> {
    Ok(())
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

fn makeValue(t: char, v: &str) -> Result<Type, String> {
    match t {
        'i' => Ok(Type::i(v.parse::<isize>().unwrap())),
        'u' => Ok(Type::u(v.parse::<u8>().unwrap())),
        'c' => Ok(Type::c(v.parse::<char>().unwrap())),
        'b' => match v {
            "t" => return Ok(Type::b(true)),
            "f" => return Ok(Type::b(false)),
            v => {
                return Err(format!(
                    "{} n'est pas un booléen, utiliser t pour true et f pour false.",
                    v
                ));
            }
        },
        t => Err(format!("{} n'est pas un type.", t)),
    }
}
