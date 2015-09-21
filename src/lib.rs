#[test]
fn it_works() {
}

#[macro_use]
extern crate vst2;

use vst2::plugin::{Info, Plugin};
use vst2::buffer::AudioBuffer;
use std::collections::vec_deque::VecDeque;


const HISTORY_SIZE: usize = 7000;

#[derive(Default)]
struct BasicPlugin{
	limit_param: f32,
	history: VecDeque<f64>,
	accumulator: f64
}

impl BasicPlugin{
	
	fn average_out(&self, avgs: Vec<f64>) -> f64{
		let mut avg = 0.0;
		for (i,item) in avgs.iter().enumerate() {
			avg+= *item;
		}
		return avg/avgs.len() as f64;
	}
	//assumes history is up to date
	fn calc_output(&mut self) -> f64 {
		let mut cap=0.0;
		let mut last=0.0;
		let mut outs = Vec::new();
		let mut neg=0;
		for s in self.history.iter() {
			cap+=*s;
			if cap > 0.1 {
				neg=1;
				if last < 0.08 {
					outs.push(*s);
				}
			}else if cap < -0.1 {
				neg=-1;
				if last > -0.08 {
					outs.push(*s);
				}
			}
			if neg == 1 {
				return self.average_out(outs);
			} else if neg == -1 {
				return self.average_out(outs);
			}
			last=cap;
		}
		return 0.0;
	}
}

impl Plugin for BasicPlugin {
	fn init(&mut self) {
		self.history.reserve_exact(HISTORY_SIZE);
		self.accumulator = 0.0;
	}
    fn get_info(&self) -> Info {
        Info {
            name: "Basic Plugin".to_string(),
            unique_id: 1358, // Used by hosts to differentiate between plugins.
            parameters: 1,
            silent_when_stopped: true,

            ..Default::default()
        }
    }
    fn get_parameter_label(&self, index: i32) -> String { 
    	match index{
    		0 => self.limit_param.to_string(),
    		_ => String::from("unknown")
    	}
    }
    fn get_parameter_name(&self, index: i32) -> String { 
    	match index{
    		0 => String::from("limit: "),
    		_ => String::from("unknown")
    	}
    }
    fn get_parameter_text(&self, index: i32) -> String { 
    	match index{
    		0 => String::from("value"),
    		_ => String::from("unknown")
    	}
    }

    fn get_parameter(&self, index: i32) -> f32 { 
    	match index{
    		0 => self.limit_param,
    		_ => 0.0
    	}
    }

    fn set_parameter(&mut self, index: i32, val: f32) { 
    	match index{
    		0 => self.limit_param=val,
    		_ => ()
    	}
    }

    fn process_f64(&mut self, buffer: AudioBuffer<f64>){
    	let (inputs, mut outputs) = buffer.split();
    	let mut historied = false;
    	for (channel, ibuf) in inputs.iter().enumerate() {
	   		for (i, sample) in ibuf.iter().enumerate() {
	   			if *sample > self.limit_param as f64 {
	   				outputs[channel][i]= self.limit_param as f64;
	   			}else if *sample < (-self.limit_param as f64) {
	   				outputs[channel][i]= -self.limit_param as f64;
	   			}else{
	   				outputs[channel][i]=ibuf[i];
	   			}
	   			if !historied {
	   				self.history.pop_back();
	   				self.history.push_front(*sample);
	   				outputs[channel][i]=self.calc_output();
	   				
	   			}
	  		}
	  		historied = true;
  		}
    }
}

plugin_main!(BasicPlugin); // Important!
