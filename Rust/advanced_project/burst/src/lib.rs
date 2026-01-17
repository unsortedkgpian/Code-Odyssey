struct Burst{

}

struct MachineSet{

}

struct BurstBuilder{
    descriptors: Vec<MachineSet>
}

impl Default for BurstBuilder{
    fn default() -> Self {
        BurstBuilder {
            descriptors: Vec::new(),
        }
    }
}

impl BurstBuilder {
    // pub fn new()-> Self {}

    pub fn add_set(&mut self, description:MachineSet) {}

    pub fn run() {}
}


fn main(){
    let mut b = BurstBuilder::default();
    b.add_set(MachineSet::new("t2.micro", "ami-18aa89b", |ssh| {
        ssh.exec("sudo apt install htop");
    }))
    b.run(||);
}
