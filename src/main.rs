use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::process::Command;

fn parse_c_file(file_path: &str) -> io::Result<bool> {
    let mut file = File::open(file_path)?;
    let mut code = String::new();
    file.read_to_string(&mut code)?;

    let expected_code = "int main() { return 2; }";
    Ok(code.trim() == expected_code)
}

fn generate_assembly(output_path: &str) -> io::Result<()> {
    let assembly_code = "
section .text
global _start

_start:
    mov rax, 60   ; syscall: exit
    mov rdi, 2    ; exit code: 2
    syscall
";
    let mut file = File::create(output_path)?;
    file.write_all(assembly_code.as_bytes())
}

fn assemble_and_link(asm_file: &str, output_executable: &str) -> io::Result<()> {
    println!("Assembling {}...", asm_file);
    let nasm_status = Command::new("nasm")
        .args(&["-f", "win64", asm_file, "-o", "temp.o"])
        .status()?;

    if !nasm_status.success() {
        eprintln!("NASM failed to run");
        return Err(io::Error::new(io::ErrorKind::Other, "NASM failed to run"));
    }

    println!("Linking temp.o to {}...", output_executable);
    let link_status = Command::new("link")
        .args(&["/subsystem:console", "/entry:_start", "temp.o", "/out:a.exe"])
        .status()?;

    if !link_status.success() {
        eprintln!("link.exe failed to run");
        return Err(io::Error::new(io::ErrorKind::Other, "link.exe failed to run"));
    }

    std::fs::rename("a.exe", output_executable)?;
    std::fs::remove_file("temp.o")?;
    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: ./compiler <c_file>");
        return Ok(());
    }

    let c_file = &args[1];
    let asm_file = "output.asm";
    let output_executable = "a.exe";

    if parse_c_file(c_file)? {
        println!("Generating assembly to: {}", asm_file);
        generate_assembly(asm_file)?;
        println!("Assembling and linking...");
        if let Err(e) = assemble_and_link(asm_file, output_executable) {
            eprintln!("Error during assembly and linking: {}", e);
        } else {
            println!("Compiled {} to {}", c_file, output_executable);
        }
    } else {
        eprintln!("Error: {} does not match the expected code.", c_file);
    }

    Ok(())
}
