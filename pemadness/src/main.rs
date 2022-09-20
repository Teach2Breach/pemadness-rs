use pemadness::{
    dbj2_hash, get_export_by_hash, get_loaded_module_by_hash, get_process_id_by_name, is_wow64,
};

mod lib;

fn main() {
    println!("PE Madness!");
    println!("Process ID {}", get_process_id_by_name("explorer.exe"));

    let module_hash = dbj2_hash("ntdll.dll".as_bytes());
    let module_base = unsafe {
        get_loaded_module_by_hash(module_hash).expect("Failed to get loaded module by name")
    };

    println!("Module Address {:p}", module_base);

    let export_hash = dbj2_hash("NtOpenProcess".as_bytes());
    let export_address = unsafe {
        get_export_by_hash(module_base, export_hash).expect("Failed to get export by hash")
    };

    println!("Export Address: {:p}", export_address);

    println!("Architecture x86_64: {}", is_wow64());
}
