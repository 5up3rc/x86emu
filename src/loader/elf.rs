use std::fs::File;
use std::io::Read;

use zero::read_str;

use xmas_elf::{ElfFile, program, sections};
use xmas_elf::symbol_table::Entry;

use machine_state::MachineState;
use decoder::Decoder;
use cpu::emu_instructions::EmulationCPU;

pub fn elf(filename: &str, symbol: &str, print_instructions: bool, print_registers: bool, benchmark: bool) {
    let mut file = File::open(filename).expect("Cannot open file");
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer).expect("Failed to read file.");

    let elf_file = ElfFile::new(&buffer);

    // get address where to load the text section
    let load_address = get_load_address(&elf_file).expect("can not get load address");

    // get the binary code from the .text section
    let text_section = elf_file.find_section_by_name(".text").expect("text section not found.");
    let code = text_section.raw_data(&elf_file);
    let code_offset = text_section.offset();

    // get the virtual address of the main function
    let main_symbol_address = get_main_symbol_address(&elf_file, &symbol);

    let mut machine_state = MachineState::new();
    machine_state.mem_write(load_address + code_offset, code);
    machine_state.rip = main_symbol_address as i64;
    machine_state.rsp = 0x1000;

    machine_state.print_instructions = print_instructions;
    machine_state.print_registers = print_registers;

    let mut cpu = EmulationCPU {};
    let mut decoder = Decoder::new(&mut cpu, &mut machine_state);
    decoder.execute(benchmark);
}

fn get_load_address(elf_file: &ElfFile) -> Option<u64> {
    for sect in elf_file.program_iter() {
        let t = sect.get_type().unwrap();
        match t {
            program::Type::Load if sect.flags() & program::FLAG_X == program::FLAG_X => {
                return Some(sect.virtual_addr());
            }
            _ => {}
        }
    }
    return None;
}

fn get_main_symbol_address(elf_file: &ElfFile, symbol_name: &str) -> u64 {
    let symbol_string_table = elf_file.find_section_by_name(".strtab")
        .expect("strtab (String table) section not found, is this a stripped binary?");
    let symbol_string_table = symbol_string_table.raw_data(&elf_file);

    let symbol_table = elf_file.find_section_by_name(".symtab")
        .expect("symtab (Symbol table) section not found");
    if let sections::SectionData::SymbolTable64(data) = symbol_table.get_data(&elf_file).unwrap() {
        let symbol =
            data.iter()
                .find(|&symbol| {
                    read_str(&symbol_string_table[symbol.name() as usize..]) == symbol_name
                })
                .expect("symbol not found");
        symbol.value()
    } else {
        unreachable!();
    }
}
