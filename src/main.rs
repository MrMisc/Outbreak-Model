use std::thread;
use rand::distributions::Uniform;
use rand::distributions::{Distribution, Standard};
use rand::{thread_rng, Rng};
use statrs::distribution::{Normal, Poisson, StudentsT, Triangular, Weibull};

extern crate serde;
extern crate serde_json;
use serde::Deserializer;
use serde::{Deserialize, Serialize};
use serde_json::json;

// use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::time::{Duration, Instant};
use std::{fs, io, process};
use std::error::Error;

use csv::Writer;


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
#[derive(Clone)]
pub struct host{
    infected:bool,
    motile:u8,
    zone:usize, //Possible zones denoted by ordinal number sequence
    prob1:f64,  //Probability of contracting disease - these are tied to zone if you create using .new() implementation within methods
    prob2:f64,  //standard deviation if required OR second probabiity value for transferring in case that is different from prob1
    x:f64,
    y:f64,
    age:f64
}

//Space
const LISTOFPROBABILITIES:[f64;5] = [0.1,0.75,0.05,0.03,0.15]; //Probability of transfer of samonella per zone - starting from zone 0 onwards
const GRIDSIZE:[f64;2] = [3000.0,3000.0];
const MAX_MOVE:f64 = 25.0;
const MEAN_MOVE:f64 = 5.0;
const STD_MOVE:f64 = 10.0;
//Disease 
const TRANSFER_DISTANCE: f64 = 1.0;//maximum distance over which hosts can trasmit diseases to one another
//Collection
const AGE_OF_HOSTCOLLECTION: f64 = 30.0*24.0;  //For instance if you were collecting chickens every 15 days
const AGE_OF_DEPOSITCOLLECTION:f64 = 1.0*24.0; //If you were collecting their eggs every 3 days
const FAECAL_CLEANUP_FREQUENCY:usize = 4; //How many times a day do you want faecal matter to be cleaned up?
//Resolution
const STEP:usize = 20;  //Number of chickens per unit distance
const HOUR_STEP: f64 = 4.0; //Number of times chickens move per hour
const LENGTH: usize = 45*24; //How long do you want the simulation to be?
impl host{
    fn transport(&mut self){
        //Where in the new zone are they transported? Presumably one door
        self.x = 0.0;
        self.y = 0.0;
        self.zone=limits::min((self.zone+1) as f64,LISTOFPROBABILITIES.len() as f64) as usize;
    }
    fn transfer(&self)->bool{ //using prob1 as the probability of contracting disease  (in other words, no separation of events between transferring and capturing disease. If something is infected, it is always infected. Potentially.... the prospective new host will not get infected, but the INFECTED is always viably transferring)
        let mut rng = thread_rng();
        let roll = Uniform::new(0.0, 1.0);
        let rollnumber: f64 = rng.sample(roll);
        // println!("DISEASE   {}",rollnumber);
        rollnumber < self.prob1
    }
    fn new(zone:usize, std:f64,loc_x:f64, loc_y:f64)->host{
        let prob:f64 = LISTOFPROBABILITIES[zone.clone()];
        //Add a random age generator
        host{infected:false,motile:0,zone:zone,prob1:prob,prob2:std,x:loc_x,y:loc_y,age:normal(5.0*24.0,3.0*24.0,11.0*24.0)}
    }
    fn new_inf(zone:usize, std:f64,loc_x:f64, loc_y:f64)->host{
        let prob:f64 = LISTOFPROBABILITIES[zone.clone()];
        host{infected:true,motile:0,zone:zone,prob1:prob,prob2:std,x:loc_x,y:loc_y,age:normal(5.0*24.0,3.0*24.0,11.0*24.0)}
    }
    fn deposit(self, consumable: bool)->host{ //Direct way to lay deposit from host. The function is 100% deterministic and layering a probability clause before this is typically expected
        let zone = self.zone.clone();
        let prob1 = self.prob1.clone();
        let prob2 = self.prob2.clone();
        let x = self.x.clone();
        let y = self.y.clone();
        let inf = self.infected.clone();
        // println!("EGG BEING LAID");
        if consumable{host{infected:inf,motile:1,zone:zone,prob1:prob1,prob2:prob2,x:x,y:y,age:0.0}}
        else{host{infected:inf,motile:2,zone:zone,prob1:prob1,prob2:prob2,x:x,y:y,age:0.0}}
    }
    fn deposit_all(vector:Vec<host>)->Vec<host>{
        //Below is an example whereby hosts deposit twice a day (fecal matter and laying eggs each once per day as an example)
        let mut vecc:Vec<host> = vector.clone();
        let mut vecc_into: Vec<host> = vector.clone().into_iter().filter(|x| x.motile==0).collect::<Vec<_>>(); //With this re are RETAINING the hosts and deposits within the original vector

        //.map wasn't working so we brute forced a loop
        for ele in vecc_into{
            let mut rng = thread_rng();
            let roll = Uniform::new(0.0,1.0);
            let rollnumber: f64 = rng.sample(roll);
            if rollnumber<1.0/24.0{//once per 24h or 1 day rate
                // println!("EGG BEING LAID");
                let no:usize = normal(1.0,0.5,2.0) as usize;
                for deposits in 0..no{
                    vecc.push(ele.clone().deposit(true)); //consumable once per day rate
                }
            }
            let mut rng = thread_rng();
            let roll = Uniform::new(0.0,1.0);
            let rollnumber: f64 = rng.sample(roll);
            if rollnumber<1.0/24.0{
                // println!("EGG BEING LAID");
                let no:usize = normal(1.0,0.5,2.0) as usize;
                for deposits in 0..no{
                    vecc.push(ele.clone().deposit(false));//non consumable excrement once per day rate
                }
            }
        }
        vecc
    }
    fn shuffle(mut self)->host{
        if self.motile==0{
            //Whether the movement is negative or positive
            let mut rng = thread_rng();
            let roll = Uniform::new(0.0, 2.4);
            let rollnumber: f64 = rng.sample(roll);
            let mult:f64 = match rollnumber{
                0.0..=0.4 => -1.0,
                0.4..=0.8 => 1.0,
                _ => 0.0
            };
            // //Perhaps they are just standing still - set it to 33% chance that chicken moves
            // let mut rng = thread_rng();
            // let roll = Uniform::new(0.0, 1.0);
            // let rollnumber: f64 = rng.sample(roll);
            // let mult2:f64 = match rollnumber{
            //     0.3.. => 1.0,
            //     _ => 0.0
            // };            
            //use truncated normal distribution (which has been forced to be normal) in order to change the values of x and y accordingly of the host - ie movement
            let new_x: f64 = limits::min(limits::max(0.0,self.x+mult*normal(MEAN_MOVE,STD_MOVE,MAX_MOVE)),GRIDSIZE[0]);
            let new_y:f64 = limits::min(limits::max(0.0,self.y+mult*normal(MEAN_MOVE,STD_MOVE,MAX_MOVE)),GRIDSIZE[1]);
            host{infected:self.infected,motile:self.motile,zone:self.zone,prob1:self.prob1,prob2:self.prob2,x:new_x,y:new_y,age:self.age+1.0/HOUR_STEP}}
        else{
            //deposits by hosts do not move obviously, but they DO age, which affects collection
            self.age += 1.0/HOUR_STEP;
            self
        }
    }
    fn shuffle_all(vector: Vec<host>)->Vec<host>{
        vector.into_iter().map(|x| x.shuffle()).collect()
    }
    fn dist(host1: &host, host2: &host)->bool{
        let diff_x: f64 = host1.x -host2.x;
        let diff_y: f64 = host1.y - host2.y;
        let t: f64 = diff_x.powf(2.0)+diff_y.powf(2.0);
        /////
        //PRINT STATEMENT
        // if t.powf(0.5)<=TRANSFER_DISTANCE{
        //     println!("{} {} vs {} {}",&host1.x,&host1.y,&host2.x,&host2.y);
        // }
        ////
        t.powf(0.5)<=TRANSFER_DISTANCE && host1.zone == host2.zone
    }
    fn transmit(mut inventory:Vec<host>,time:usize)->Vec<host>{//Current version logic: Once the diseased host passes the "test" in fn transfer, then ALL other hosts within distance contract
        //Locate all infected hosts
        let mut cloneof: Vec<host> = inventory.clone();
        cloneof = cloneof.into_iter().filter_map(|mut x|{
            if x.infected{ //x.transfer is how we internalise the probabilistic nature (not definitive way) that a disease can or cannot spread from an infected individual
                Some(x)
            }else{
                None
            }
        }).collect();
        inventory = inventory.into_iter().filter(|x| !x.infected).collect::<Vec<host>>();    
        inventory = inventory.into_iter().filter_map(|mut x|{
            if cloneof.iter().any(|inf| host::dist(&inf,&x)){
                let before = x.infected.clone();
                x.infected=x.transfer();
                if !before && x.infected{
                    println!("{} {} {}",x.x,x.y,time);
                }
                // println!("{} vs {}",&inf.x,&x.x,&inf.y,&x.y);
                Some(x)
            }else{
                Some(x)
            }
        }).collect();
        inventory.extend(cloneof);
        inventory
    }
    fn cleanup(inventory:Vec<host>)->Vec<host>{
        inventory.into_iter().filter_map(|mut x|{
            if x.motile==2{
                None
            }else{
                Some(x)
            }
        }).collect()
    }
    fn collect(inventory:Vec<host>)->[Vec<host>;2]{   //hosts and deposits potentially get collected
        let mut collection:Vec<host> = Vec::new();
        let vec1:Vec<host> = inventory.into_iter().filter_map(|mut x| {
            if x.motile==0 && x.age>AGE_OF_HOSTCOLLECTION{
                // println!("Collecting host(s)...{} days old",x.age/24.0);
                collection.push(x);
                None
            }else if x.motile == 1 && x.age>AGE_OF_DEPOSITCOLLECTION{
                // println!("Collecting deposit(s)...");
                collection.push(x);
                None
            }else{
                Some(x)
            }
        }).collect();
        [vec1,collection]  //collection vector here to be added and pushed into the original collection vector from the start of the loop! This function merely outputs what should be ADDED to collection!
    }
    fn report(inventory:&Vec<host>)->[f64;4]{ //simple function to quickly return the percentage of infected hosts
        let inf: f64 = inventory.clone().into_iter().filter(|x| {
            x.infected && x.motile==0
        }).collect::<Vec<_>>().len() as f64;
        let noofhosts: f64 = inventory.clone().into_iter().filter(|x| {
            x.motile==0
        }).collect::<Vec<_>>().len() as f64;

        let inf2: f64 = inventory.clone().into_iter().filter(|x| {
            x.infected && x.motile==1
        }).collect::<Vec<_>>().len() as f64;
        let noofhosts2: f64 = inventory.clone().into_iter().filter(|x| {
            x.motile==1
        }).collect::<Vec<_>>().len() as f64;        

        [inf/(noofhosts+1.0),inf2/(noofhosts2+1.0),noofhosts,noofhosts2]
    }
}

