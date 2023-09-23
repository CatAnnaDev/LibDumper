use std::{fs::File, io, io::Read};
use std::io::Write;
use goblin::mach::fat::FatArch;
use goblin::mach::MachO;

fn parse_data(arch: FatArch, data: &[u8]) {
    let object = MachO::parse(arch.slice(data), 0).expect("Failed to parse Mach-O object");
    for data in object.exports().unwrap(){
        println!("{:#?}", data);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path_to_dylib = "/Users/psyko/Library/Application Support/Steam/steamapps/common/Devour/Devour.app/Contents/Frameworks/GameAssembly.dylib";
    let mut dylib_buffer = Vec::new();
    File::open(&path_to_dylib)?.read_to_end(&mut dylib_buffer)?;
    let multiarch = goblin::mach::MultiArch::new(&dylib_buffer)?;

    /*
    let mut usr_input = String::new();
    print!("Arch ?:" );
    io::stdout().flush()?;
    io::stdin().read_line(&mut usr_input)?;
    */
    
    Ok(
        for arch in multiarch.iter_arches() {
            let arch = arch?;
            match arch.cputype {
                goblin::mach::cputype::CPU_TYPE_X86_64 => {
                    println!("Cpu Type {}: {:#?}", arch.cputype, arch);
                    parse_data(arch, dylib_buffer.as_slice());
                }
                goblin::mach::cputype::CPU_TYPE_ARM64 => {
                    println!("Cpu Type {}: {:#?}", arch.cputype, arch);
                    parse_data(arch, dylib_buffer.as_slice());
                }
                _ => {
                    eprintln!("No Arch supported in DyLib")
                }
            }
        }
    )
}