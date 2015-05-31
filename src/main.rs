#[macro_use]
extern crate strangeio;
use strangeio::*;
use std::ops::DerefMut;


struct TestRig {
    label: String,
    unit_type: String,
    state: RackSignal,

    pub connections: Vec<UnitConnection>,

}

impl TestRig {
    fn new(label: String) -> TestRig {
        TestRig {
            label: label,
            unit_type: "TestRig".to_string(),
            state: RackSignal::Idle,
            connections: Vec::new() 
        }
    }

}


impl ProcessorUnit for TestRig {
    fn init(&mut self) {
        self.state = RackSignal::Active;
        self.unit_msg("Initialised");
    }

    fn cycle(&mut self, connections: &mut Vec<UnitConnection>, sock: Option<&mut UnitConnection>) {
        self.unit_msg("Cycling");
        for c in connections {
            println!("{}", c.plug);
        }
    }

    fn get_unit_label(&self) -> &str {
        &self.label
    }

    fn get_unit_type(&self) -> &str {
        &self.unit_type
    }

    fn get_unit_signal(&self) -> RackSignal {
        self.state
    }

    fn feed(&mut self) -> FeedBlock {
        let samples: PcmSample = (300.00);
        feed_block!("audio_out", samples)
    }

    fn build_scheme(&mut self) -> Vec<UnitConnection> {
        vec![
        gen_connection!("audio_out".to_string()),
        ]
    }
}

fn create_TestRig (label: String) -> TestRig {
    TestRig::new(label)
}

fn main() {
    let mut u = create_TestRig("foobar".to_string());
    let v = create_TestRig("foobar2".to_string());

    let mut uh = UnitHolder::new();

    let s = uh.add_unit(Box::new(u));
    let d = uh.add_unit(Box::new(v));


    cycle_rack(&mut uh);
    cycle_rack(&mut uh);
}
