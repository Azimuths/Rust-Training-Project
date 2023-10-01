fn get_modifier(&mut self) -> i32 {
    let deep = 1..=5;
    let low = 6..=10;
    let mid = 11..=15;
    let high = 16..=20;
    let mut money = 0;

    if self.roll == 1 {
        money = -3;
        if self.up {
            self.multiplier += 1;
        } else {
            self.up = false;
            self.multiplier = 1;
        }
        self.skip = true;
    } else if self.roll == 20 {
        money = 3;
        self.multiplier += 1;
        self.skip = true;
    } else if deep.contains(&self.roll) {
        money = -3;
    } else if low.contains(&self.roll) {
        money = -1;
    } else if mid.contains(&self.roll) {
        money = 1;
        // Check for special multiplier conditions for 11+
        if self.values.len() >= 2 {
            let len = self.values.len();
            let last_three = &self.values[len - 2..len];
            if last_three.iter().all(|&x| x >= 11) {
                self.multiplier *= 3;
            }
        }
    } else if high.contains(&self.roll) {
        money = 3;
    } else {
        println!("Error");
    }
    return money;
}

fn get_multiplier(&mut self) -> i32 {
    let mut index = 0;

    println!("The values are {:?}", self.values);
    if self.skip {
        self.skip = false;
        return self.multiplier;
    }
    if self.values.len() < 3 {
        return self.multiplier;
    }
    if self.values.len() > 3 {
        index = self.values.len() - 3;
    }
    println!("The index is {}", index);
    println!(
        "The values are {}, {}, {}",
        self.values[index],
        self.values[index + 1],
        self.values[index + 2]
    );
    
    // Check for special multiplier conditions for 11+
    if self.values[index] >= 11 && self.values[index + 1] >= 11 && self.values[index + 2] >= 11 {
        self.multiplier *= 3;
        // Check if a 20 follows or three more 11+ values follow
        if self.values.len() >= index + 4 {
            let next_three = &self.values[index + 3..index + 6];
            if self.values[index + 3] == 20 || next_three.iter().all(|&x| x >= 11) {
                self.multiplier *= 3;
            }
        }
    } else {
        self.multiplier = 1;
    }
    
    return self.multiplier;
}
