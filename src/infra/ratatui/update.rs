use super::model::{Model, RUNSTATE, MODELSTATE};

#[derive(Debug, PartialEq)]
pub enum MESSAGE {
    HOME,
    SETTING,
    QUIT,
    DISTRIBUTE_MSG(String),
    NONE,
}

pub fn update(model: &mut Model, message: MESSAGE) -> MESSAGE {
    match message {
        MESSAGE::HOME => {
            model.model_state = MODELSTATE::HOME;
            MESSAGE::NONE
        }
        MESSAGE::SETTING => {
            model.model_state = MODELSTATE::SETTING;
            MESSAGE::NONE
        }
        MESSAGE::QUIT => {
            model.running_state = RUNSTATE::END;
            MESSAGE::NONE
        }
        MESSAGE::DISTRIBUTE_MSG(event) => {
            match model.model_state {
                MODELSTATE::HOME => update_home(model, &event),
                MODELSTATE::SETTING => update_setting(model, &event),
                _ => MESSAGE::NONE,
            }
        }
        _ => MESSAGE::NONE,
    }
}


pub fn update_home(model: &mut Model, event: &str) -> MESSAGE {
    if model.model_state != MODELSTATE::HOME {
        return MESSAGE::NONE;
    }
    match event {
        "Q" => {
            model.running_state = RUNSTATE::END;
            MESSAGE::NONE
        }
        "S" => {
            model.model_state = MODELSTATE::SETTING;
            MESSAGE::NONE
        }
        _ => MESSAGE::NONE,
    }
}


pub fn update_setting(model: &mut Model, event: &str) -> MESSAGE {
    if model.model_state != MODELSTATE::SETTING {
        return MESSAGE::NONE;
    }
    match event {
        "B" => {
            model.model_state = MODELSTATE::HOME;
            MESSAGE::NONE
        }
        _ => MESSAGE::NONE,
    }
}
