pub struct Unlocked;
pub struct Locked(i32);

pub struct MyBox<State = Unlocked> {
    data: String,
    state: State,
}

impl MyBox<Unlocked> {
    pub fn new(data: String) -> Self {
        Self {
            data,
            state: Unlocked,
        }
    }

    pub fn peek(&self) -> &str {
        &self.data
    }

    pub fn lock(self, pin: i32) -> MyBox<Locked> {
        MyBox {
            data: self.data,
            state: Locked(pin),
        }
    }

    pub fn duplicate(&self) -> MyBox<Unlocked> {
        MyBox {
            data: self.data.clone(),
            state: Unlocked,
        }
    }
}

impl MyBox<Locked> {
    pub fn unlock(self, pin: i32) -> Result<MyBox<Unlocked>, ()> {
        if self.state.0 == pin {
            Ok(MyBox {
                data: self.data,
                state: Unlocked,
            })
        } else {
            Err(())
        }
    }

    pub fn duplicate(&self) -> MyBox<Locked> {
        MyBox {
            data: self.data.clone(),
            state: Locked(self.state.0),
        }
    }
}
