use std::fmt::Display;

trait Priority {
    fn priority(&self) -> u32;
}

#[derive(Clone)]
struct Task {
    name: String,
    priority: u32,
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Task: [{}]: {} prio", self.name, self.priority)
    }
}

impl Priority for Task {
    fn priority(&self) -> u32 {
        self.priority
    }
}

#[allow(dead_code)]
fn summarize_obsolete<T: Display + Priority + Clone>(items: &[T]) -> String {
    let mut sorted: Vec<T> = items.into(); // requires Clone
    sorted.sort_by_key(|t| t.priority());
    sorted.reverse();
    let mut res = String::new();
    sorted.iter().enumerate().for_each(|(i, task)| {
        let line = format!("{}. {task}\n", i + 1);
        res.insert_str(res.len(), line.as_str());
    });
    res
}

// fully functional version + less memory since no Clone (no owned T)
fn summarize<T: Display + Priority>(items: &[T]) -> String {
    let mut sorted: Vec<_> = items.iter().collect(); // not owning! it's Vec<&T>
    sorted.sort_by_key(|item| std::cmp::Reverse(item.priority())); // reversing by Reverse wrapper

    sorted
        .iter()
        .enumerate()
        .map(|(i, task)| format!("{}. {task}", i + 1))
        .collect::<Vec<_>>()
        .join("\n")
}

fn main() {
    let tasks = vec![
        Task {
            name: "review".to_string(),
            priority: 10,
        },
        Task {
            name: "poc".to_string(),
            priority: 100,
        },
        Task {
            name: "tests".to_string(),
            priority: 1,
        },
    ];
    let task_summary = summarize(&tasks);
    println!("{task_summary}");
}

use std::fmt::Display;

trait Priority {
    fn priority(&self) -> u32;
}

#[derive(Clone)]
struct Task {
    name: String,
    priority: u32,
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Task: [{}]: {} prio", self.name, self.priority)
    }
}

impl Priority for Task {
    fn priority(&self) -> u32 {
        self.priority
    }
}

#[allow(dead_code)]
fn summarize<T: Display + Priority + Clone>(items: &[T]) -> String {
    let mut sorted: Vec<T> = items.into(); // requires Clone
    sorted.sort_by_key(|t| t.priority());
    sorted.reverse();
    let mut res = String::new();
    sorted.iter().enumerate().for_each(|(i, task)| {
        let line = format!("{}. {task}\n", i + 1);
        res.insert_str(res.len(), line.as_str());
    });
    res
}

// fully functional version + less memory since no Clone (no owned T)
fn summarize_v2<T: Display + Priority>(items: &[T]) -> String {
    let mut sorted: Vec<_> = items.iter().collect(); // not owning! it's Vec<&T>
    sorted.sort_by_key(|item| std::cmp::Reverse(item.priority())); // reversing by Reverse wrapper

    sorted
        .iter()
        .enumerate()
        .map(|(i, task)| format!("{i}. {task}"))
        .collect::<Vec<_>>()
        .join("\n")
}

fn main() {
    let tasks = vec![
        Task {
            name: "poc".to_string(),
            priority: 100,
        },
        Task {
            name: "tests".to_string(),
            priority: 1,
        },
    ];
    let task_summary = summarize_v2(&tasks);
    println!("{task_summary}");
    // 1. Task: [poc]: 100 prio
    // 2. Task: [review]: 10 prio
    // 3. Task: [tests]: 1 prio
}
