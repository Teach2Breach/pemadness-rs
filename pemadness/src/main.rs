use pemadness::get_process_id_by_name;

mod lib;

fn main() {
    println!("PE Madness!");
    println!("Process ID {}", get_process_id_by_name("notepad.exe"));
}
