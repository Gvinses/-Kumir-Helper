use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::Path;
use console::Term;

struct Translation {
    english: &'static str,
    russian: &'static str,
}

fn main() {

    let term = Term::stdout();
    term.clear_screen().expect("Failed to clear the console");


    println!("Enter the path to the input file:");
    let mut input_path = String::new();
    std::io::stdin().read_line(&mut input_path).expect("Failed to read input path");
    let input_path = input_path.trim();

    println!("Enter the path to the output file:");
    let mut output_path = String::new();
    std::io::stdin().read_line(&mut output_path).expect("Failed to read output path");
    let output_path = output_path.trim();

    let file = File::open(input_path).expect("Failed to open input file");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).expect("Failed to read file contents");

    let dataset: Vec<Translation> = vec![
        Translation { english: "using (draftsman)", russian: "использовать Чертежник"},
        Translation { english: "pen_down", russian: "опустить перо"},
        Translation { english: "pen_up", russian: "поднять перо"},
        Translation { english: "main", russian: "алг" },
        Translation { english: "start", russian: "нач" },
        Translation { english: "end", russian: "кон" },
        Translation { english: "moveTo()", russian: "сместиться в точку" },
        Translation { english: "vectorTo()", russian: "сместиться на вектор()" },
    ];

    let mut translated_contents = String::with_capacity(contents.len());

    for line in contents.lines() {
        let mut translated_line = String::new();
        for translation in &dataset {
            if line.contains(translation.english) {
                if translation.english == "moveTo()" {
                    let mut x_y_values = String::new();
                    println!("moveTo() >>> Enter x and y values (separated by ','): ");
                    std::io::stdin().read_line(&mut x_y_values).expect("Failed to read line");
                    let x_y: Vec<&str> = x_y_values.split(",").collect();
                    let x = x_y[0].trim().parse::<f64>().unwrap();
                    let y = x_y[1].trim().parse::<f64>().unwrap();
                    translated_line.push_str(&format!("{}({}, {})", translation.russian, x, y));
                } else if translation.english == "vectorTo()" {
                    let mut x_y_values = String::new();
                    println!("vectorTo() >>> Enter x and y values (separated by ','): ");
                    std::io::stdin().read_line(&mut x_y_values).expect("Failed to read line");
                    let x_y: Vec<&str> = x_y_values.split(",").collect();
                    let x = x_y[0].trim().parse::<f64>().unwrap();
                    let y = x_y[1].trim().parse::<f64>().unwrap();
                    translated_line.push_str(&format!("{}({}, {})", translation.russian, x, y));
                } else {
                    let translated = line.replace(translation.english, translation.russian);
                    translated_line.push_str(&translated); 
                }
            }
        }
        translated_contents.push_str(&translated_line);
        translated_contents.push('\n');
    }

    let output_path = output_path;
    let mut output_file = File::create(output_path).expect("Failed to create output file");
    output_file.write_all(&[0xEF, 0xBB, 0xBF])
       .expect("Failed to write UTF-8 BOM to output file");

    output_file.write_all(translated_contents.as_bytes())
       .expect("Failed to write to output file");  

    println!("Translation completed and saved to the output file.");
}