use advent_of_code::Pos2D;

advent_of_code::solution!(12);

#[derive(Clone, Debug)]
struct Farm {
    regions: Vec<Region>,
    serial_id: u32,
}

impl Farm {
    fn create_region(&mut self, plot: &Pos2D, plant_type: char) -> u32 {
        let region = Region::new(self.serial_id, plot, plant_type);
        let region_id = region.id;
        self.regions.push(region);

        self.serial_id += 1;
        region_id
    }

    fn get_region(&self, plot: &Pos2D) -> Option<&Region> {
        self.regions.iter().find(|r| r.contains(plot))
    }

    fn left_pos(&self, position: &Pos2D) -> Option<Pos2D> {
        if position.x == 0 {
            return None;
        }

        Some(Pos2D {
            x: position.x - 1,
            y: position.y,
        })
    }

    fn top_pos(&self, position: &Pos2D) -> Option<Pos2D> {
        if position.y == 0 {
            return None;
        }

        Some(Pos2D {
            x: position.x,
            y: position.y - 1,
        })
    }

    fn get_sorted_regions(&self, first: u32, second: u32) -> (u32, u32) {
        let first = self.regions.iter().find(|r| r.id == first).unwrap();
        let second = self.regions.iter().find(|r| r.id == second).unwrap();

        if first.get_area() >= second.get_area() {
            (first.id, second.id)
        } else {
            (second.id, first.id)
        }
    }

    fn merge_regions(&mut self, first: u32, second: u32) {
        let (parent_id, child_id) = self.get_sorted_regions(first, second);

        let child_idx = self.regions.iter().position(|r| r.id == child_id).unwrap();
        let child = self.regions.iter_mut().find(|r| r.id == child_id).unwrap();
        let child_perim = child.get_perimeter();

        if parent_id != child_id {
            let mut plots = Vec::new();
            while child.plots.len() > 0 {
                let plot = child.plots.pop().unwrap();
                plots.push(plot);
            }

            let parent = self.regions.iter_mut().find(|r| r.id == parent_id).unwrap();
            for plot in plots.iter() {
                parent.plots.push(plot.clone());
            }

            parent.perimeter += child_perim - 2;
            // println!("{}", parent.perimeter);
            self.regions.remove(child_idx);
        } else {
            let parent = self.regions.iter_mut().find(|r| r.id == parent_id).unwrap();
            parent.perimeter -= 2;
            // println!("{}", parent.perimeter);
        }
    }
}

#[derive(Clone, Debug)]
struct Region {
    id: u32,
    plant_type: char,
    plots: Vec<Pos2D>,
    perimeter: usize,
}

impl Region {
    fn new(id: u32, plot: &Pos2D, plant_type: char) -> Region {
        Region {
            id,
            plant_type,
            plots: vec![plot.clone()],
            perimeter: 4,
        }
    }

    fn get_area(&self) -> usize {
        self.plots.len()
    }

    fn get_perimeter(&self) -> usize {
        self.perimeter
    }

    fn get_fence_price(&self) -> u32 {
        let res = self.get_area() * self.get_perimeter();
        res.try_into().unwrap()
    }

    fn contains(&self, plot: &Pos2D) -> bool {
        self.plots.iter().any(|p| p.x == plot.x && p.y == plot.y)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut farm = Farm {
        serial_id: 0,
        regions: Vec::new(),
    };

    for (y, line) in input.lines().enumerate() {
        for (x, plant_type) in line.chars().enumerate() {
            let plot = Pos2D::new(x as i32, y as i32);
            let new_region_id = farm.create_region(&plot, plant_type);

            // println!("Found `{}` at (x: {}, y: {})", plant_type, plot.x, plot.y);

            if let Some(left) = farm.left_pos(&plot) {
                if let Some(left_region) = farm.get_region(&left) {
                    if left_region.plant_type == plant_type {
                        farm.merge_regions(left_region.id, new_region_id);
                    }
                }
            }

            let active_region = farm.get_region(&plot).unwrap();
            if let Some(top) = farm.top_pos(&plot) {
                if let Some(top_region) = farm.get_region(&top) {
                    if top_region.plant_type == active_region.plant_type {
                        farm.merge_regions(top_region.id, active_region.id);
                    }
                }
            }
        }
    }

    let mut total = 0;
    for region in farm.regions {
        total += region.get_fence_price();
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
