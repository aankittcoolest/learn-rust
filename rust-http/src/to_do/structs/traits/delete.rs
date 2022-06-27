use serde_json::Map;
use serde_json::Value;
use crate::state::write_to_file;

pub trait Delete {
    fn delete(&self, title: &str, state: &mut Map<String,Value>) {
        state.remove(title);
        write_to_file("./state.json",state);
        println!("{} is being deleted", title)
    }
}