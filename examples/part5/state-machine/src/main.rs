//! State Machine Example
//!
//! Demonstrates state machine patterns using enums and typestates.
//!
//! # State Machine Types
//! ```text
//!     ┌─────────────────────────────────────────────────────────┐
//!     │              State Machine Approaches                   │
//!     ├─────────────────────────────────────────────────────────┤
//!     │                                                         │
//!     │  Enum-based:                                            │
//!     │  ├── Runtime state transitions                          │
//!     │  ├── Single type holds all states                       │
//!     │  └── Invalid transitions return errors                  │
//!     │                                                         │
//!     │  Typestate (compile-time):                              │
//!     │  ├── State encoded in type system                       │
//!     │  ├── Different types for each state                     │
//!     │  └── Invalid transitions are compile errors             │
//!     │                                                         │
//!     └─────────────────────────────────────────────────────────┘
//! ```

use std::collections::HashMap;

fn main() {
    println!("=== State Machine Patterns ===\n");

    println!("--- Enum-Based State Machine ---");
    enum_state_machine();

    println!("\n--- Typestate Pattern ---");
    typestate_pattern();

    println!("\n--- Event-Driven State Machine ---");
    event_driven();

    println!("\n--- Traffic Light Example ---");
    traffic_light();

    println!("\n--- Document Workflow ---");
    document_workflow();
}

// ============================================
// Enum-Based State Machine
// ============================================

/// Order states
#[derive(Debug, Clone, PartialEq)]
enum OrderState {
    Pending,
    Confirmed { confirmed_at: String },
    Shipped { tracking: String },
    Delivered { delivered_at: String },
    Cancelled { reason: String },
}

#[derive(Debug)]
struct Order {
    id: u64,
    items: Vec<String>,
    state: OrderState,
}

#[derive(Debug)]
enum OrderError {
    InvalidTransition { from: String, to: String },
}

impl Order {
    fn new(id: u64, items: Vec<String>) -> Self {
        Order {
            id,
            items,
            state: OrderState::Pending,
        }
    }

    fn confirm(&mut self) -> Result<(), OrderError> {
        match &self.state {
            OrderState::Pending => {
                self.state = OrderState::Confirmed {
                    confirmed_at: "2024-01-15".to_string(),
                };
                Ok(())
            }
            _ => Err(OrderError::InvalidTransition {
                from: format!("{:?}", self.state),
                to: "Confirmed".to_string(),
            }),
        }
    }

    fn ship(&mut self, tracking: String) -> Result<(), OrderError> {
        match &self.state {
            OrderState::Confirmed { .. } => {
                self.state = OrderState::Shipped { tracking };
                Ok(())
            }
            _ => Err(OrderError::InvalidTransition {
                from: format!("{:?}", self.state),
                to: "Shipped".to_string(),
            }),
        }
    }

    fn deliver(&mut self) -> Result<(), OrderError> {
        match &self.state {
            OrderState::Shipped { .. } => {
                self.state = OrderState::Delivered {
                    delivered_at: "2024-01-20".to_string(),
                };
                Ok(())
            }
            _ => Err(OrderError::InvalidTransition {
                from: format!("{:?}", self.state),
                to: "Delivered".to_string(),
            }),
        }
    }

    fn cancel(&mut self, reason: String) -> Result<(), OrderError> {
        match &self.state {
            OrderState::Pending | OrderState::Confirmed { .. } => {
                self.state = OrderState::Cancelled { reason };
                Ok(())
            }
            _ => Err(OrderError::InvalidTransition {
                from: format!("{:?}", self.state),
                to: "Cancelled".to_string(),
            }),
        }
    }
}

fn enum_state_machine() {
    let mut order = Order::new(1, vec!["Book".into(), "Pen".into()]);
    println!("  Created: {:?}", order.state);

    order.confirm().unwrap();
    println!("  Confirmed: {:?}", order.state);

    order.ship("TRACK123".to_string()).unwrap();
    println!("  Shipped: {:?}", order.state);

    order.deliver().unwrap();
    println!("  Delivered: {:?}", order.state);

    // Invalid transition
    if let Err(e) = order.cancel("Changed mind".to_string()) {
        println!("  Cannot cancel: {:?}", e);
    }
}

// ============================================
// Typestate Pattern (Compile-Time)
// ============================================

mod typestate {
    use std::marker::PhantomData;

    // State marker types
    pub struct Draft;
    pub struct PendingReview;
    pub struct Published;

    pub struct BlogPost<State> {
        content: String,
        _state: PhantomData<State>,
    }

    // Only Draft posts can have content edited
    impl BlogPost<Draft> {
        pub fn new(content: String) -> Self {
            BlogPost {
                content,
                _state: PhantomData,
            }
        }

        pub fn edit(&mut self, new_content: String) {
            self.content = new_content;
        }

