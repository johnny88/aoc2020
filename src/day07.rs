use std::collections::HashMap;

const CONTAINS: &str = " contain ";
const EMPTY_BAG: &str = "no other";
const BAGS: &str = "bags";
const BAG: &str = "bag";

struct Child {
    name: String,
    amount: usize,
}

type ChildHash = HashMap<String, Vec<Child>>;

type ParentHash = HashMap<String, Vec<String>>;

fn get_parents(h: &ParentHash, bag: String) -> Vec<String> {
    let bag_parents = match h.get(&bag) {
        Some(vector) => vector,
        None => return vec![],
    };
    let mut vec_parents = bag_parents.to_vec();
    for b in bag_parents {
        let v = get_parents(&h, b.to_string());
        for item in v {
            match vec_parents.contains(&item) {
                true => (),
                false => vec_parents.push(item),
            }
        }
    }
    vec_parents
}

pub fn part1(input: String) {
    let mut h: ParentHash = HashMap::new();

    for l in input.lines().collect::<Vec<_>>().iter() {
        let s_clean = l.replace(BAGS, "").replace(BAG, "").replace(".", "");
        let v = s_clean.split(CONTAINS).collect::<Vec<_>>();

        if v[1].contains(EMPTY_BAG) {
            continue;
        }

        let contents = v[1].split(',').collect::<Vec<_>>();

        for c in contents.iter() {
            let item = c.trim().splitn(2, " ").collect::<Vec<_>>();

            match h.contains_key(item[1]) {
                true => {
                    let parents = h.get_mut(item[1]).unwrap();
                    parents.push(v[0].trim().to_string());
                }
                false => {
                    h.insert(item[1].to_string(), vec![v[0].trim().to_string()]);
                }
            };
        }
    }

    println!("{}", get_parents(&h, "shiny gold".to_string()).len());
}

fn separate_parent_from_child(s: &str) -> (String, Vec<String>) {
    let s_clean = s.replace(BAGS, "").replace(BAG, "").replace(".", "");
    let v = s_clean.split(CONTAINS).collect::<Vec<_>>();

    let contents = v[1].split(',').collect::<Vec<_>>();
    (
        v[0].trim().to_string(),
        contents.iter().map(|x| x.to_string()).collect(),
    )
}

fn num_bags(h: &ChildHash, bag: String) -> usize {
    let bag_children = match h.get(&bag) {
        Some(vector) => vector,
        None => return 1,
    };
    bag_children.iter().fold(1, |mut acc, b| {
        acc += b.amount * num_bags(h, b.name.clone());
        acc
    })
}

pub fn part2(input: String) {
    let mut h: ChildHash = HashMap::new();

    for l in input.lines().collect::<Vec<_>>().iter() {
        let (parent, children) = separate_parent_from_child(*l);

        for c in children.iter() {
            if c.contains(EMPTY_BAG) {
                h.insert(parent.clone(), vec![]);
                continue;
            }
            let item = c.trim().splitn(2, " ").collect::<Vec<_>>();

            match h.contains_key(&parent) {
                true => {
                    let children = h.get_mut(&parent).unwrap();
                    children.push(Child {
                        name: item[1].trim().to_string(),
                        amount: item[0].parse::<usize>().unwrap(),
                    });
                }
                false => {
                    h.insert(
                        parent.clone(),
                        vec![Child {
                            name: item[1].trim().to_string(),
                            amount: item[0].parse::<usize>().unwrap(),
                        }],
                    );
                }
            };
        }
    }

    println!("{}", num_bags(&h, "shiny gold".to_string()) - 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CASE_1: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    const TEST_CASE_2: &str = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    #[test]
    fn day7_part_1_test_case_1() {
        println!("\n\n********************\n\n");
        part1(TEST_CASE_1.to_owned());
        println!("\n\n********************\n\n");
    }

    #[test]
    fn day7_part_2_test_case_1() {
        println!("\n\n********************\n\n");
        part2(TEST_CASE_1.to_owned());
        println!("\n\n********************\n\n");
    }

    #[test]
    fn day7_part_2_test_case_2() {
        println!("\n\n********************\n\n");
        part2(TEST_CASE_2.to_owned());
        println!("\n\n********************\n\n");
    }
}
