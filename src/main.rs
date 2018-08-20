extern crate lldb;

//use lldb::*;

use std::env;

mod analyse;
pub use analyse::AnalyseCold;
pub use analyse::AnalyseHot;
pub use analyse::Strategy;


fn main() {

    // 处理冰冷的尸体，初始化analyse_cold 对象时需要将二进制程序对象地址传入(暂时去重的策略为两种:
    // Strategy::ALLEqual & Strategy::RootEqual)
//    let mut analyse_cold = AnalyseCold::new(String::from("testcase/app"), Strategy::RootEqual);

    //批量处理某个文件夹下所有的coredump文件
//    analyse_cold.add_crashes(String::from("testcase/coredump"));

    //单条处理某个coredump文件
//    analyse_cold.add_crash(String::from("testcase/coredump/core_test"));

//    println!("{:?}", analyse_cold.num_unique_crashes()); 

    
    // 处理热乎的尸体，初始化analyse对象时，调用空构造器即可
//    SBDebugger::initialize();

//    let debugger = SBDebugger::create(false);
//    debugger.set_async(false);
//    println!("{:?}", debugger);
//    if let Some(target) = debugger.create_target_simple("testcase/app") {
//        let process = target.load_core("testcase/coredump/core_test");

        //创建analyse_hot 对象，调用空构造器即可(暂时去重的策略为两种: Strategy::ALLEqual &
        //Strategy::RooTEqual)
//        let mut analyse_hot = AnalyseHot::new(Strategy::ALLEqual);

        //单条分析crash，只需将 将死的SBProcess传入即可
//        analyse_hot.add_crash(&process);
//        println!("{:?}", analyse_hot.num_unique_crashes());
     //   println!("{:?}", process.exit_description());
//    }
    
//    SBDebugger::terminate();
//    println!("SBDebugger::terminate:   {:?}", debugger);

    let mut arguments = Vec::new();

    for argument in env::args() {
        arguments.push(argument);
    }

//    for a in arguments{
//        println!("*** argument = {}", a);
//    }
//

    let mut app = "";
    let mut directory = "";
    let mut corefile = "";
    let mut output = "output";
    let mut strategy = Strategy::ALLEqual;

    let mut i = 1;
    let mut valid = false;

    while i<arguments.len() {
        match arguments[i].as_ref() {
            "--help" => {
                println!("This is a help list of crash analyse!");
                println!("OPTIONS:");
                println!("\t -a <NAME> \t Name of the target application to analyse.");
                println!("\t -d <NAME> \t Name of the core files' directory.");
                println!("\t -c <NAME> \t Name of the coredump file.");
                println!("\t -o <NAME> \t Name of unique crashes' result file.");
                println!("\t -s <NAME> \t Name of deduplicate strategy: \n \t \t \t \t 1.all(default) --- all the traces' stack must be the same. \n \t \t \t \t 2.root --- only the root of traces' stack must be the same.");
                valid = false;
                break;
            },
            "-a" => {
                println!("target: {:?}", arguments[i+1]);
                app = &arguments[i+1];
                valid = true;
            },
            "-d" => {
                println!("directory: {:?}", arguments[i+1]);
                directory = &arguments[i+1];
                valid = true;
            },
            "-c" => {
                println!("core file: {:?}", arguments[i+1]);
                corefile = &arguments[i+1];
                valid = true;
            },
            "-o" => {
                println!("report file: {:?}", arguments[i+1]);
                output = &arguments[i+1];
            },
            "-s" => {
                println!("deduplicate strategy: {:?}", arguments[i+1]);
                if arguments[i+1] == "root"{
                    strategy = Strategy::RootEqual;
                }else{
                    strategy = Strategy::ALLEqual;
                }
            },
            _ => {
                println!("argument '{:?}' which wasn't expected, or isn't valid in this context", arguments[i]);
                println!("use '--help' for more imformations.");
                valid = false;
                break;
            },
        }
       // println!("{:?}",arguments[i]);
        i+=2;
    }

    if valid {
        let mut analyse_cold = AnalyseCold::new(app.to_string(), strategy);
        if analyse_cold.add_crashes(directory.to_string()) {
            if analyse_cold.num_unique_crashes() == 0 {
                println!("Error! No such application: {:?} or the directory {:?} is empty!", app,directory);
            }else{
                analyse_cold.report(output.to_string(), app.to_string());
            }
        }else if corefile != "" {
            analyse_cold.add_crash(corefile.to_string());
            if analyse_cold.num_unique_crashes() == 0 {
                println!("Error! No such corefile:{:?}", corefile);
            }else{
                analyse_cold.report(output.to_string(), app.to_string());
            }
        }else {
            println!("Error! No such direction: {:?}", directory);
        }
    }

}


