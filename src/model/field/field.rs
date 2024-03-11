use napi::{Result, JsUnknown, Env};
use teo::prelude::model::Field as TeoField;

use crate::object::{teo_object_to_js_any, js_any_to_teo_object};

#[napi(js_name = "Field")]
pub struct Field {
    pub(crate) teo_field: &'static mut TeoField,
}

#[napi]
impl Field {

    #[napi]
    pub fn set_data(&mut self, key: String, value: JsUnknown, env: Env) -> Result<()> {
        self.teo_field.data.insert(key, js_any_to_teo_object(value, env)?);
        Ok(())
    }

    #[napi]
    pub fn data(&mut self, key: String, env: Env) -> Result<JsUnknown> {
        Ok(match self.teo_field.data.get(key.as_str()) {
            Some(object) => teo_object_to_js_any(object, &env)?,
            None => env.get_undefined()?.into_unknown(),
        })
    }
}