// fn main_(){
//     //generate chickens 10 units apart from one another
//     let mut chickens: Vec<host> = Vec::new();
//     let mut feast: Vec<host> =  Vec::new();
//     let step = STEP;
//     for i in (0..GRIDSIZE[0] as u64).step_by(step){
//         // println!("{}",i as f64)
//         for j in (0..GRIDSIZE[1] as u64).step_by(step){
//             chickens.push(host::new(1,0.2,i as f64,j as f64));
//         }
//     }
//     chickens.push(host::new_inf(1,0.2,55.0,55.0)); // the infected
//     for time in (0..30*24){
//         let mut collect: Vec<host> = Vec::new();
//         for _ in 0..4{
//             chickens = host::shuffle_all(chickens);
//             chickens = host::transmit(chickens);
//         } //Say chickens move/don't move every 15min - 4 times per hour
//         chickens = host::deposit_all(chickens);
//         [chickens,collect] = host::collect(chickens);
//         //Collect the hosts and deposits as according
//         feast.append(&mut collect);
//         if chickens.len()>10{
//             let perc = host::report(&chickens)[0]*100.0;
//             let total_hosts = host::report(&chickens)[2];
//             let no = host::report(&chickens)[0]*total_hosts;
//             println!("{}% of the chickens in the farm are infected, out of {} total - {} infected",perc,total_hosts,no);
//             let perc = host::report(&chickens)[1]*100.0;
//             let total_hosts = host::report(&chickens)[3];
//             let no = host::report(&chickens)[1]*total_hosts;            
//             println!("{}% of the eggs in the farm are infected, out of {} total - {} infected",perc,total_hosts,no);
//         }
//         if host::report(&chickens)[0]>0.95{
//             println!("It took {} days for 95% contamination of hosts to be contaminated", (time as f64)/(24.0));
//             break;
//         }

        
//     }

