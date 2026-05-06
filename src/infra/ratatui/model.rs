/// Model
#[derive(Debug)]
pub struct Model {
    pub model_state: MODELSTATE,
    pub running_state: RUNSTATE,
}

impl Model {
    pub fn new() -> Self {
        Model {
            model_state: MODELSTATE::HOME,
            running_state: RUNSTATE::RUNNING,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum RUNSTATE {
    RUNNING,
    END,
}

#[derive(Debug, PartialEq)]
pub enum MODELSTATE {
    HOME,
    SETTING,
}
