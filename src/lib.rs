pub type PcmSample = (f32);
use std::marker::Copy;

pub struct FeedBlock {
    pub out: String, 
    pub samples: Box<PcmSample>
}

#[macro_export]
macro_rules! feed_block {
    ($out: expr, $samples: ident) => {
        FeedBlock {
            out: $out.to_string(),
            samples: Box::new($samples)
        }
    }
}

pub enum UnitState {
    Idle,
    Init,
    Active,
    Failure
}

#[derive(Copy, Clone)]
pub enum RackSignal {
    Idle,
    Active,
    Failure,
    AC,
}


pub trait RackUnit {
    fn init(&mut self);
    fn cycle(&mut self);
    fn feed(&mut self) -> FeedBlock;

    fn get_unit_label(&self) -> &str;
    fn get_unit_type(&self) -> &str;
    fn get_unit_state(&self) -> RackSignal;

    fn unit_msg(&self, msg: &str) {
        println!("{} [{}]: {}", self.get_unit_label(), self.get_unit_type(), msg);
    }

    fn rack_signal(&mut self, signal_state: RackSignal) -> RackSignal {

        match signal_state {

            RackSignal::AC =>  {
                let s = self.get_unit_state();

                match self.get_unit_state() {

                    RackSignal::Idle =>  self.init(),
                    RackSignal::Active => self.cycle(),
                    _ => { }
                }
            }

            _ => { }

        }

        RackSignal::Active
    }
}

pub struct UnitHolder {
    units: Vec<Box<RackUnit>>
}

impl UnitHolder {
        pub fn new() -> UnitHolder {
            UnitHolder {
                units: Vec::new()
            }
        }

        pub fn add_unit(&mut self, unit: Box<RackUnit>) -> u32 {
            self.units.push(unit);

            self.units.len() as u32
        }

        pub fn get_unit(&mut self, index: usize) -> &mut RackUnit {
            &mut *self.units[0]
        }

        pub fn get_ref(&self, index: usize) -> &Box<RackUnit> {
            &self.units[0]
        }

        pub fn get_size(&self) -> i32 {
            self.units.len() as i32
        }
}

struct Rack {
    rack_state: RackSignal
}

impl Rack {
    fn new() -> Rack {
        Rack {
            rack_state: RackSignal::Idle
        }
    }

    fn get_state(&self) -> &RackSignal {
        &self.rack_state
    }

}

pub fn cycle_rack(holder: &mut UnitHolder) {
    let unit = holder.get_unit(0);
    unit.rack_signal(RackSignal::AC);
}

