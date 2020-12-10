/// Advent of Code Day 7
/// 
/// Day 7 input is a list of lines in the format "{color} bags contain x {color2} bag, x {color3} bag, etc"
/// 
/// My solution for this is a huge mess of linked Rc<RefCells>>, was fun to write though.

use std::io::Error;
use std::fs;

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

/// Day 7 Part 2 problem is finding how many bags can eventually contain at least one shiny gold bag.
pub fn part_one() -> Result<usize, Error> {
    let input = fs::read_to_string("./day7_input.txt")?;
    let lines: Vec<&str> = input.split("\n").collect();

    let bags_map = build_bag_map_from_input(lines);
    Ok(bags_map.values().filter(|b| b.borrow().holds_gold()).count())
}

/// Day 7 Part 2 needs to figure out how many bags your shiny gold bag can hold. (all nested bags)
pub fn part_two() -> Result<usize, Error> {
    let input = fs::read_to_string("./day7_input.txt")?;
    let lines: Vec<&str> = input.split("\n").collect();

    
    let bags_map = build_bag_map_from_input(lines);
    let holds = bags_map.get("shiny gold").unwrap().borrow().holds_how_many();
    Ok(holds) 
}


// Helper function for day 7 problem, builds a map of {bag_color: Bag}, where each bag contains whatever other bags it holds and the amount
fn build_bag_map_from_input(lines: Vec<&str>) -> HashMap<String, Rc<RefCell<Bag>>> {

    // Create a new bag map which holds {bag_color: Bag}
    let mut bags_map: HashMap<String, Rc<RefCell<Bag>>> = HashMap::new();

    // Iterate each line of bag information, it will be in the format of "{color} bags contain {x} {color} bags, etc, etc."
    for line in lines {

        // Remove dots from line, and split it at contain to separate the current bag color, and the bags it contains.
        let line = line.replace(".", "");
        let split: Vec<&str> = line.split("contain").collect();

        // Parse color of current bag out of split[1].
        let color: String = split[0].split(" ").filter(|s| s.len() > 0 && *s != "bags").fold(String::new(), |w, s| w + s + " ");
        let color = color.trim_end().to_string();

        // Parse the color of bags current bag holds, as well as the count. Filters out bags which don't contain any others. Returns a Vec of (bag_color, count).
        let holds: Vec<&str> = split[1].split(",").collect();
        let holds: Vec<(String, usize)> = holds.iter().map(|s| {
            let (mut color, mut count) = (String::new(), 0);
            for i in s.split(" ").filter(|s| s.len() > 0 && *s != "bags" && *s != "bag") {
                if let Ok(v) = i.parse::<usize>() {
                    count = v;
                } else {
                    color = color + " " + i;
                }
            }
            let color = color.trim().to_string();
            (color, count)
        }).filter(|b| b.0 != "no other").collect();


        // If the current main color bag doesn't exist in our bags_map, create it.
        if bags_map.get(&color).is_none() {
            bags_map.insert(
                color.clone(),
                Rc::new(RefCell::new(Bag::from(color.clone())))
            );
        }

        // Iterate the vec of (bag_color, count) and get/create the Bag for each one, then add a reference to it to the current main bag.
        for hb in holds {
            
            // Get the &Rc to the current bag. If it the current bag doesn't exist create it.
            if bags_map.get(&hb.0).is_none() {
                bags_map.insert(
                    hb.0.clone(), 
                    Rc::new(RefCell::new(Bag::from(hb.0.clone())))
                );
            }
            let bag = bags_map.get(&hb.0).unwrap();

            // Add the current 'hold bag' to the actual bag we are iterating.
            bags_map.get(&color).unwrap()
                    .borrow_mut()
                    .holds
                    .insert(hb.0.clone(), (hb.1, Rc::clone(bag)));
        }
  
    }
    bags_map
}

#[derive(Debug)]
struct Bag {
    color: String,
    holds: HashMap<String, (usize, Rc<RefCell<Bag>>)>
}


impl Bag {
    fn from(color: String) -> Bag {
        Bag {
            color,
            holds: HashMap::new()
        }
    }

    // If this bag holds gold directly, or if any bag it holds holds gold, return true.
    fn holds_gold(&self) -> bool {
        if !self.holds.get("shiny gold").is_none() {
            return true
        }
        if self.holds.values().map(|x| {x.1.borrow().holds_gold()}).any(|x|x) {
            return true
        }
        return false
    }

    // Returns the amount of bags this bag holds (includes nested bag capacities).
    fn holds_how_many(&self) -> usize {
        self.holds.values().map(|b| {
            (b.0 * b.1.borrow().holds_how_many()) + b.0
        }).sum()
    }
}
