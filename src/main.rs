use sysinfo::{System, SystemExt, ProcessExt,Pid};
use std::io::{stdin};
fn main() {
    menu();
}

fn menu(){
    let mut option_menu =1;
    while option_menu !=0 {
        println!("Welcome Menu😀");
        println!("[1] - List App Process");
        println!("[2] - Detail one Process by his PID");
        println!("[3] - Detail one Process by name");
        println!("[0] - Exit");
        println!("Please Select an Option : ");

        let mut buffer_read = String::new();
        stdin().read_line(&mut buffer_read ).unwrap();

        option_menu = match buffer_read.trim().parse::<i32>() {
            Ok(value)=>value,
            Err(e)=>{
                println!("{}",e);
                -2
            }
        };
        match option_menu {
            0=>println!("Good Bye"),
            1=>list_all_process(),
            2=>detail_one_process(),
            _=>{
                println!("{}",option_menu);
                println!("INPUT INCORRECT PLEASE TRY AGAIN");
            }
        }
        println!("\n\n");
    }
}

fn list_all_process(){

    let mut sys = System::new_all();
    sys.refresh_all();
    let mut number_process =0;
    let mut process_vec:Vec<String> = vec![];
    for (pid, process) in sys.processes() {
        process_vec.push(format!("{} PID{}", process.name(), pid));
    }
    process_vec.sort();
    for pr in process_vec {
        number_process +=1 ;
        println!("[{}] - {}",number_process,pr);
    }
}

fn detail_one_process(){
    let mut sys = System::new_all();
    sys.refresh_all();

    println!("Enter the PID of one App:");
    let mut buffer_read = String::new();
    stdin().read_line(&mut buffer_read ).unwrap();

    let pid = Pid::from(
        match buffer_read.trim().parse::<usize>(){
            Ok(res)=>res,
            Err(_)=>{
                println!("The Value given its incorrect");
                0
            }
        }
    );
    
    match sys.process(pid){
        Some(process_result)=>{
            println!("Process Name: {}",process_result.name());
            println!("Process memory consumption: {} bytes",process_result.memory());
            println!("Process cpu consumption: {} %",process_result.cpu_usage());
        },
        None=>{
            println!("Process don't exist")
        }
    };

}


// fn press_anykey_to_continue(){
//     println!("Press any Key to continue");
//     stdin().read_line(&mut String::new()).unwrap();

// }