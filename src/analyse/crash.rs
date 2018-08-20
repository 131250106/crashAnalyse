use lldb::*;

//use std::slice::Iter;

use super::Strategy;


pub fn create_crash( s: &Strategy, process: &SBProcess ) -> Box<Crash> {

    let mut threads = Vec::<Mythread>::new();

    let sbthreads = process.threads().collect::<Vec<SBThread>>();
                        
    for i in 0..sbthreads.len() {
        let index_id = sbthreads[i].index_id(); 
        let name = sbthreads[i].name().to_string();
        let stop_reason = sbthreads[i].stop_reason();
                                                                            
        let mut frames = Vec::<Myframe>::new();
                                                                                       
        let sbframes = sbthreads[i].frames().collect::<Vec<SBFrame>>();
                                                                                    
                                                                                    
        for j in 0..sbframes.len() {
            let file = sbframes[j].module().filespec();
                                                                                                                    
            let mut file_name = String::new();
            let mut function_name = String::new();
            let mut line_name = String::new();
            let mut line_num = 0;
            if file.exists() {
                file_name = file.filename().to_string();
                if let Some(function) = sbframes[j].function_name() {
                    function_name = function.to_string();
                }

                if let Some(line) = sbframes[j].pc_address().line_entry() {
                    line_name = line.filespec().filename().to_string();
                    line_num = line.line();
                }
            }
                                                                                                                                                                                                    
            let frame_ : Myframe = Myframe { file_name: file_name, function_name: function_name, line_name: line_name, line_num: line_num };
            frames.push(frame_);
        }
                                                                                                
        let thread_ : Mythread = Mythread { index_id: index_id, name: name, stop_reason: stop_reason, frames: frames };
        threads.push(thread_);
    }

    match s {
        Strategy::ALLEqual => return Box::new(CrashAll{ threads: threads}),
        Strategy::RootEqual => return Box::new(CrashRoot{ threads:threads}),
    };

}

pub fn print(crash: &Box<Crash>){
    for thread in crash.get_threads(){
//        println!("thread id: {}\tname: {}\tstop reason:{:?}", thread.index_id, thread.name, thread.stop_reason);
//        let mut test = format!("stop reason: {:?}", thread.stop_reason);
//        println!("{}",test);
        thread.print();
    }
}

pub fn report(crash: &Box<Crash>) -> String {
    let mut report = "".to_string();
    for thread in crash.get_threads(){
       report += &thread.report();
       report += "\n";
    }
    return report;
}

pub trait Crash {
    fn is_same(&self, crash_: &Box<Crash>) -> bool;
    fn get_threads(&self) -> &Vec<Mythread>;
}

pub struct CrashAll {
    threads: Vec<Mythread>,
}

//impl CrashAll {
//    fn get_threads(&self) -> &Vec<Mythread> {
//        &self.threads
//    }
//}

pub struct CrashRoot {
    threads: Vec<Mythread>,
}

//impl CrashRoot {
//    fn get_threads(&self) -> &Vec<Mythread> {
//        &self.threads
//    }
//}

impl Crash for CrashAll {

    fn get_threads(&self) -> &Vec<Mythread> {
        &self.threads
    }

    fn is_same(&self, crash_: &Box<Crash>) -> bool {

       let _threads = &self.threads;
       let threads_ = crash_.get_threads();

       if _threads.len() != threads_.len() {
           return false;
       }

       let threads_len = _threads.len();

       for i in 0..threads_len {

           if _threads[i].index_id != threads_[i].index_id {
               return false;
           }

           if _threads[i].name != threads_[i].name {
               return false;
           }

           if _threads[i].stop_reason != threads_[i].stop_reason {
               return false;
           }

           let i_frames = &_threads[i].frames;
           let frames_i = &threads_[i].frames;

           if i_frames.len() != frames_i.len() {
               return false;
           }

           let frames_len = i_frames.len();
           for j in 0..frames_len {
               
               if i_frames[j].file_name != frames_i[j].file_name {
                   return false;
               }

               if i_frames[j].function_name != frames_i[j].function_name {
                   return false;
               }

               if i_frames[j].line_name != frames_i[j].line_name {
                   return false;
               }

               if i_frames[j].line_num != frames_i[j].line_num {
                   return false;
               }
           }
       }

       return true;
   }
}

impl Crash for CrashRoot {

    fn get_threads(&self) -> &Vec<Mythread> {
        &self.threads
    }

    fn is_same(&self, crash_: &Box<Crash>) -> bool {
        
        let _threads = &self.threads;
        let threads_ = crash_.get_threads();

        if _threads.len() != threads_.len(){
            return false;
        }

        let len = _threads.len();

        for i in 0..len {
            if _threads[i].index_id != threads_[i].index_id {
                return false;
            }

            if _threads[i].name != threads_[i].name {
                return false;
            }
            
            if _threads[i].stop_reason != threads_[i].stop_reason {
                return false;
            }

            let i_frames = &_threads[i].frames;
            let frames_i = &threads_[i].frames;

            let _index = i_frames.len() - 1;
            let index_ = frames_i.len() - 1;

            if i_frames[_index].file_name != frames_i[index_].file_name {
                return false;
            }

            if i_frames[_index].function_name != frames_i[index_].function_name {
                return false;
            }

            if i_frames[_index].line_name != frames_i[index_].line_name {
                return false;
            }

            if i_frames[_index].line_num != frames_i[index_].line_num {
                return false;
            }

        }
        return true;
    }
}


struct Myframe {
    file_name: String,
    function_name: String,
    line_name: String,
    line_num: u32,
}

impl Myframe {

    pub fn print(&self) {
        println!("\tfile name:{}\tfunction name: {}\tline:{} {}", self.file_name, self.function_name, self.line_name, self.line_num);
    }

    pub fn report(&self) -> String {
        format!("\tfile name:{}\tfunction name: {}\tline:{} {}\n", self.file_name, self.function_name, self.line_name, self.line_num)
    }
}

struct Mythread {
    index_id: u32,
    name: String,
    stop_reason: StopReason,
    frames: Vec<Myframe>,
}

impl Mythread {

    pub fn print(&self) {
        println!("thread id: {}\tname: {}\tstop reason: {:?}", self.index_id, self.name, self.stop_reason);

        for frame in &self.frames{
            frame.print();
        }
    }

    pub fn report(&self) -> String {
        let mut report = format!("thread id: {}\tname: {}\tstop reason: {:?}\n", self.index_id, self.name, self.stop_reason);

        for frame in &self.frames{
            report += &frame.report();
        }
        return report;
    }
}
