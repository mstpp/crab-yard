// Event System with Observers (Publisher-Subscriber Pattern)

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

struct Observer {
    name: String,
    // Handler function that takes event data
    handler: Box<dyn Fn(&str)>,
}

struct EventBus {
    // Map event types to lists of observers
    // Multiple events can share the same observer!
    subscribers: RefCell<HashMap<String, Vec<Rc<Observer>>>>,
}

impl EventBus {
    fn new() -> Self {
        EventBus {
            subscribers: RefCell::new(HashMap::new()),
        }
    }

    // Subscribe an observer to an event type
    fn subscribe(&self, event_type: &str, observer: Rc<Observer>) {
        self.subscribers
            .borrow_mut()
            .entry(event_type.to_string())
            .or_insert_with(Vec::new)
            .push(observer);
    }

    // Unsubscribe an observer from a specific event
    fn unsubscribe(&self, event_type: &str, observer: &Rc<Observer>) {
        self.subscribers
            .borrow_mut()
            .entry(event_type.to_string())
            .and_modify(|v| {
                if let Some(index) = v.iter().position(|i| i.name == observer.name) {
                    v.remove(index);
                }
            });
    }

    // Publish an event - all subscribers get notified
    fn publish(&self, event_type: &str, data: &str) {
        if let Some(observers) = self.subscribers.borrow().get(event_type) {
            for observer in observers {
                (observer.handler)(data)
            }
        }
    }

    // Get count of subscribers for an event
    fn subscriber_count(&self, event_type: &str) -> usize {
        if let Some(entry) = self.subscribers.borrow().get(event_type) {
            return entry.len();
        } else {
            return 0;
        }
    }

    // Get total reference count for an observer
    fn observer_ref_count(observer: &Rc<Observer>) -> usize {
        Rc::strong_count(observer)
    }
}

impl Observer {
    fn new(name: &str, handler: Box<dyn Fn(&str)>) -> Rc<Self> {
        Rc::new(Observer {
            name: name.to_string(),
            handler: handler,
        })
    }
}

// 3. Test with this scenario:
fn main() {
    let bus = EventBus::new();

    // Create observers with different behaviors
    let logger = Observer::new(
        "Logger",
        Box::new(|data| {
            println!("[LOG] 🪵 {}", data);
        }),
    );

    let metrics = Observer::new(
        "Metrics",
        Box::new(|data| {
            println!("[METRICS] Ⓜ️ Recording: {}", data);
        }),
    );

    let alerter = Observer::new(
        "Alerter",
        Box::new(|data| {
            println!("[ALERT] ⚠️ {}", data);
        }),
    );

    // Subscribe observers to events
    // Logger listens to everything
    bus.subscribe("user_login", Rc::clone(&logger));
    bus.subscribe("user_logout", Rc::clone(&logger));
    bus.subscribe("error", Rc::clone(&logger));

    // Metrics only tracks user events
    bus.subscribe("user_login", Rc::clone(&metrics));
    bus.subscribe("user_logout", Rc::clone(&metrics));

    // Alerter only watches errors
    bus.subscribe("error", Rc::clone(&alerter));

    println!("=== Publishing user_login ===");
    bus.publish("user_login", "User Alice logged in");
    let user_login_subs = bus.subscriber_count("user_login");
    println!("Subscribers count: {}", user_login_subs);
    // Should print: [LOG] and [METRICS]

    println!("\n=== Publishing error ===");
    bus.publish("error", "Database connection failed");
    // Should print: [LOG] and [ALERT]

    let logger_subs_count = EventBus::observer_ref_count(&logger);
    println!(
        "\nLogger is subscribed to ({} - 1) events",
        logger_subs_count
    );
    println!("Logger has {} total references", logger_subs_count);

    // Unsubscribe and see changes
    bus.unsubscribe("error", &logger);
    println!("\nAfter unsubscribing logger from errors:");
    bus.publish("error", "Another error");
    // Should only print [ALERT], not [LOG]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_bus_new() {
        let e = EventBus::new();
        assert_eq!(e.subscribers.borrow().len(), 0);
    }
}
