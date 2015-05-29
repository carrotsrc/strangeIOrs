extern crate strangeio;
use strangeio::*;
use std::ops::DerefMut;


struct MegaUnit {
    label: String,
    unit_type: String,
    state: UnitState
}

impl MegaUnit {
    fn new(label: String) -> MegaUnit {
        MegaUnit {
            label: label,
            unit_type: "MegaUnit".to_string(),
            state: UnitState::Idle,
        }
    }
}


impl RackUnit for MegaUnit {
    fn init(&mut self) {
        self.state = UnitState::Ready;
        self.unit_msg("Initialised");
    }

    fn cycle(&mut self) {
        match self.state {
            UnitState::Idle => self.init(),

            _ => self.unit_msg("Cycling")
        }
    }

    fn get_unit_label(&self) -> &str {
        &self.label
    }

    fn get_unit_type(&self) -> &str {
        &self.unit_type
    }

}

fn create_MegaUnit(label: String) -> MegaUnit {
    MegaUnit::new(label)
}



struct UnitConnection {
    sindex: u32,
    plug: String,

    dindex: u32,
    sock: String
}



fn main() {
    let mut u = create_MegaUnit("foobar".to_string());
    let mut v = create_MegaUnit("foobar2".to_string());
    let mut uh = UnitHolder::new();

    let s = uh.add_unit(Box::new(u));
    let d = uh.add_unit(Box::new(v));

    let conA = UnitConnection {
        sindex: s,
        plug: "audio_out".to_string(),
        dindex: d,
        sock: "audio_in".to_string()
    };

    cycle_rack(&mut uh);
    cycle_rack(&mut uh);
}
