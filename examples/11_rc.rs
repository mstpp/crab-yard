use std::rc::Rc;

#[derive(Debug)]
struct Node {
    message: String,
    next: Option<Rc<Node>>,
}
fn main() -> anyhow::Result<()> {
    // since Rc is immutable, we can't add
    let tail = Rc::new(Node {
        message: "last node".to_string(),
        next: None,
    });
    let head = Rc::new(Node {
        message: "first before last".to_string(),
        next: Some(Rc::clone(&tail)),
    });
    let new_head = Rc::new(Node {
        message: "2nd before last".to_string(),
        next: Some(Rc::clone(&head)),
    });

    println!("{:?}", new_head);
    println!("Tail ref count: {}", Rc::strong_count(&tail));
    println!("Head ref count: {}", Rc::strong_count(&head));
    println!("New head ref count: {}", Rc::strong_count(&new_head));

    Ok(())
}
