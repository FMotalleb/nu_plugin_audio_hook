use nu_plugin::{self, EvaluatedCall, LabeledError};
use nu_protocol::{Category, PluginSignature, SyntaxShape, Value};

pub struct Plugin;

impl nu_plugin::Plugin for Plugin {
    fn signature(&self) -> Vec<PluginSignature> {
        vec![PluginSignature::build("sound make")
            .required(
                "Frequency",
                SyntaxShape::Int,
                "Frequency of the sound to make",
            )
            .required("duration", SyntaxShape::Int, "duration of the sound")
            .category(Category::Experimental)]
    }

    fn run(
        &mut self,
        _name: &str,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        let freq: Value = call.req(0);
        Ok(Value::test_bool(true))
    }
}

fn main() {
    nu_plugin::serve_plugin(&mut Plugin {}, nu_plugin::MsgPackSerializer {})
}
