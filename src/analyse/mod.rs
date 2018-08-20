use lldb::*;
use std::fs;
use std::collections::HashMap;
use std::collections::hash_map::Iter;
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

mod crash;
pub use self::crash::Crash;

pub enum Strategy {
    ALLEqual,
    RootEqual,
}


pub struct AnalyseCold {
    app_str: String,
    debugger: SBDebugger,
    analyse: AnalyseCrash,
}

impl Drop for AnalyseCold{
    fn drop(&mut self) {
        SBDebugger::terminate();
        println!("SBDebugger::terminate:   {:?}", self.debugger);
    }
}


impl AnalyseCold {
    pub fn new(app_str: String, s: Strategy) -> AnalyseCold {

        SBDebugger::initialize();

        let debugger = SBDebugger::create(false);
        debugger.set_async(false);
        println!("{:?}", debugger);

        AnalyseCold { app_str: app_str, debugger: debugger, analyse: AnalyseCrash::new(s) }
    }


    pub fn add_crashes(&mut self, dir_str: String) -> bool {
       if let Ok(entries) = fs::read_dir(&dir_str) {
            for entry in entries { 
                if let Ok(entry) = entry {
                    let path = entry.path();
                    println!("{:?}", entry.path());
                                                                                                           
                    let core_path = path.to_str().unwrap();
                    self.add_crash(core_path.to_string());
                                                                                                                                                                   
                }
            }
            return true;
        }

        return false;
    }

    pub fn add_crash(&mut self, core_path: String) -> bool {
        if let Some(target) = self.debugger.create_target_simple(&self.app_str) {
            let process = target.load_core(&core_path);

            return self.analyse.add_crash(core_path, &process);
        }

        return false;
    }


    pub fn num_unique_crashes(&self) -> usize {
        self.analyse.num_unique_crashes()
    }

    pub fn get_unique_crashes(&self) -> Iter<String, Box<Crash>> {
        self.analyse.get_unique_crashes()
    }

    pub fn report(&self, path: String, app: String) {
        self.analyse.report(path, app);
    } 

}


pub struct AnalyseHot {
    analyse: AnalyseCrash,
}

impl AnalyseHot {

    pub fn new(s: Strategy) -> AnalyseHot {
        AnalyseHot { analyse: AnalyseCrash::new(s) }
    }

    pub fn add_crash(&mut self, process: &SBProcess) -> bool {
        self.analyse.add_crash(String::from("Hot"), process)
    }

    pub fn num_unique_crashes(&self) -> usize {
        self.analyse.num_unique_crashes()
    }

    pub fn get_unique_crashes(&self) -> Iter<String, Box<Crash>> {
        self.analyse.get_unique_crashes()
    }

    pub fn report(&self, path: String, app: String) {
        self.analyse.report(path, app);
    }

}

struct AnalyseCrash {
    strategy: Strategy,
    crashes: HashMap<String, Box<Crash>>, 
}

impl AnalyseCrash{

    fn new(strategy: Strategy) -> AnalyseCrash {
        AnalyseCrash{ strategy: strategy, crashes: HashMap::new() }
    }
    
    fn num_unique_crashes(&self) -> usize {
        self.crashes.len()
    }

    fn get_unique_crashes(&self) -> Iter<String, Box<Crash>>  {
        self.crashes.iter()
    }

    fn contains(&self, crash_: &Box<Crash>) -> bool {
        for (_, _crash) in &self.crashes {
            if _crash.is_same(crash_) {
                return true;
            }
        }
        return false;
    }

    fn add_crash(&mut self, core_path: String,  process: &SBProcess ) -> bool {
       let crash = crash::create_crash(&self.strategy, process);

       if self.contains(&crash) == false {
           self.insert(core_path, crash);
       }
       
       return true;
    }

    fn insert(&mut self, str: String, crash: Box<Crash>) {
        self.crashes.insert(str, crash);
    }

    fn report(&self, report_path: String, app: String) {
        let path = Path::new(&report_path);
        let display = path.display();
        
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why.description()),
            Ok(file) => file,
        };

        println!("total unique crashes number: {:?}", self.num_unique_crashes());

        let mut report = format!("crashes analyse for application = {}\ntotal unique crashes number: {:?}\n", app, self.num_unique_crashes());
        
        for (key,crash) in self.get_unique_crashes() {
            println!("coredump file: {:?}\n",key);
            crash::print(crash);
            report += &format!("coredump file: {:?}\n", key);
            report += &crash::report(crash);
        }

        match file.write_all(report.as_bytes()) {
            Err(why) => {
                panic!("couldn't write to {}: {}", display, why.description())
            },
            Ok(_) => println!("successfully wrote to {}", display),
        };

        println!("report!");
    }
}
