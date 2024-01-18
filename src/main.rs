use std::{env, fs::File, io::Read};
use std::error::Error;
use goblin::{mach::Mach, Object};

fn main() -> Result<(), Box<dyn Error>> {

    let input = env::args().nth(1).ok_or("usage: ./test_tmp [executable file]")?;
    let mut file = File::open(input)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let result = Object::parse(&buffer)?;

    match result {
        Object::Mach(mac_os) => {
            match mac_os {
                Mach::Fat(e) => {
                    for x in e.iter_arches() {
                        let x = x?;
                        let arch_data = &buffer[x.offset as usize..(x.offset + x.size) as usize];
                        let result = Object::parse(arch_data)?;
                        match result {
                            Object::Mach(e) => {
                                match e {
                                    Mach::Binary(e) => {
                                        println!("Dependency found:");
                                        println!("Multi Arch {}:", get_cpu_type_mac_os(&x.cputype));
                                        for x in e.libs {
                                            println!("{}", x);
                                        }
                                        println!()
                                    },
                                    _ => {}
                                }
                            }
                            _ => {}
                        }

                    }
                }
                Mach::Binary(e) => {
                    println!("Dependency found: ({})", get_cpu_type_mac_os(&e.header.cputype));
                    for x in e.libs {
                        println!("{}", x);
                    }
                }
            }

        }
        Object::Elf(elf) => {
            println!("Dependency found: ");
            for x in elf.libraries {
                println!("{x}");
            }
        }
        Object::PE(pe)=> {
            println!("Dependency found: ");
            for x in pe.libraries {
                println!("{x}");
            }
        }
        Object::Archive(ar) => {
            println!("Archive not supported {:#?}", ar);
            for x in ar.members() {
                println!("Member: {}", x);
            }
        }
        Object::COFF(coff) =>{
            println!("COFF (Common Object File Format) not supported {:#?}", coff);
        }
        Object::Unknown(u) => {
            println!("Unknown not supported magic number is {}", u);
        }
        _ => {
            println!("Error unknown architecture.");
        }
    }
    Ok(())
}

fn get_cpu_type_mac_os(cpu: &u32) -> String {
    match cpu {
        16777223 => String::from("x86_64"),
        16777228 => String::from("arm64"),
        _ => String::from("Unknown"),
    }
}
