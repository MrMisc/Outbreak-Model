use std::thread;
use rand::distributions::Uniform;
use rand::distributions::{Distribution, Standard};
use rand::{thread_rng, Rng};
use statrs::distribution::{Normal, Poisson, StudentsT, Triangular, Weibull};


pub mod limits{
    pub fn min(a:f64,b:f64)->f64{
        if a<b{
            a
        }
        else{
            b
        }
    }
    pub fn max(a:f64,b:f64)->f64{
        if a<b{
            b
        }
        else{
            a
        }
    }
}

pub fn normal(mean: f64, std: f64, upper:f64) -> f64 {
    let mut thing: f64 = 0.0;
    loop {
        let mut rng = thread_rng();
        let v: &Vec<f64> = &Normal::new(mean, std)
            .unwrap()
            .sample_iter(&mut rng)
            .take(1)
            .collect();
        thing = v[0];
        if thing > 0.0 && thing<upper{
            break;
        }
    }
    thing
}

pub struct host{
    infected:bool,
    airborne:bool,
    zone:usize, //Possible zones denoted by ordinal number sequence
    prob1:f64,
    prob2:f64,  //standard deviation if required
    x:f64,
    y:f64
}

const LISTOFPROBABILITIES:[f64;5] = [0.1,0.2,0.05,0.03,0.15];
const GRIDSIZE:[f64;2] = [100.0,100.0];
const MAX_MOVE:f64 = 10.0;

const MEAN_MOVE:f64 = 2.0;
const STD_MOVE:f64 = 1.0;

const TRANSFER_DISTANCE: f64 = 2.0;//maximumm distance over which hosts can trasmit diseases to one another

impl host{
    fn transfer(&self)->bool{
        let mut rng = thread_rng();
        let roll = Uniform::new(0.0, 1.0);
        let rollnumber: f64 = rng.sample(roll);
        rollnumber < LISTOFPROBABILITIES[self.zone] && self.infected
    }
    fn new(zone:usize, std:f64,loc_x:f64, loc_y:f64)->host{
        let prob:f64 = LISTOFPROBABILITIES[zone.clone()];
        host{infected:false,airborne:false,zone:zone,prob1:prob,prob2:std,x:loc_x,y:loc_y}
    }
    fn new_inf(zone:usize, std:f64,loc_x:f64, loc_y:f64)->host{
        let prob:f64 = LISTOFPROBABILITIES[zone.clone()];
        host{infected:true,airborne:false,zone:zone,prob1:prob,prob2:std,x:loc_x,y:loc_y}
    }
    fn shuffle(self)->host{
        let mut rng = thread_rng();
        let roll = Uniform::new(0.0, 1.0);
        let rollnumber: f64 = rng.sample(roll);
        let mult:f64 = match rollnumber{
            0.5.. => -1.0,
            _ => 1.0
        };
        let new_x: f64 = limits::min(limits::max(0.0,self.x+mult*normal(MEAN_MOVE,STD_MOVE,MAX_MOVE)),GRIDSIZE[0]);
        let new_y:f64 = limits::min(limits::max(0.0,self.y+mult*normal(MEAN_MOVE,STD_MOVE,MAX_MOVE)),GRIDSIZE[1]);
        host{infected:self.infected,airborne:self.airborne,zone:self.zone,prob1:self.prob1,prob2:self.prob2,x:new_x,y:new_y}
    }
    fn dist(host1: &host, host2: &host)->bool{
        let diff_x: f64 = host1.x -host2.x;
        let diff_y: f64 = host1.y - host2.y;
        let t: f64 = diff_x.powf(2.0)+diff_y.powf(2.0);
        t.powf(0.5)<=TRANSFER_DISTANCE

    }
    // fn transmit(population:Vec<host>)->Vec<host>{

    // }
}


fn main() {
    let mut chicken:host = host::new(1,0.2,2.0,2.0);
    let mut chicken2: host = host::new_inf(1,0.2,50.0,50.0);
    for j in 0..10000{
        println!(" Chicken is currently located at {:.2} @x and {:.2} @y", chicken.x,chicken.y);
        println!(" Chicken 2 is currently located at {:.2} @x and {:.2} @y", chicken2.x,chicken2.y);
        let CON = host::dist(&chicken,&chicken2);
        println!("Are chickens in contact distance? : {}", CON);
        if CON{
            println!("CONTACT CONFIRMED!");
            println!("Took {} minutes!",j);
            break;
        }
        chicken = chicken.shuffle();
        chicken2 = chicken2.shuffle();
    }
    
    println!("{} is the probability tied to the selected zone for this host!", chicken.prob1);
    println!("Hello, world!");
}
