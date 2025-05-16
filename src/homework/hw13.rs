use std::collections::BTreeMap;

#[derive(Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Debug)]
struct Rectangle {
    a: Point,
    b: Point,
}

#[derive(Debug)]
struct Event {
    x: i32,
    y1: i32,
    y2: i32,
    delta: i32,
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}

fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    let mut events = vec![];
    for rect in xs {
        let x1 = rect.a.x.min(rect.b.x);
        let x2 = rect.a.x.max(rect.b.x);
        let y1 = rect.a.y.min(rect.b.y);
        let y2 = rect.a.y.max(rect.b.y);

        events.push(Event { x: x1, y1, y2, delta: 1 });  // початок прямокутника
        events.push(Event { x: x2, y1, y2, delta: -1 }); // кінець прямокутника
    }

    events.sort_by_key(|e| e.x);

    let mut active = BTreeMap::new();
    let mut prev_x = events[0].x;
    let mut area = 0;

    for event in events {
        let width = event.x - prev_x;
        if width > 0 {
            // рахуємо покриту висоту
            let mut height = 0;
            let mut current = 0;
            let mut last_y = 0;
            for (&y, &cnt) in &active {
                if current > 0 {
                    height += y - last_y;
                }
                current += cnt;
                last_y = y;
            }
            area += width * height;
        }

        *active.entry(event.y1).or_insert(0) += event.delta;
        *active.entry(event.y2).or_insert(0) -= event.delta;

        if active[&event.y1] == 0 {
            active.remove(&event.y1);
        }
        if active[&event.y2] == 0 {
            active.remove(&event.y2);
        }

        prev_x = event.x;
    }

    area
}

fn area_occupied_test() {
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 60);
}

fn main() {
    area_occupied_test();
    println!("Test passed!");
}
