use sysinfo::{System,Pid, SystemExt, ProcessExt, CpuExt};
use std::io::{stdin};
use std::{thread,time};
use regex::Regex;
use colored::*;
const ONE_GHZ_CONSUMPTION_WATS:f32 = 33.3;
const ONE_GB_RAM_CONSUMPTION_WATS_DDR4:f32 = 0.3725;


fn main() {
    menu();
}

fn menu(){
    let mut option_menu =1;
    while option_menu !=0 {
        println!("--------Menu😀--------");
        println!("[1] - List App Process");
        println!("[2] - Detail one Process by his PID");
        // println!("[3] - Detail one Process by name");
        println!("[0] - Exit");
        println!("{}","Please Select an Option []: ".italic());

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
            0=>println!("Good Bye!!"),
            1=>list_all_process(),
            2=>detail_one_process(),
            _=>{
                println!("{}",option_menu);
                println!("INPUT INCORRECT PLEASE TRY AGAIN");
            }
        }
        press_enter_to_continue();
        println!("");
    }
}

fn list_all_process(){

    let mut sys = System::new_all();
    sys.refresh_all();
    let mut number_process =0;
    let mut process_vec:Vec<String> = vec![];
    for (pid, process) in sys.processes() {
        process_vec.push(format!("{} [PID 🆔: {}]", process.name(), pid));
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

    println!("{}","Write the PID of one App:".italic());
    let mut buffer_read = String::new();
    stdin().read_line(&mut buffer_read ).unwrap();

    let pid = Pid::from(
        match buffer_read.trim().parse::<usize>(){
            Ok(res)=>res,
            Err(_)=>{
                println!("{}","The Value given its incorrect, you can see the PID selecting option one, at the end of one process on []".bright_red());
                0
            }
        }
    );
    
    match sys.process(pid){
        Some(process_result)=>{
            
            let cpus = sys.cpus().len() as f32;
            println!("Process Name: {}",process_result.name().magenta());
            println!("Process parent pid: {} ",process_result.parent().unwrap_or(Pid::from(0)).to_string().green());
            println!("Process Status: {}",process_result.status().to_string().green());
            println!("Process memory consumption: {} {}",process_result.memory().to_string().green(),"bytes".green());
            
            let mut op =0;
            let mut metrics_cpu:Vec<f32> = vec![];
            println!("{}","Taking metrics of CPU usage, Please Wait a few seconds...".italic());
            while op !=40 {
                sleetpp(200);
                sys.refresh_all();
                metrics_cpu.push(sys.process(pid).unwrap().cpu_usage()/cpus);
                // println!("Process cpu usage: {} %",sys.process(pid).unwrap().cpu_usage()/cpus);
                op+=1;
            }
            let components = components_computer();
            let avarage_cpu_usage =average(metrics_cpu)/8.0;
            println!("Process cpu usage : {} {}",&avarage_cpu_usage.to_string().green(),"%".green());

            // println!("[] {} % /sec",average(metrics_cpu)/8.0);
            calculate_consumption(components,avarage_cpu_usage ,sys.process(pid).unwrap().memory() );
            println!("");
        },
        None=>{
            println!("Process don't exist")
        }
    };

}

fn average(vect:Vec<f32>)->f32{
    let mut sum =0.0;
    for met in &vect{
        sum+= met;
    }
    sum / (vect.len() as f32)
}

fn sleetpp(time:u64){
    let time_millis = time::Duration::from_millis(time);
    // let now = time::Instant::now();
    thread::sleep(time_millis);
    // println!("{:?}",now.elapsed());
}

fn press_enter_to_continue(){
    println!("{}","Press Enter to continue...".italic());
    stdin().read_line(&mut String::new()).unwrap();
}

fn components_computer()->ComponentsCalc{
    let re = Regex::new(r"[0-9]+(\.[0-9]+)?GHz").unwrap();
    let mut sys = System::new_all();
    sys.refresh_all();
    sleetpp(100);
    // println!("{:#?}",sys.global_cpu_info());
    // println!("{:#?}",sys.cpus());
    // println!("Ram memory {:?} bytes",sys.total_memory());
    let frequency_string = match re.captures(sys.global_cpu_info().brand()){
        Some(val)=>val.get(0),
        None=>None
    };
    let mut components_res=ComponentsCalc{
        cpu_frequency_ghz:0.0,
        ram_memory_bytes:0,
    };
    match frequency_string{
        Some(val)=>{
            let freq_str:String = String::from(&val.as_str()[0..(val.as_str().len()-4)]);
            components_res.cpu_frequency_ghz =freq_str.parse::<f64>().unwrap();
        },
        None=>{
            // --------------------------------------------frecuency given in mhz
            components_res.cpu_frequency_ghz = (sys.global_cpu_info().frequency() as f64)/1000.0;
        }
    };
    components_res.ram_memory_bytes = sys.total_memory();
    components_res
}

fn calculate_consumption(components:ComponentsCalc,f_app_cpu:f32,f_app_memory:u64){
    let full_cpu_consumption_wats = components.cpu_frequency_ghz * (ONE_GHZ_CONSUMPTION_WATS as f64);
    let app_consumption_cpu_wats = (full_cpu_consumption_wats * (f_app_cpu as f64))/100.0;

    let full_memory_consumption_wats = (components.ram_memory_bytes/1000000000)as f32 * ONE_GB_RAM_CONSUMPTION_WATS_DDR4;
    let app_consumption_memory_wats =  (full_memory_consumption_wats * ((f_app_memory as f32)/1000000000.0))/100.0;

    
    println!("App Consumption {} {}", (app_consumption_cpu_wats+app_consumption_memory_wats as f64).to_string().blue(),"WATS".blue());
}

struct ComponentsCalc{
    cpu_frequency_ghz:f64,
    ram_memory_bytes:u64,
}