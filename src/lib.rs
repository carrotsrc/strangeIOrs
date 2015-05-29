pub type PcmSample = (f32);

pub struct FeedBlock {
    out: String, 
    samples: Box<PcmSample>
}

pub enum UnitState {
    Idle,
    Init,
    Ready
}

pub trait RackUnit {
    fn init(&mut self);
    fn cycle(&mut self);

    fn get_unit_label(&self) -> &str;
    fn get_unit_type(&self) -> &str;

    fn unit_msg(&self, msg: &str) {
        println!("{} [{}]: {}", self.get_unit_label(), self.get_unit_type(), msg);
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

pub fn cycle_rack(holder: &mut UnitHolder) {

    let unit = holder.get_unit(0);
    unit.cycle();
}