        pub fn submit_for_review(self) -> BlogPost<PendingReview> {
            BlogPost {
                content: self.content,
                _state: PhantomData,
            }
        }
    }

    // PendingReview posts can be approved or rejected
    impl BlogPost<PendingReview> {
        pub fn approve(self) -> BlogPost<Published> {
            BlogPost {
                content: self.content,
                _state: PhantomData,
            }
        }

        pub fn reject(self) -> BlogPost<Draft> {
            BlogPost {
                content: self.content,
                _state: PhantomData,
            }
        }
    }

    // Published posts can only be read
    impl BlogPost<Published> {
        pub fn content(&self) -> &str {
            &self.content
        }
    }

    // All states can get content length
    impl<State> BlogPost<State> {
        pub fn len(&self) -> usize {
            self.content.len()
        }
    }
}

fn typestate_pattern() {
    use typestate::*;

    // Create draft
    let mut post = BlogPost::new("Initial content".to_string());
    println!("  Draft created, length: {}", post.len());

    // Edit while in draft
    post.edit("Updated content for review".to_string());
    println!("  Edited draft");

    // Submit for review (consumes draft, returns pending)
    let pending = post.submit_for_review();
    println!("  Submitted for review, length: {}", pending.len());

    // Cannot edit pending post - this would not compile:
    // pending.edit("hack");

    // Approve (consumes pending, returns published)
    let published = pending.approve();
    println!("  Published! Content: {}", published.content());

    // Cannot edit published post - this would not compile:
    // published.edit("hack");

    println!("  (Invalid transitions are compile errors!)");
}

// ============================================
// Event-Driven State Machine
// ============================================

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum State {
    Idle,
    Running,
    Paused,
    Stopped,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Event {
    Start,
    Pause,
    Resume,
    Stop,
    Reset,
}

struct StateMachine {
    state: State,
    transitions: HashMap<(State, Event), State>,
}

impl StateMachine {
    fn new() -> Self {
        let mut transitions = HashMap::new();

        // Define valid transitions
        transitions.insert((State::Idle, Event::Start), State::Running);
        transitions.insert((State::Running, Event::Pause), State::Paused);
        transitions.insert((State::Running, Event::Stop), State::Stopped);
        transitions.insert((State::Paused, Event::Resume), State::Running);
        transitions.insert((State::Paused, Event::Stop), State::Stopped);
        transitions.insert((State::Stopped, Event::Reset), State::Idle);

        StateMachine {
            state: State::Idle,
            transitions,
        }
    }

    fn process(&mut self, event: Event) -> Result<&State, String> {
        let key = (self.state.clone(), event.clone());
        match self.transitions.get(&key) {
            Some(new_state) => {
                self.state = new_state.clone();
                Ok(&self.state)
            }
            None => Err(format!(
                "Invalid transition: {:?} + {:?}",
                self.state, event
            )),
        }
    }

    fn state(&self) -> &State {
        &self.state
    }
}

fn event_driven() {
    let mut sm = StateMachine::new();
    println!("  Initial state: {:?}", sm.state());

    let events = vec![
        Event::Start,
        Event::Pause,
        Event::Resume,
        Event::Stop,
        Event::Reset,
    ];

    for event in events {
        match sm.process(event.clone()) {
            Ok(state) => println!("  {:?} -> {:?}", event, state),
            Err(e) => println!("  Error: {}", e),
        }
    }

    // Try invalid transition
    if let Err(e) = sm.process(Event::Pause) {
        println!("  Invalid: {}", e);
    }
}

// ============================================
// Traffic Light Example
// ============================================

#[derive(Debug, Clone, Copy)]
enum TrafficLight {
    Red { remaining: u32 },
    Yellow { remaining: u32 },
    Green { remaining: u32 },
}

impl TrafficLight {
    fn new() -> Self {
        TrafficLight::Red { remaining: 30 }
    }

    fn tick(&mut self) {
        *self = match *self {
            TrafficLight::Red { remaining } if remaining > 0 => TrafficLight::Red {
                remaining: remaining - 1,
            },
            TrafficLight::Red { .. } => TrafficLight::Green { remaining: 25 },

            TrafficLight::Green { remaining } if remaining > 0 => TrafficLight::Green {
                remaining: remaining - 1,
            },
            TrafficLight::Green { .. } => TrafficLight::Yellow { remaining: 5 },

            TrafficLight::Yellow { remaining } if remaining > 0 => TrafficLight::Yellow {
                remaining: remaining - 1,
            },
            TrafficLight::Yellow { .. } => TrafficLight::Red { remaining: 30 },
        };
    }

    fn can_proceed(&self) -> bool {
        matches!(self, TrafficLight::Green { .. })
    }

    fn color(&self) -> &'static str {
        match self {
            TrafficLight::Red { .. } => "🔴",
            TrafficLight::Yellow { .. } => "🟡",
            TrafficLight::Green { .. } => "🟢",
        }
    }
}

