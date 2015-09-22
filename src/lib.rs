#[test]
fn it_works() {
}

#[macro_use]
extern crate vst2;

use vst2::plugin::{Info, Plugin};
use vst2::buffer::AudioBuffer;
use std::collections::vec_deque::VecDeque;



#[derive(Default)]
struct BasicPlugin{
	history: VecDeque<f64>,
	accumulator: f64,
	kickback: f64,

	//params..
	limit_param: f32, //0
	kickback_threshold_param: f64, //1
	cap_param: f64, //2 //divided by 10
	kickback_mul_param: f64, //multiplied by 10 // 3
	history_depth_param: f64, //multiplied by 100 // 4
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
		if self.kickback > self.kickback_threshold_param || self.kickback < -self.kickback_threshold_param {
			if self.kickback > 0.0 {
				self.kickback += -0.05;
			} else {
				self.kickback += 0.05;
			}
			return self.kickback;
		}
		for s in self.history.iter() {
			cap+=*s;
			if cap > self.cap_param {
				neg=1;
				if last < self.cap_param * 0.8 {
					outs.push(*s);
				}
			}else if cap < -self.cap_param {
				neg=-1;
				if last > -(self.cap_param*0.8) {
					outs.push(*s);
				}
			}
			if neg == 1 {
				let t=self.average_out(outs);
				self.kickback = t * 0.4;
				return t;
			} else if neg == -1 {
				let t=self.average_out(outs);
				self.kickback = t * 0.4;
				return t;
			}
			last=cap;
		}
		return 0.0;
	}
}

impl Plugin for BasicPlugin {
	fn init(&mut self) {
		self.history.reserve_exact(6000);
		self.accumulator = 0.0;

	}
    fn get_info(&self) -> Info {
        Info {
            name: "Basic Plugin".to_string(),
            unique_id: 1358, // Used by hosts to differentiate between plugins.
            parameters: 5,
            silent_when_stopped: true,
            inputs: 1,
            outputs: 1,

            ..Default::default()
        }
    }
    fn get_parameter_label(&self, index: i32) -> String { 
    	match index{
    		0 => self.limit_param.to_string(),
    		1 => self.kickback_threshold_param.to_string(),
    		2 => self.cap_param.to_string(),
    		_ => String::from("unknown")
    	}
    }
    fn get_parameter_name(&self, index: i32) -> String { 
    	match index{
    		0 => String::from("limit: "),
    		1 => String::from("kickback thres: "),
    		2 => String::from("cap: "),
    		_ => String::from("unknown")
    	}
    }
    fn get_parameter_text(&self, index: i32) -> String { 
    	match index{
    		0 => String::from("value"),
    		1 => String::from("value"),
    		_ => String::from("unknown")
    	}
    }

    fn get_parameter(&self, index: i32) -> f32 { 
    	match index{
    		0 => self.limit_param,
    		1 => self.kickback_threshold_param as f32,
    		2 => self.cap_param as f32 / 10.0,
    		_ => 0.0
    	}
    }

    fn set_parameter(&mut self, index: i32, val: f32) { 
    	match index{
    		0 => self.limit_param=val,
    		1 => self.kickback_threshold_param=val as f64,
    		2 => self.cap_param = val as f64 / 10.0 ,
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
