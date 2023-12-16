// absolutely terrible code. using a struct for this mightve been a mistake


fn main() {
    part1();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct XY {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, PartialEq)]
struct Ray {
    position: XY,
    next_relative: XY,
    to_be_removed: bool,
}

impl Ray {
    fn new(position: XY, next: XY) -> Ray {
        Ray {
            position,
            next_relative: next,
            to_be_removed: false,
        }
    }

    fn move_next(
        &mut self,
        grid: &Grid,
        new_rays_buffer: &mut Vec<Ray>,
        rays_history: &mut Vec<(XY, XY)>,
    ) {
        // if the ray has been here with the same position and direction, remove it
        if rays_history.contains(&(self.position, self.next_relative)) {
            self.to_be_removed = true;
            return;
        }

        // push current position to history
        rays_history.push((self.position, self.next_relative));

        let next_x = self.position.x + self.next_relative.x;
        let next_y = self.position.y + self.next_relative.y;

        // if its out of bounds, remove itself from the rays
        if next_x < 0
            || next_y < 0
            || next_y as usize >= grid.len()
            || next_x as usize >= grid[next_y as usize].len()
        {
            self.to_be_removed = true;
            return;
        }

        let next_char = grid[next_y as usize][next_x as usize];

        match next_char {
            '.' => {
                self.position.x += self.next_relative.x;
                self.position.y += self.next_relative.y;
            }
            '/' => {
                self.position.x += self.next_relative.x;
                self.position.y += self.next_relative.y;
                match self.next_relative {
                    XY { x: 1, y: 0 } => {
                        self.next_relative = XY { x: 0, y: -1 };
                    }
                    XY { x: 0, y: -1 } => {
                        self.next_relative = XY { x: 1, y: 0 };
                    }
                    XY { x: -1, y: 0 } => {
                        self.next_relative = XY { x: 0, y: 1 };
                    }
                    XY { x: 0, y: 1 } => {
                        self.next_relative = XY { x: -1, y: 0 };
                    }
                    _ => panic!("Invalid ray direction"),
                }
            }
            '\\' => {
                self.position.x += self.next_relative.x;
                self.position.y += self.next_relative.y;
                match self.next_relative {
                    XY { x: 1, y: 0 } => {
                        self.next_relative = XY { x: 0, y: 1 };
                    }
                    XY { x: 0, y: 1 } => {
                        self.next_relative = XY { x: 1, y: 0 };
                    }
                    XY { x: -1, y: 0 } => {
                        self.next_relative = XY { x: 0, y: -1 };
                    }
                    XY { x: 0, y: -1 } => {
                        self.next_relative = XY { x: -1, y: 0 };
                    }
                    _ => panic!("Invalid ray direction"),
                }
            }
            '-' => {
                if self.next_relative.y != 0 {
                    // remove itself from the rays, and create 2 new rays, shooting to the left and right
                    let new_ray_left = Ray::new(
                        XY {
                            x: next_x,
                            y: next_y,
                        },
                        XY { x: -1, y: 0 },
                    );
                    let new_ray_right = Ray::new(
                        XY {
                            x: next_x,
                            y: next_y,
                        },
                        XY { x: 1, y: 0 },
                    );
                    new_rays_buffer.push(new_ray_left);
                    new_rays_buffer.push(new_ray_right);

                    // remove itself from the rays
                    self.to_be_removed = true;
                } else {
                    self.position.x += self.next_relative.x;
                    self.position.y += self.next_relative.y;
                }
            }
            '|' => {
                if self.next_relative.x != 0 {
                    // remove itself from the rays, and create 2 new rays, shooting up and down
                    let new_ray_up = Ray::new(
                        XY {
                            x: next_x,
                            y: next_y,
                        },
                        XY { x: 0, y: -1 },
                    );
                    let new_ray_down = Ray::new(
                        XY {
                            x: next_x,
                            y: next_y,
                        },
                        XY { x: 0, y: 1 },
                    );
                    new_rays_buffer.push(new_ray_up);
                    new_rays_buffer.push(new_ray_down);

                    // remove itself from the rays
                    self.to_be_removed = true;
                } else {
                    self.position.x += self.next_relative.x;
                    self.position.y += self.next_relative.y;
                }
            }
            _ => panic!("Invalid char"),
        }
    }
}

type Grid = Vec<Vec<char>>;

fn print_history(grid: &Grid, history: &Vec<XY>) {
    let mut grid_history = grid.clone();

    for xy in history {
        grid_history[xy.y as usize][xy.x as usize] = '#';
    }

    for line in grid_history {
        println!("{}", line.iter().collect::<String>());
    }

    println!();
}

fn part1() {
    let file = std::fs::read_to_string("input.txt").unwrap();

    let grid = file
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Grid>();

    let start_ray = Ray::new(XY { x: -1, y: 0 }, XY { x: 1, y: 0 });
    let mut rays: Vec<Ray> = vec![start_ray];
    let mut rays_history: Vec<(XY, XY)> = vec![];

    loop {
        // println!("rays: {:?}", rays);
        // println!("rays_history: {:?}", rays_history);
        // println!("grid: {:?}", grid);

        // if there are no more rays, we are done
        if rays.len() == 0 {
            break;
        }

        let mut new_rays_buffer: Vec<Ray> = vec![];
        // move all the rays
        for ray in &mut rays {
            ray.move_next(&grid, &mut new_rays_buffer, &mut rays_history);
        }

        // remove the rays that are marked to be removed
        rays.retain(|ray| !ray.to_be_removed);

        // add the new rays to the rays
        rays.append(&mut new_rays_buffer);

        // if there are no more rays, we are done
        if rays.len() == 0 {
            break;
        }

        // std::thread::sleep(std::time::Duration::from_millis(100));
    }

    // remove the first entry, since it is the starting point and has x:-1
    rays_history.remove(0);


    print_history(&grid, &rays_history.iter().map(|(xy, _)| *xy).collect::<Vec<XY>>());
    println!("rays: {:?}", rays);

    // count the unique history positions
    let mut history_positions: Vec<XY> = rays_history.iter().map(|(xy, _)| *xy).collect();
    history_positions.sort();
    history_positions.dedup();

    println!("part1: {}", history_positions.len());
}