fn traffic_light() {
    let mut light = TrafficLight::new();
    println!("  Traffic light simulation:");

    // Simulate a few transitions
    for i in 0..5 {
        println!(
            "    Step {}: {} Can proceed: {}",
            i,
            light.color(),
            light.can_proceed()
        );

        // Fast-forward through remaining time
        while matches!(
            light,
            TrafficLight::Red { remaining } |
            TrafficLight::Green { remaining } |
            TrafficLight::Yellow { remaining } if remaining > 0
        ) {
            light.tick();
        }
        light.tick(); // Transition to next state
    }
}

// ============================================
// Document Workflow
// ============================================

mod workflow {
    #[derive(Debug, Clone)]
    pub enum DocumentState {
        Draft {
            author: String,
            content: String,
        },
        UnderReview {
            author: String,
            content: String,
            reviewer: String,
        },
        Approved {
            author: String,
            content: String,
            approved_by: String,
        },
        Rejected {
            author: String,
            content: String,
            reason: String,
        },
        Published {
            author: String,
            content: String,
            published_at: String,
        },
    }

    impl DocumentState {
        pub fn new_draft(author: String, content: String) -> Self {
            DocumentState::Draft { author, content }
        }

        pub fn submit_for_review(self, reviewer: String) -> Result<Self, &'static str> {
            match self {
                DocumentState::Draft { author, content } => Ok(DocumentState::UnderReview {
                    author,
                    content,
                    reviewer,
                }),
                _ => Err("Can only submit drafts for review"),
            }
        }

        pub fn approve(self) -> Result<Self, &'static str> {
            match self {
                DocumentState::UnderReview {
                    author,
                    content,
                    reviewer,
                } => Ok(DocumentState::Approved {
                    author,
                    content,
                    approved_by: reviewer,
                }),
                _ => Err("Can only approve documents under review"),
            }
        }

        pub fn reject(self, reason: String) -> Result<Self, &'static str> {
            match self {
                DocumentState::UnderReview {
                    author, content, ..
                } => Ok(DocumentState::Rejected {
                    author,
                    content,
                    reason,
                }),
                _ => Err("Can only reject documents under review"),
            }
        }

        pub fn publish(self) -> Result<Self, &'static str> {
            match self {
                DocumentState::Approved {
                    author, content, ..
                } => Ok(DocumentState::Published {
                    author,
                    content,
                    published_at: "2024-01-15".to_string(),
                }),
                _ => Err("Can only publish approved documents"),
            }
        }

        pub fn revise(self) -> Result<Self, &'static str> {
            match self {
                DocumentState::Rejected {
                    author, content, ..
                } => Ok(DocumentState::Draft { author, content }),
                _ => Err("Can only revise rejected documents"),
            }
        }
    }
}

fn document_workflow() {
    use workflow::DocumentState;

    // Happy path
    let doc = DocumentState::new_draft("Alice".into(), "Article content".into());
    println!("  Created: Draft");

    let doc = doc.submit_for_review("Bob".into()).unwrap();
    println!("  Submitted for review");

    let doc = doc.approve().unwrap();
    println!("  Approved");

    let doc = doc.publish().unwrap();
    println!("  Published: {:?}", doc);

    // Rejection path
    let doc2 = DocumentState::new_draft("Carol".into(), "Another article".into());
    let doc2 = doc2.submit_for_review("Dave".into()).unwrap();
    let doc2 = doc2.reject("Needs more details".into()).unwrap();
    println!("\n  Rejected: {:?}", doc2);

    let doc2 = doc2.revise().unwrap();
    println!("  Back to draft for revision");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_state_transitions() {
        let mut order = Order::new(1, vec!["item".into()]);
        assert!(matches!(order.state, OrderState::Pending));

        order.confirm().unwrap();
        assert!(matches!(order.state, OrderState::Confirmed { .. }));

        order.ship("TRACK".into()).unwrap();
        assert!(matches!(order.state, OrderState::Shipped { .. }));
    }

    #[test]
    fn test_invalid_transition() {
        let mut order = Order::new(1, vec!["item".into()]);
        let result = order.ship("TRACK".into());
        assert!(result.is_err());
    }

    #[test]
    fn test_state_machine() {
        let mut sm = StateMachine::new();
        assert_eq!(sm.state(), &State::Idle);

        sm.process(Event::Start).unwrap();
        assert_eq!(sm.state(), &State::Running);
    }

    #[test]
    fn test_traffic_light() {
        let mut light = TrafficLight::new();
        assert!(matches!(light, TrafficLight::Red { .. }));

        // Tick through red (30 ticks to reach 0, then 1 more to transition)
        for _ in 0..31 {
            light.tick();
        }
        assert!(matches!(light, TrafficLight::Green { .. }));
    }
}
