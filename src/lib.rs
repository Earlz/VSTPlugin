#[test]
fn it_works() {
}

#[macro_use]
extern crate vst2;

use vst2::plugin::{Info, Plugin};
use vst2::buffer::AudioBuffer;

#[derive(Default)]
struct BasicPlugin{
	limit_param: f32,
}

impl Plugin for BasicPlugin {
    fn get_info(&self) -> Info {
        Info {
            name: "Basic Plugin".to_string(),
            unique_id: 1358, // Used by hosts to differentiate between plugins.
            parameters: 1,

            ..Default::default()
        }
    }
    fn get_parameter_label(&self, index: i32) -> String { 
    	match index{
    		0 => String::from("max"),
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
    		0 => String::from("xxx"),
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
    	for (channel, ibuf) in inputs.iter().enumerate() {
	   		for (i, sample) in ibuf.iter().enumerate() {
	   			if *sample > self.limit_param as f64 {
	   				outputs[channel][i]= self.limit_param as f64;
	   			}else if *sample < (-self.limit_param as f64) {
	   				outputs[channel][i]= -self.limit_param as f64;
	   			}else{
	   				outputs[channel][i]=ibuf[i];
	   			}
	  		}
  		}
    }
}

plugin_main!(BasicPlugin); // Important!
