use std::cell::{RefCell,RefMut};
use std::ops::{DerefMut};
pub type PcmSample = (f32);
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

#[derive(Copy, Clone)]
pub enum FeedSignal {
    Ready,
    Ok { plug: i32 },
    Wait,
    Failure
}

#[derive(Copy, Clone)]
pub enum RackSignal {
    Idle,
    Active,
    Failure,
    AC,
}

pub struct UnitConnection {
    pub src_index: i32,
    pub plug: String,
    pub dst_index: i32,
    pub sock: String,

    pub signal: FeedSignal,
    pub block: Option<Box<PcmSample>>,
}

#[macro_export]
macro_rules! gen_connection {
    ($name: expr) => {
        UnitConnection {
            src_index: 0,
            plug: $name,
            dst_index: 0,
            sock: String::new(),

            signal: FeedSignal::Ready,
            block: None,
        }
    }
}


pub trait ProcessorUnit {
    fn init(&mut self);
    fn cycle(&mut self, connections: &mut Vec<UnitConnection>, sock: Option<&mut UnitConnection>) -> FeedSignal;
    fn feed(&mut self) -> FeedBlock;
    fn build_scheme(&mut self) -> Vec<UnitConnection>;

    fn get_unit_label(&self) -> &str;
    fn get_unit_type(&self) -> &str;
    fn get_unit_signal(&self) -> RackSignal;

    fn unit_msg(&self, msg: &str) {
        println!("{} [{}]: {}", self.get_unit_label(), self.get_unit_type(), msg);
    }


}

pub struct UnitContainer {
    unit: Box<ProcessorUnit>,
    connections: Vec<UnitConnection>,
}

impl UnitContainer {
    fn new(mut unit: Box<ProcessorUnit>) -> UnitContainer {
        UnitContainer {
            connections: unit.build_scheme(),
            unit: unit,
        }
    }

    fn rack_signal(&mut self, signal_state: RackSignal) -> RackSignal {

        match signal_state {

            RackSignal::AC =>  {

                match self.unit.get_unit_signal() {
                    RackSignal::Idle =>  self.unit.init(),
                    RackSignal::Active => { self.unit.cycle(&mut self.connections, None); },
                    _ => { }
                }
            }

            _ => { }

        }
        RackSignal::Active
    }
}

pub struct UnitHolder {
    units: Vec<RefCell<UnitContainer>>
}

impl UnitHolder {
        pub fn new() -> UnitHolder {
            UnitHolder {
                units: Vec::new()
            }
        }

        pub fn add_unit(&mut self, unit: Box<ProcessorUnit>) -> u32 {
            self.units.push(RefCell::new(UnitContainer::new(unit)));

            self.units.len() as u32
        }

        pub fn get_unit(&mut self, index: usize) -> RefMut<UnitContainer> {
            self.units[index].borrow_mut()
        }

        pub fn get_size(&self) -> i32 {
            self.units.len() as i32
        }
}

pub fn cycle_rack(holder: &mut UnitHolder) {
    let mut unit = holder.get_unit(0);
    unit.deref_mut().rack_signal(RackSignal::AC);
}

