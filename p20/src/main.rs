use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::rc::Rc;

fn main() {
    part_one();
    part_two();
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Link<T>,
    prev: Link<T>,
}

fn part_one() {
    let numbers = include_str!("../input.txt")
        .lines()
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<VecDeque<isize>>();
    let mut nodes = vec![];
    let mut prev = None;
    for number in &numbers {
        let node = Rc::new(RefCell::new(Node {
            val: number,
            next: None,
            prev: prev.clone(),
        }));
        nodes.push(node.clone());
        match prev.take() {
            Some(prev_node) => {
                prev_node.borrow_mut().next = Some(node.clone());
            }
            None => {}
        }
        prev = Some(node.clone());
    }
    nodes[0].borrow_mut().prev = Some(nodes[nodes.len() - 1].clone());
    nodes[nodes.len() - 1].borrow_mut().next = Some(nodes[0].clone());

    for node in &nodes {
        let node_val = (**node).borrow().val;
        let forwards = *node_val > 0;
        for _ in 0..(node_val.abs() % (nodes.len() as isize - 1)) {
            if forwards {
                let next = RefCell::borrow(node).next.clone().unwrap();
                let next_next = RefCell::borrow(&next).next.clone().unwrap();
                let prev = RefCell::borrow(&node).prev.clone().unwrap();
                // dbg!(
                //     RefCell::borrow(&next).val,
                //     RefCell::borrow(&next_next).val,
                //     RefCell::borrow(&prev).val,
                // );
                node.borrow_mut().next = Some(next_next.clone());
                node.borrow_mut().prev = Some(next.clone());

                next_next.borrow_mut().prev = Some(node.clone());
                prev.borrow_mut().next = Some(next.clone());

                next.borrow_mut().prev = Some(prev.clone());
                next.borrow_mut().next = Some(node.clone());
            } else {
                let next = RefCell::borrow(node).next.clone().unwrap();
                let prev = RefCell::borrow(&node).prev.clone().unwrap();
                let prev_prev = RefCell::borrow(&prev).prev.clone().unwrap();

                node.borrow_mut().next = Some(prev.clone());
                node.borrow_mut().prev = Some(prev_prev.clone());

                prev_prev.borrow_mut().next = Some(node.clone());
                next.borrow_mut().prev = Some(prev.clone());

                prev.borrow_mut().prev = Some(node.clone());
                prev.borrow_mut().next = Some(next.clone());
            }
        }
    }

    let mut answer = 0;
    let zero_index = numbers.iter().position(|&x| x == 0).unwrap();
    let mut curr = nodes[zero_index].clone();
    for _ in 0..1000 {
        let temp = RefCell::borrow(&curr.clone()).next.clone().unwrap();
        curr = temp;
    }
    dbg!(RefCell::borrow(&curr.clone()).val);
    answer += RefCell::borrow(&curr.clone()).val;
    for _ in 0..1000 {
        let temp = RefCell::borrow(&curr.clone()).next.clone().unwrap();
        curr = temp;
    }
    dbg!(RefCell::borrow(&curr.clone()).val);
    answer += RefCell::borrow(&curr.clone()).val;
    for _ in 0..1000 {
        let temp = RefCell::borrow(&curr.clone()).next.clone().unwrap();
        curr = temp;
    }
    dbg!(RefCell::borrow(&curr.clone()).val);
    answer += RefCell::borrow(&curr.clone()).val;

    println!("Part 1 answer {}", answer,);
    // let mut curr = nodes[0].clone();
    // for _ in 0..nodes.len() {
    //     let temp = RefCell::borrow(&curr.clone()).next.clone().unwrap();
    //     curr = temp;
    //     dbg!(RefCell::borrow(&curr.clone()).val);
    // }
    // let temp = numbers.iter().map(|x| *x as i64).collect::<Vec<i64>>();
    // println!("Part 1 actual answer {}", solve(&temp[..], 1, 1));
}

fn part_two() {
    let numbers = include_str!("../input.txt")
        .lines()
        .map(|x| x.parse::<isize>().unwrap() * 811589153)
        .collect::<VecDeque<isize>>();
    let mut nodes = vec![];
    let mut prev = None;
    for number in &numbers {
        let node = Rc::new(RefCell::new(Node {
            val: number,
            next: None,
            prev: prev.clone(),
        }));
        nodes.push(node.clone());
        match prev.take() {
            Some(prev_node) => {
                prev_node.borrow_mut().next = Some(node.clone());
            }
            None => {}
        }
        prev = Some(node.clone());
    }
    nodes[0].borrow_mut().prev = Some(nodes[nodes.len() - 1].clone());
    nodes[nodes.len() - 1].borrow_mut().next = Some(nodes[0].clone());

    for _ in 0..10 {
        for node in &nodes {
            let node_val = (**node).borrow().val;
            let forwards = *node_val > 0;
            for _ in 0..(node_val.abs() % (nodes.len() as isize - 1)) {
                if forwards {
                    let next = RefCell::borrow(node).next.clone().unwrap();
                    let next_next = RefCell::borrow(&next).next.clone().unwrap();
                    let prev = RefCell::borrow(&node).prev.clone().unwrap();
                    // dbg!(
                    //     RefCell::borrow(&next).val,
                    //     RefCell::borrow(&next_next).val,
                    //     RefCell::borrow(&prev).val,
                    // );
                    node.borrow_mut().next = Some(next_next.clone());
                    node.borrow_mut().prev = Some(next.clone());

                    next_next.borrow_mut().prev = Some(node.clone());
                    prev.borrow_mut().next = Some(next.clone());

                    next.borrow_mut().prev = Some(prev.clone());
                    next.borrow_mut().next = Some(node.clone());
                } else {
                    let next = RefCell::borrow(node).next.clone().unwrap();
                    let prev = RefCell::borrow(&node).prev.clone().unwrap();
                    let prev_prev = RefCell::borrow(&prev).prev.clone().unwrap();

                    node.borrow_mut().next = Some(prev.clone());
                    node.borrow_mut().prev = Some(prev_prev.clone());

                    prev_prev.borrow_mut().next = Some(node.clone());
                    next.borrow_mut().prev = Some(prev.clone());

                    prev.borrow_mut().prev = Some(node.clone());
                    prev.borrow_mut().next = Some(next.clone());
                }
            }
        }
    }

    let mut answer = 0;
    let zero_index = numbers.iter().position(|&x| x == 0).unwrap();
    let mut curr = nodes[zero_index].clone();
    for _ in 0..1000 {
        let temp = RefCell::borrow(&curr.clone()).next.clone().unwrap();
        curr = temp;
    }
    dbg!(RefCell::borrow(&curr.clone()).val);
    answer += RefCell::borrow(&curr.clone()).val;
    for _ in 0..1000 {
        let temp = RefCell::borrow(&curr.clone()).next.clone().unwrap();
        curr = temp;
    }
    dbg!(RefCell::borrow(&curr.clone()).val);
    answer += RefCell::borrow(&curr.clone()).val;
    for _ in 0..1000 {
        let temp = RefCell::borrow(&curr.clone()).next.clone().unwrap();
        curr = temp;
    }
    dbg!(RefCell::borrow(&curr.clone()).val);
    answer += RefCell::borrow(&curr.clone()).val;

    println!("Part 2 answer {}", answer,);
}