//     println!("########################SUMMARY STATISTICS################################");
//     if chickens.len()>10{println!("By the end of 12 days, {}% of hosts ended up contaminated.",host::report(&chickens)[0]*100.0);}
//     //Report end of collection contamination
//     let perc = host::report(&feast)[0]*100.0;
//     let total_hosts = host::report(&feast)[2];
//     let no = host::report(&feast)[0]*total_hosts;


//     println!("{}% of the chickens in the collection are infected, out of {} total - {} infected",perc,total_hosts,no);

//     let perc = host::report(&feast)[1]*100.0;
//     let total_hosts = host::report(&feast)[3];
//     let no = host::report(&feast)[1]*total_hosts;            
//     println!("{}% of the eggs in the farm are collection, out of {} total - {} infected",perc,total_hosts,no);    

//     println!("{} total number of collected hosts and deposits",feast.len());
// }


fn main(){
    //generate chickens 10 units apart from one another
    let mut chickens: Vec<host> = Vec::new();
    let mut feast: Vec<host> =  Vec::new();
    let step = STEP;
    //GENERATE CLEAN HOSTS
    for i in (0..GRIDSIZE[0] as u64).step_by(step){
        // println!("{}",i as f64)
        for j in (0..GRIDSIZE[1] as u64).step_by(step){
            chickens.push(host::new(1,0.2,i as f64,j as f64));
        }
    }
    //GENERATE INFECTED HOST
    chickens.push(host::new_inf(1,0.2,GRIDSIZE[0]/2.0,GRIDSIZE[1]/2.0)); // the infected

    //CSV FILE
    let filestring: String = format!("./output.csv");
    if fs::metadata(&filestring).is_ok() {
        fs::remove_file(&filestring).unwrap();
    }
    // Open the file in append mode for writing
    let mut file = OpenOptions::new()
    .write(true)
    .create(true)
    .append(true) // Open in append mode
    .open(&filestring)
    .unwrap();
    let mut wtr = Writer::from_writer(file);
    for time in 0..LENGTH{
        let mut collect: Vec<host> = Vec::new();
        for _ in 0..HOUR_STEP as usize{
            chickens = host::shuffle_all(chickens);
            chickens = host::transmit(chickens,time.clone());
        } //Say chickens move/don't move every 15min - 4 times per hour
        chickens = host::deposit_all(chickens);
        [chickens,collect] = host::collect(chickens);
        //Collect the hosts and deposits as according
        feast.append(&mut collect);
        //Farm
        let perc = host::report(&chickens)[0]*100.0;
        let total_hosts = host::report(&chickens)[2];
        let no = host::report(&chickens)[0]*total_hosts;
        let perc2 = host::report(&chickens)[1]*100.0;
        let total_hosts2 = host::report(&chickens)[3];
        let no2 = host::report(&chickens)[1]*total_hosts2;        
        //Collection
        let _perc = host::report(&feast)[0]*100.0;
        let _total_hosts = host::report(&feast)[2];
        let _no = host::report(&feast)[0]*_total_hosts;
        let _perc2 = host::report(&feast)[1]*100.0;
        let _total_hosts2 = host::report(&feast)[3];
        let _no2 = host::report(&feast)[1]*_total_hosts2;            
        // println!("{} {} {} {} {} {}",perc,total_hosts,no,perc2,total_hosts2,no2);    
        // println!("{} {} {} {} {} {} {} {} {} {} {} {}",perc,total_hosts,no,perc2,total_hosts2,no2,_perc,_total_hosts,_no,_perc2,_total_hosts2,_no2);
        wtr.write_record(&[
            perc.to_string(),
            total_hosts.to_string(),
            no.to_string(),
            perc2.to_string(),
            total_hosts2.to_string(),
            no2.to_string(),
            _perc.to_string(),
            _total_hosts.to_string(),
            _no.to_string(),
            _perc2.to_string(),
            _total_hosts2.to_string(),
            _no2.to_string(),
        ])
        .unwrap();
        if time % (24/FAECAL_CLEANUP_FREQUENCY) ==0{
            chickens = host::cleanup(chickens);
        }
        // if host::report(&chickens)[2]<5.0{break;}
    }
    wtr.flush().unwrap();
    println!("{} {} {}",GRIDSIZE[0],GRIDSIZE[1],LENGTH);

    
    // Open a file for writing
    let mut file = File::create("parameters.txt").expect("Unable to create file");

    // Write constants to the file
    // Space
    writeln!(file, "## Space").expect("Failed to write to file");
    writeln!(file, "- LISTOFPROBABILITIES: {:?} (Probability of transfer of salmonella per zone)", LISTOFPROBABILITIES).expect("Failed to write to file");
    writeln!(file, "- GRIDSIZE: {:?} (Size of the grid)", GRIDSIZE).expect("Failed to write to file");
    writeln!(file, "- MAX_MOVE: {} (Maximum move value)", MAX_MOVE).expect("Failed to write to file");
    writeln!(file, "- MEAN_MOVE: {} (Mean move value)", MEAN_MOVE).expect("Failed to write to file");
    writeln!(file, "- STD_MOVE: {} (Standard deviation of move value)", STD_MOVE).expect("Failed to write to file");

    // Disease
    writeln!(file, "\n## Disease").expect("Failed to write to file");
    writeln!(file, "- TRANSFER_DISTANCE: {} (Maximum distance for disease transmission)", TRANSFER_DISTANCE).expect("Failed to write to file");

    // Collection
    writeln!(file, "\n## Collection").expect("Failed to write to file");
    writeln!(file, "- AGE_OF_HOSTCOLLECTION: {} days", AGE_OF_HOSTCOLLECTION/24.0).expect("Failed to write to file");
    writeln!(file, "- AGE_OF_DEPOSITCOLLECTION: {} days", AGE_OF_DEPOSITCOLLECTION/24.0).expect("Failed to write to file");
    writeln!(file, "- FAECAL_CLEANUP_FREQUENCY: {} times per day", 24/FAECAL_CLEANUP_FREQUENCY).expect("Failed to write to file");

    // Resolution
    writeln!(file, "\n## Resolution").expect("Failed to write to file");
    writeln!(file, "- STEP: {} (Chickens per unit distance)", STEP).expect("Failed to write to file");
    writeln!(file, "- HOUR_STEP: {} (Chickens move per hour)", HOUR_STEP).expect("Failed to write to file");
    writeln!(file, "- LENGTH: {} (Simulation duration in hours)", LENGTH).expect("Failed to write to file");


}
