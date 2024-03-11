pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // RefCell allows us to follow the borrowing rules at runtime and not have them checked at
    // compile time
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                // we wrap the vector in a refcell so we can mutate sent_messages, even though the
                // trait states that the method can only have a reference to self and not a
                // mutable reference
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // Here we indicate we want to borrow the value inside the refcell as mutable
            // Allowing a struct to alter it's internal state, regardless of if it has an immutable
            // reference, is called the "Interior Mutability Pattern"
            self.sent_messages.borrow_mut().push(String::from(message));

            // The code below will compile - as the this is not being checked at compile time by
            // the borrow checker
            // It will, however, Panic at runtime as we have more than one RefMut<T> (mutable reference) to the
            // the RefCell active in the same scope. The borrow checker fails, so panics stating
            // that the value was already borrowed
            //
            // To get around this, we can use Rc and RefCell together!
            // See main.rs for an example

            //let mut one_borrow = self.sent_messages.borrow_mut();
            //let mut two_borrow = self.sent_messages.borrow_mut();

            //one_borrow.push(String::from(message));
            //two_borrow.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        // immutable reference used, but mock_messenger is free to mutate it's internal state
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);
    }
}
