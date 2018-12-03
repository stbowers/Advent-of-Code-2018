extern crate aoc_utils;

use aoc_utils::file_utils;

struct Claim {
    pub id: u32,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl Claim {
    pub fn new(id: u32, x: u32, y: u32, width: u32, height: u32) -> Claim {
        return Claim {
            id: id,
            x: x,
            y: y,
            width: width,
            height: height,
        };
    }

    pub fn contains_point(&self, x: u32, y: u32) -> bool {
        return (x >= self.x)
            && (y >= self.y)
            && (x < self.x + self.width)
            && (y < self.y + self.height);
    }
}

fn main() {
    let mut buffer: String = String::new();
    let lines: Vec<&str> =
        file_utils::get_input("./input.txt", &mut buffer).expect("Error reading input file");

    let claims: Vec<Claim> = get_claims(&lines);
    let overlap: u32 = count_overlap(&claims);
    let isolated_claim: u32 = find_isolated_claim_id(&claims);

    println!("Claims: {}", claims.len());

    println!("Overlap: {} in^2", overlap);

    println!("Isolated Claim: #{}", isolated_claim);
}

fn find_isolated_claim_id(claims: &Vec<Claim>) -> u32 {
    for claim in claims.iter() {
        let mut overlap: bool = false;
        for x in claim.x..(claim.x + claim.width) {
            for y in claim.y..(claim.y + claim.height) {
                let num_claims = claims
                    .iter()
                    .filter(|claim| claim.contains_point(x, y))
                    .count();

                if num_claims >= 2 {
                    overlap = true;
                    break;
                }
            }

            if overlap {
                break;
            }
        }

        if !overlap {
            return claim.id;
        }
    }

    return 0;
}

fn count_overlap(claims: &Vec<Claim>) -> u32 {
    let mut overlap: u32 = 0;

    let max_x: u32 = claims
        .iter()
        .map(|claim| claim.x + claim.width)
        .max()
        .unwrap();
    let max_y: u32 = claims
        .iter()
        .map(|claim| claim.y + claim.height)
        .max()
        .unwrap();

    for x in 0..(max_x + 1) {
        for y in 0..(max_y + 1) {
            let num_claims = claims
                .iter()
                .filter(|claim| claim.contains_point(x, y))
                .count();

            if num_claims >= 2 {
                overlap += 1;
            }
        }
    }
}

fn get_claims(lines: &Vec<&str>) -> Vec<Claim> {
    let mut claims: Vec<Claim> = Vec::new();
    for &line in lines.iter() {
        if line.len() > 0 {
            claims.push(parse_claim(line));
        }
    }

    return claims;
}

fn parse_claim(line: &str) -> Claim {
    let mut rest_of_line: &str = line;

    let mut line_scan: Vec<&str> = rest_of_line.splitn(2, "#").collect();
    rest_of_line = line_scan[1];

    line_scan = rest_of_line.splitn(2, " @ ").collect();
    rest_of_line = line_scan[1];
    let id: u32 = line_scan[0].parse::<u32>().unwrap();

    line_scan = rest_of_line.splitn(2, ",").collect();
    rest_of_line = line_scan[1];
    let x: u32 = line_scan[0].parse::<u32>().unwrap();

    line_scan = rest_of_line.splitn(2, ": ").collect();
    rest_of_line = line_scan[1];
    let y: u32 = line_scan[0].parse::<u32>().unwrap();

    line_scan = rest_of_line.splitn(2, "x").collect();
    rest_of_line = line_scan[1];
    let width: u32 = line_scan[0].parse::<u32>().unwrap();

    let height: u32 = rest_of_line.parse::<u32>().unwrap();

    return Claim::new(id, x, y, width, height);
}
