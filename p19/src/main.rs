use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    part_one();
    part_two();
}

fn _quality(
    time: isize,
    ore_robot: isize,
    clay_robot: isize,
    obsidian_robot: (isize, isize),
    geode_robot: (isize, isize),
    resources: [isize; 4],
    robots: [isize; 4],
    memo: &mut HashMap<(isize, [isize; 4], [isize; 4]), isize>,
) -> isize {
    if time == 0 {
        return resources[3];
    }
    let mut resources = resources.clone();
    resources[0] = std::cmp::min(
        resources[0],
        *vec![ore_robot, clay_robot, obsidian_robot.0, geode_robot.0]
            .iter()
            .max()
            .unwrap()
            * 2,
    );
    resources[1] = std::cmp::min(resources[1], obsidian_robot.1 * 2);
    resources[2] = std::cmp::min(resources[2], geode_robot.1 * 2);
    let memo_key = &(time, resources, robots);
    if let Some(answer) = memo.get(memo_key) {
        return *answer;
    }

    let mut answers = vec![];

    if resources[0] >= geode_robot.0 && resources[2] >= geode_robot.1 {
        let mut new_resources = resources.clone();
        let mut new_robots = robots.clone();
        new_resources[0] -= geode_robot.0;
        new_resources[2] -= geode_robot.1;
        for i in 0..4 {
            new_resources[i] += robots[i] * 1;
        }
        new_robots[3] += 1;
        answers.push(_quality(
            time - 1,
            ore_robot,
            clay_robot,
            obsidian_robot,
            geode_robot,
            new_resources,
            new_robots,
            memo,
        ));
    }

    if resources[0] >= obsidian_robot.0
        && resources[1] >= obsidian_robot.1
        && robots[2] <= geode_robot.1
    {
        let mut new_resources = resources.clone();
        let mut new_robots = robots.clone();
        new_resources[0] -= obsidian_robot.0;
        new_resources[1] -= obsidian_robot.1;
        for i in 0..4 {
            new_resources[i] += robots[i] * 1;
        }
        new_robots[2] += 1;
        answers.push(_quality(
            time - 1,
            ore_robot,
            clay_robot,
            obsidian_robot,
            geode_robot,
            new_resources,
            new_robots,
            memo,
        ));
    }

    if resources[0] >= clay_robot && robots[1] < obsidian_robot.1 {
        let mut new_resources = resources.clone();
        let mut new_robots = robots.clone();
        new_resources[0] -= clay_robot;
        for i in 0..4 {
            new_resources[i] += robots[i] * 1;
        }
        new_robots[1] += 1;
        answers.push(_quality(
            time - 1,
            ore_robot,
            clay_robot,
            obsidian_robot,
            geode_robot,
            new_resources,
            new_robots,
            memo,
        ));
    }

    if resources[0] >= ore_robot
        && robots[0]
            <= *vec![ore_robot, clay_robot, obsidian_robot.0, geode_robot.0]
                .iter()
                .max()
                .unwrap()
    {
        let mut new_resources = resources.clone();
        let mut new_robots = robots.clone();
        new_resources[0] -= ore_robot;
        for i in 0..4 {
            new_resources[i] += robots[i] * 1;
        }
        new_robots[0] += 1;
        answers.push(_quality(
            time - 1,
            ore_robot,
            clay_robot,
            obsidian_robot,
            geode_robot,
            new_resources,
            new_robots,
            memo,
        ));
    }

    let mut new_resources = resources.clone();
    for i in 0..4 {
        new_resources[i] += robots[i] * 1;
    }
    answers.push(_quality(
        time - 1,
        ore_robot,
        clay_robot,
        obsidian_robot,
        geode_robot,
        new_resources,
        robots.clone(),
        memo,
    ));

    let answer = answers.iter().max().unwrap();
    memo.insert(memo_key.clone(), *answer);
    *answer
}

fn quality(
    time: isize,
    ore_robot: isize,
    clay_robot: isize,
    obsidian_robot: (isize, isize),
    geode_robot: (isize, isize),
) -> isize {
    let resources = [0; 4];
    let robots = [1, 0, 0, 0];
    let mut memo: HashMap<(isize, [isize; 4], [isize; 4]), isize> = HashMap::new();
    _quality(
        time,
        ore_robot,
        clay_robot,
        obsidian_robot,
        geode_robot,
        resources,
        robots,
        &mut memo,
    )
}

fn part_one() {
    let blueprints: Vec<(usize, isize, isize, isize, isize, isize, isize)> =
        include_str!("../input.txt")
            .lines()
            .enumerate()
            .map(|(i, x)| {
                let words: Vec<&str> = x.split_whitespace().collect();
                (
                    i + 1,
                    words[6].parse::<isize>().unwrap(),
                    words[12].parse::<isize>().unwrap(),
                    words[18].parse::<isize>().unwrap(),
                    words[21].parse::<isize>().unwrap(),
                    words[27].parse::<isize>().unwrap(),
                    words[30].parse::<isize>().unwrap(),
                )
            })
            .collect();

    let mut sum = 0;
    for blueprint in blueprints {
        let answer = quality(
            24,
            blueprint.1,
            blueprint.2,
            (blueprint.3, blueprint.4),
            (blueprint.5, blueprint.6),
        );
        dbg!(blueprint.0, answer);
        sum += blueprint.0 * answer as usize;
    }

    println!("Part 1 answer {}", sum)
}

fn part_two() {
    let blueprints: Vec<(usize, isize, isize, isize, isize, isize, isize)> =
        include_str!("../input.txt")
            .lines()
            .enumerate()
            .map(|(i, x)| {
                let words: Vec<&str> = x.split_whitespace().collect();
                (
                    i + 1,
                    words[6].parse::<isize>().unwrap(),
                    words[12].parse::<isize>().unwrap(),
                    words[18].parse::<isize>().unwrap(),
                    words[21].parse::<isize>().unwrap(),
                    words[27].parse::<isize>().unwrap(),
                    words[30].parse::<isize>().unwrap(),
                )
            })
            .take(3)
            .collect();

    let mut product = 1;
    for blueprint in blueprints {
        let answer = quality(
            32,
            blueprint.1,
            blueprint.2,
            (blueprint.3, blueprint.4),
            (blueprint.5, blueprint.6),
        );
        dbg!(blueprint.0, answer);
        product *= answer as usize;
    }

    println!("Part 2 answer {}", product)
}
