#[test]
fn it_works() {
}

#[macro_use]
extern crate vst2;

use vst2::plugin::{Info, Plugin};
use vst2::buffer::AudioBuffer;

#[derive(Default)]
struct BasicPlugin;

impl Plugin for BasicPlugin {
    fn get_info(&self) -> Info {
        Info {
            name: "Basic Plugin".to_string(),
            unique_id: 1358, // Used by hosts to differentiate between plugins.

            ..Default::default()
        }
    }
    fn process_f64(&mut self, buffer: AudioBuffer<f64>){
    	let (inputs, mut outputs) = buffer.split();
    	for (channel, ibuf) in inputs.iter().enumerate() {
	   		for (i, sample) in ibuf.iter().enumerate() {
	   			if *sample > 0.4 {
	   				outputs[channel][i]=0.4;
	   			}else if *sample < (-0.4) {
	   				outputs[channel][i]=-0.4;
	   			}else{
	   				outputs[channel][i]=ibuf[i];
	   			}
	  		}
  		}
    }
}

plugin_main!(BasicPlugin); // Important!
