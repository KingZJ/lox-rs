use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: generate-ast <output directory>");
        std::process::exit(64);
    }

    let output_dir = args.get(1).unwrap();
    let base_name = "Expr";
    let notation = vec![
        format!("Binary : Box<{base_name}> left, Token operator, Box<{base_name}> right"),
        format!("Grouping : Box<{base_name}> expression"),
        "Literal : Object value".to_owned(),
        format!("Unary : Token operator, Box<{base_name}> right"),
    ];
    let notation = parse_notation(base_name, notation);
    // println!("{:#?}", notation);
    define_ast(output_dir, base_name, &notation).unwrap();
}

fn parse_notation(base_name: &str, notation: Vec<String>) -> Vec<Notation> {
    let mut result: Vec<Notation> = vec![];
    for item in notation {
        let mut iter = item.split(":");
        let derive_name = iter.next().unwrap().trim().to_owned();
        let mut fields: Vec<(String, String)> = vec![];
        for field_str in iter.next().unwrap().split(",") {
            let mut field_iter = field_str.trim().split(" ");
            // (Object, value)
            fields.push((
                field_iter.next().unwrap().to_owned(),
                field_iter.next().unwrap().to_owned(),
            ));
        }

        result.push(Notation {
            base_name: base_name.to_string(),
            derive_name,
            fields,
        });
    }

    result
}

fn define_ast(output_dir: &str, base_name: &str, notation: &Vec<Notation>) -> io::Result<()> {
    let path = format!("{output_dir}/{}.rs", base_name.to_lowercase());
    let f = fs::File::create(path)?;
    let mut buffer = io::BufWriter::new(f);

    writeln!(buffer, "use crate::token::{{Token, Object}};")?;
    writeln!(buffer, "use crate::error::LoxError;")?;

    define_enum(&mut buffer, base_name, notation)?;
    define_impl_enum(&mut buffer, base_name, notation)?;

    define_struct(&mut buffer, notation)?;
    define_impl_struct(&mut buffer, base_name, notation)?;

    define_visitor(&mut buffer, base_name, notation)?;
    Ok(())
}

fn define_enum(
    buffer: &mut io::BufWriter<File>,
    base_name: &str,
    notation: &Vec<Notation>,
) -> io::Result<()> {
    writeln!(buffer, "\npub enum {} {{", base_name)?;
    for item in notation {
        writeln!(buffer, "\t{0}({0}{1}),", item.derive_name, item.base_name)?;
    }
    writeln!(buffer, "}}")?;
    Ok(())
}

fn define_impl_enum(
    buffer: &mut io::BufWriter<File>,
    base_name: &str,
    notation: &Vec<Notation>,
) -> io::Result<()> {
    writeln!(buffer, "\nimpl {} {{", base_name)?;
    writeln!(
        buffer,
        "\tpub fn accept<T>(&self, visitor: Box<&dyn {}Visitor<T>>) -> Result<T, LoxError> {{",
        base_name
    )?;
    writeln!(buffer, "\t\tmatch self {{")?;
    for item in notation {
        writeln!(
            buffer,
            "\t\t\t{}::{}(b) => b.accept(visitor),",
            item.base_name, item.derive_name
        )?;
    }
    writeln!(buffer, "\t\t}}")?;
    writeln!(buffer, "\t}}")?;
    writeln!(buffer, "}}")?;

    Ok(())
}

fn define_struct(buffer: &mut io::BufWriter<File>, notation: &Vec<Notation>) -> io::Result<()> {
    for item in notation {
        writeln!(
            buffer,
            "\npub struct {}{} {{",
            item.derive_name, item.base_name
        )?;
        for field in item.fields.iter() {
            writeln!(buffer, "\t{}: {},", field.1, field.0)?;
        }
        writeln!(buffer, "}}")?;
    }

    Ok(())
}

fn define_impl_struct(
    buffer: &mut io::BufWriter<File>,
    base_name: &str,
    notation: &Vec<Notation>,
) -> io::Result<()> {
    for item in notation {
        writeln!(buffer, "\nimpl {}{} {{", item.derive_name, item.base_name)?;
        writeln!(
            buffer,
            "\tpub fn accept<T>(&self, visitor: Box<&dyn {}Visitor<T>>) -> Result<T, LoxError> {{",
            base_name
        )?;
        writeln!(
            buffer,
            "\t\tvisitor.visit_{}_{}(self)",
            item.derive_name.to_lowercase(),
            item.base_name.to_lowercase()
        )?;
        writeln!(buffer, "\t}}")?;

        writeln!(buffer, "}}")?;
    }

    Ok(())
}

fn define_visitor(
    buffer: &mut io::BufWriter<File>,
    base_name: &str,
    notation: &Vec<Notation>,
) -> io::Result<()> {
    writeln!(buffer, "\npub trait {}Visitor<T> {{", base_name)?;
    for item in notation {
        writeln!(
            buffer,
            "\tpub fn visit_{}_{}(&self, {1}: &{}{}) -> Result<T, LoxError>;",
            item.derive_name.to_lowercase(),
            item.base_name.to_lowercase(),
            item.derive_name,
            item.base_name
        )?;
    }
    writeln!(buffer, "}}")?;

    Ok(())
}

#[derive(Debug)]
struct Notation {
    base_name: String,             // Expr
    derive_name: String,           // Binary
    fields: Vec<(String, String)>, // Expr left
}
