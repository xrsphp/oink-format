use napi::{JsFunction, Result};
use napi::threadsafe_function::{ErrorStrategy, ThreadSafeCallContext, ThreadsafeFunction};
use teo::prelude::pipeline::item::validator::Validity;
use teo::prelude::{Next, Middleware};
use crate::middleware::SendMiddlewareCallback;
use crate::request::send_next::SendNext;
use teo::prelude::{Value as TeoValue, Arguments, Namespace as TeoNamespace, object::Object as TeoObject, handler::Group as TeoHandlerGroup, Enum as TeoEnum, Member as TeoEnumMember, Model as TeoModel, model::Field as TeoField, model::Property as TeoProperty, model::Relation as TeoRelation, Arguments as TeoArgs, pipeline, model, transaction, request, response::Response as TeoResponse};
use crate::dynamic::{js_ctx_object_from_teo_transaction_ctx, js_model_object_from_teo_model_object};
use crate::handler::group::HandlerGroup;
use crate::model::model::Model;
use crate::model::relation::relation::Relation;
use crate::model::field::field::Field;
use crate::model::property::property::Property;
use crate::object::promise::TeoObjectOrPromise;
use crate::object::teo_object_to_js_any;
use crate::object::arguments::teo_args_to_js_args;
use crate::object::value::teo_value_to_js_any;
use crate::r#enum::member::member::EnumMember;
use crate::r#enum::r#enum::Enum;
use crate::request:: RequestCtx;
use crate::response::response_or_promise::ResponseOrPromise;
use crate::result::IntoTeoResult;
use teo::prelude::error_runtime_ext::ErrorRuntimeExt;


#[napi(js_name = "Namespace")]
pub struct Namespace {
    pub(crate) teo_namespace: &'static mut TeoNamespace,
}

#[napi]
impl Namespace {

    #[napi(js_name = "isMain")]
    pub fn is_main(&self) -> bool {
        self.teo_namespace.is_main()
    }

    #[napi(js_name = "isStd")]
    pub fn is_std(&self) -> bool {
        self.teo_namespace.is_std()
    }

    #[napi]
    pub fn path(&self) -> Vec<&str> {
        self.teo_namespace.path()
    }

    #[napi]
    pub fn namespace(&'static mut self, name: String) -> Option<Namespace> {
        self.teo_namespace.namespace_mut(name.as_str()).map(|n| Namespace { teo_namespace: n })
    }

    #[napi]
    pub fn namespace_or_create(&'static mut self, name: String) -> Namespace {
        Namespace{ teo_namespace: self.teo_namespace.namespace_mut_or_create(name.as_str()) }
    }

    #[napi]
    pub fn namespace_at_path(&'static mut self, path: Vec<&str>) -> Option<Namespace> {
        self.teo_namespace.namespace_mut_at_path(&path).map(|n| Namespace { teo_namespace: n })
    }

    #[napi]
    pub fn namespace_or_create_at_path(&'static mut self, path: Vec<&str>) -> Namespace {
        Namespace{ teo_namespace: self.teo_namespace.namespace_mut_or_create_at_path(&path) }
    }

    #[napi(js_name = "defineModelDecorator", ts_args_type = "name: string, body: (args: {[key: string]: any}, model: Model) => void")]
    pub fn define_model_decorator(&mut self, name: String, callback: JsFunction) -> Result<()> {
        let tsfn: ThreadsafeFunction<(teo::prelude::Arguments, &mut TeoModel), ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<(Arguments, &mut TeoModel)>| {
            let arguments = teo_args_to_js_args(&ctx.value.0, &ctx.env)?;
            let js_model = Model { teo_model: ctx.value.1 };
            Ok(vec![arguments, js_model.into_instance(ctx.env)?.as_object(ctx.env)])
        })?;
        let tsfn_cloned = &*Box::leak(Box::new(tsfn));
        self.teo_namespace.define_model_decorator(name.as_str(), |arguments, model| {
            let static_model: &'static mut TeoModel = unsafe { &mut *(model as * mut TeoModel) };
            let _ = tsfn_cloned.call((arguments, static_model), napi::threadsafe_function::ThreadsafeFunctionCallMode::Blocking);
            Ok(())
        });
        Ok(())
    }

    #[napi(js_name = "defineModelFieldDecorator", ts_args_type = "name: string, body: (args: {[key: string]: any}, field: Field) => void")]
    pub fn define_model_field_decorator(&mut self, name: String, callback: JsFunction) -> Result<()> {
        let tsfn: ThreadsafeFunction<(teo::prelude::Arguments, &mut TeoField), ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<(Arguments, &mut TeoField)>| {
            let arguments = teo_args_to_js_args(&ctx.value.0, &ctx.env)?;
            let js_model = Field { teo_field: ctx.value.1 };
            Ok(vec![arguments, js_model.into_instance(ctx.env)?.as_object(ctx.env)])
        })?;
        let tsfn_cloned = &*Box::leak(Box::new(tsfn));
        self.teo_namespace.define_model_field_decorator(name.as_str(), |arguments, model| {
            let static_model: &'static mut TeoField = unsafe { &mut *(model as * mut TeoField) };
            let _ = tsfn_cloned.call((arguments, static_model), napi::threadsafe_function::ThreadsafeFunctionCallMode::Blocking);
            Ok(())
        });
        Ok(())
    }

    #[napi(js_name = "defineModelRelationDecorator", ts_args_type = "name: string, body: (args: {[key: string]: any}, relation: Relation) => void")]
    pub fn define_model_relation_decorator(&mut self, name: String, callback: JsFunction) -> Result<()> {
        let tsfn: ThreadsafeFunction<(teo::prelude::Arguments, &mut TeoRelation), ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<(Arguments, &mut TeoRelation)>| {
            let arguments = teo_args_to_js_args(&ctx.value.0, &ctx.env)?;
            let js_model = Relation { teo_relation: ctx.value.1 };
            Ok(vec![arguments, js_model.into_instance(ctx.env)?.as_object(ctx.env)])
        })?;
        let tsfn_cloned = &*Box::leak(Box::new(tsfn));
        self.teo_namespace.define_model_relation_decorator(name.as_str(), |arguments, model| {
            let static_model: &'static mut TeoRelation = unsafe { &mut *(model as * mut TeoRelation) };
            let _ = tsfn_cloned.call((arguments, static_model), napi::threadsafe_function::ThreadsafeFunctionCallMode::Blocking);
            Ok(())
        });
        Ok(())
    }

    #[napi(js_name = "defineModelPropertyDecorator", ts_args_type = "name: string, body: (args: {[key: string]: any}, property: Property) => void")]
    pub fn define_model_property_decorator(&mut self, name: String, callback: JsFunction) -> Result<()> {
        let tsfn: ThreadsafeFunction<(teo::prelude::Arguments, &mut TeoProperty), ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<(Arguments, &mut TeoProperty)>| {
            let arguments = teo_args_to_js_args(&ctx.value.0, &ctx.env)?;
            let js_model = Property { teo_property: ctx.value.1 };
            Ok(vec![arguments, js_model.into_instance(ctx.env)?.as_object(ctx.env)])
        })?;
        let tsfn_cloned = &*Box::leak(Box::new(tsfn));
        self.teo_namespace.define_model_property_decorator(name.as_str(), |arguments, model| {
            let static_model: &'static mut TeoProperty = unsafe { &mut *(model as * mut TeoProperty) };
            let _ = tsfn_cloned.call((arguments, static_model), napi::threadsafe_function::ThreadsafeFunctionCallMode::Blocking);
            Ok(())
        });
        Ok(())
    }

    #[napi(js_name = "defineEnumDecorator", ts_args_type = "name: string, body: (args: {[key: string]: any}, e: Enum) => void")]
    pub fn define_enum_decorator(&mut self, name: String, callback: JsFunction) -> Result<()> {
        let tsfn: ThreadsafeFunction<(teo::prelude::Arguments, &mut TeoEnum), ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<(Arguments, &mut TeoEnum)>| {
            let arguments = teo_args_to_js_args(&ctx.value.0, &ctx.env)?;
            let js_model = Enum { teo_enum: ctx.value.1 };
            Ok(vec![arguments, js_model.into_instance(ctx.env)?.as_object(ctx.env)])
        })?;
        let tsfn_cloned = &*Box::leak(Box::new(tsfn));
        self.teo_namespace.define_enum_decorator(name.as_str(), |arguments, model| {
            let static_model: &'static mut TeoEnum = unsafe { &mut *(model as * mut TeoEnum) };
            let _ = tsfn_cloned.call((arguments, static_model), napi::threadsafe_function::ThreadsafeFunctionCallMode::Blocking);
            Ok(())
        });
        Ok(())
    }

    #[napi(js_name = "defineEnumMemberDecorator", ts_args_type = "name: string, body: (args: {[key: string]: any}, member: EnumMember) => void")]
    pub fn define_enum_member_decorator(&mut self, name: String, callback: JsFunction) -> Result<()> {
        let tsfn: ThreadsafeFunction<(teo::prelude::Arguments, &mut TeoEnumMember), ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<(Arguments, &mut TeoEnumMember)>| {
            let arguments = teo_args_to_js_args(&ctx.value.0, &ctx.env)?;
            let js_model = EnumMember { teo_enum_member: ctx.value.1 };
            Ok(vec![arguments, js_model.into_instance(ctx.env)?.as_object(ctx.env)])
        })?;
        let tsfn_cloned = &*Box::leak(Box::new(tsfn));
        self.teo_namespace.define_enum_member_decorator(name.as_str(), |arguments, model| {
            let static_model: &'static mut TeoEnumMember = unsafe { &mut *(model as * mut TeoEnumMember) };
            let _ = tsfn_cloned.call((arguments, static_model), napi::threadsafe_function::ThreadsafeFunctionCallMode::Blocking);
            Ok(())
        });
        Ok(())
    }

    #[napi(js_name = "definePipelineItem", ts_args_type = "name: string, body: (input: any, args: {[key: string]: any}, object: any, teo: any) => any | Promise<any>")]
    pub fn define_pipeline_item(&mut self, name: String, callback: JsFunction) -> Result<()> {
        let tsfn: ThreadsafeFunction<(TeoObject, TeoArgs, model::Object, transaction::Ctx), ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<(TeoObject, TeoArgs, model::Object, transaction::Ctx)>| {
            let js_value = teo_object_to_js_any(&ctx.value.0, &ctx.env)?;
            let js_args = teo_args_to_js_args(&ctx.value.1, &ctx.env)?;
            let js_object = js_model_object_from_teo_model_object(ctx.env, ctx.value.2.clone())?;
            let js_ctx = js_ctx_object_from_teo_transaction_ctx(ctx.env, ctx.value.3.clone(), "")?;
            Ok(vec![js_value, js_args.into_unknown(), js_object.into_unknown(), js_ctx.into_unknown()])
        })?;
        let tsfn_cloned = &*Box::leak(Box::new(tsfn));
        self.teo_namespace.define_pipeline_item(name.as_str(), move |args: TeoArgs, ctx: pipeline::Ctx| async move {
            let object = ctx.value().clone();
            let model_object = ctx.object().clone();
            let transaction_ctx = ctx.transaction_ctx().clone();
            let result: TeoObjectOrPromise = tsfn_cloned.call_async((object, args, model_object, transaction_ctx)).await.into_teo_result()?;
            result.to_teo_object().await.into_teo_result()
        });
        Ok(())
    }

    #[napi(js_name = "defineTransformPipelineItem", ts_args_type = "name: string, callback: (input: any, args: {[key: string]: any}, object: any, teo: any) => any | Promise<any>")]
    pub fn define_transform_pipeline_item(&mut self, name: String, callback: JsFunction) -> Result<()> {
        self.define_pipeline_item(name, callback)
    }

    #[napi(ts_args_type = "name: string, callback: (input: any, args: {[key: string]: any}, object: any, teo: any) => boolean | string | undefined | null | Promise<boolean | string | undefined | null>")]
    pub fn define_validator_pipeline_item(&mut self, name: String, callback: JsFunction) -> Result<()> {
        let tsfn: ThreadsafeFunction<(TeoValue, TeoArgs, model::Object, transaction::Ctx), ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<(TeoValue, TeoArgs, model::Object, transaction::Ctx)>| {
            let js_value = teo_value_to_js_any(&ctx.value.0, &ctx.env)?;
            let js_args = teo_args_to_js_args(&ctx.value.1, &ctx.env)?;
            let js_object = js_model_object_from_teo_model_object(ctx.env, ctx.value.2.clone())?;
            let js_ctx = js_ctx_object_from_teo_transaction_ctx(ctx.env, ctx.value.3.clone(), "")?;
            Ok(vec![js_value, js_args.into_unknown(), js_object.into_unknown(), js_ctx.into_unknown()])
        })?;
        let tsfn_cloned = &*Box::leak(Box::new(tsfn));
        self.teo_namespace.define_validator_pipeline_item(name.as_str(), move |value: TeoValue, args: TeoArgs, ctx: pipeline::Ctx| async move {
            let result: TeoObjectOrPromise = tsfn_cloned.call_async((value, args, ctx.object().clone(), ctx.transaction_ctx())).await.into_teo_result()?;
            let teo_value = result.to_teo_object().await.into_teo_result()?;
            if let Some(teon_value) = teo_value.as_teon() {
                Ok::<Validity, teo::prelude::Error>(match teon_value {
                    TeoValue::String(s) => {
                        Validity::Invalid(s.to_owned())
                    },
                    TeoValue::Bool(b) => if *b {
                        Validity::Valid
                    } else {
                        Validity::Invalid("value is invalid".to_owned())
                    },
                    _ => Validity::Valid
                })
            } else {
                Err::<Validity, teo::prelude::Error>(teo::prelude::Error::new("invalid validator return value"))
            }
        });
        Ok(())
    }

    /// Register a named callback.
    #[napi(ts_args_type = "name: string, callback: (input: any, args: {[key: string]: any}, object: any, teo: any) => void | Promise<void>")]
    pub fn define_callback_pipeline_item(&mut self, name: String, callback: JsFunction) -> Result<()> {
        let tsfn: ThreadsafeFunction<(TeoValue, TeoArgs, model::Object, transaction::Ctx), ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<(TeoValue, TeoArgs, model::Object, transaction::Ctx)>| {
            let js_value = teo_value_to_js_any(&ctx.value.0, &ctx.env)?;
            let js_args = teo_args_to_js_args(&ctx.value.1, &ctx.env)?;
            let js_object = js_model_object_from_teo_model_object(ctx.env, ctx.value.2.clone())?;
            let js_ctx = js_ctx_object_from_teo_transaction_ctx(ctx.env, ctx.value.3.clone(), "")?;
            Ok(vec![js_value, js_args.into_unknown(), js_object.into_unknown(), js_ctx.into_unknown()])
        })?;
        let tsfn_cloned = &*Box::leak(Box::new(tsfn));
        self.teo_namespace.define_callback_pipeline_item(name.as_str(), move |value: TeoValue, args: TeoArgs, ctx: pipeline::Ctx| async move {
            let model_object = ctx.object().clone();
            let transaction_ctx = ctx.transaction_ctx().clone();
            let result: TeoObjectOrPromise = tsfn_cloned.call_async((value, args, model_object, transaction_ctx)).await.into_teo_result()?;
            result.to_teo_object().await.into_teo_result()?;
            Ok(())
        });
        Ok(())
    }

    #[napi(js_name = "defineComparePipelineItem<T>", ts_args_type = "name: string, callback: (oldValue: T, newValue: T, args: {[key: string]: any}, object: any, teo: any) => boolean | string | undefined | null | Promise<boolean | string | undefined | null>")]
    pub fn define_compare_pipeline_item(&mut self, name: String, callback: JsFunction) -> Result<()> {
        let tsfn: ThreadsafeFunction<(TeoValue, TeoValue, TeoArgs, model::Object, transaction::Ctx), ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<(TeoValue, TeoValue, TeoArgs, model::Object, transaction::Ctx)>| {
            let js_value_old = teo_value_to_js_any(&ctx.value.0, &ctx.env)?;
            let js_value_new = teo_value_to_js_any(&ctx.value.1, &ctx.env)?;
            let js_args = teo_args_to_js_args(&ctx.value.2, &ctx.env)?;
            let js_object = js_model_object_from_teo_model_object(ctx.env, ctx.value.3.clone())?;
            let js_ctx = js_ctx_object_from_teo_transaction_ctx(ctx.env, ctx.value.4.clone(), "")?;
            Ok(vec![js_value_old, js_value_new, js_args.into_unknown(), js_object.into_unknown(), js_ctx.into_unknown()])
        })?;
        let tsfn_cloned = &*Box::leak(Box::new(tsfn));
        self.teo_namespace.define_compare_pipeline_item(Box::leak(Box::new(name)).as_str(), move |old: TeoValue, new: TeoValue, args: TeoArgs, object: TeoObject, ctx: pipeline::Ctx| async move {
            let result: TeoObjectOrPromise = tsfn_cloned.call_async((old, new, args, ctx.object().clone(), ctx.transaction_ctx())).await.into_teo_result()?;
            let teo_value = result.to_teo_object().await.into_teo_result()?;
            if let Some(teon_value) = teo_value.as_teon() {
                Ok::<Validity, teo::prelude::Error>(match teon_value {
                    TeoValue::String(s) => {
                        Validity::Invalid(s.to_owned())
                    },
                    TeoValue::Bool(b) => if *b {
                        Validity::Valid
                    } else {
                        Validity::Invalid("value is invalid".to_owned())
                    },
                    _ => Validity::Valid
                })
            } else {
                Err::<Validity, teo::prelude::Error>(teo::prelude::Error::new("invalid validator return value"))
            }
        });
        Ok(())
    }

    #[napi(js_name = "_defineHandler", ts_args_type = "name: string, callback: (ctx: RequestCtx) => Response | Promise<Response>")]
    pub fn define_handler(&mut self, name: String, callback: JsFunction) -> Result<()> {
        let tsfn: ThreadsafeFunction<request::Ctx, ErrorStrategy::CalleeHandled> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<request::Ctx>| {
            let request_ctx = RequestCtx::new(ctx.value.clone());
            let request_ctx_instance = request_ctx.into_instance(ctx.env)?;
            let request_ctx_unknown = request_ctx_instance.as_object(ctx.env).into_unknown();
            Ok(vec![request_ctx_unknown])
        })?;
        let tsfn_cloned = &*Box::leak(Box::new(tsfn));
        self.teo_namespace.define_handler(name.as_str(), move |ctx: request::Ctx| async move {
            let response_unknown: ResponseOrPromise = tsfn_cloned.call_async(Ok(ctx)).await.into_teo_result()?;
            Ok::<TeoResponse, teo::prelude::Error>(response_unknown.to_teo_response().await.into_teo_result()?)
        });
        Ok(())
    }

    #[napi(js_name = "defineHandlerGroup", ts_args_type = "name: string, callback: (group: HandlerGroup) => void")]
    pub fn define_handler_group(&mut self, name: String, callback: JsFunction) -> Result<()> {
        let tsfn: ThreadsafeFunction<HandlerGroup, ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<HandlerGroup>| {
            let handler_group = ctx.value;
            Ok(vec![handler_group])
        })?;
        let tsfn_cloned = &*Box::leak(Box::new(tsfn));
        self.teo_namespace.define_handler_group(name.as_str(), |teo_handler_group: &mut TeoHandlerGroup| {
            let static_model: &'static mut TeoHandlerGroup = unsafe { &mut *(teo_handler_group as * mut TeoHandlerGroup) };
            let handler_group = HandlerGroup { teo_handler_group: static_model };
            let _ = tsfn_cloned.call(handler_group, napi::threadsafe_function::ThreadsafeFunctionCallMode::Blocking);
        });
        Ok(())
    }

    #[napi(js_name = "defineModelHandlerGroup", ts_args_type = "name: string, callback: (group: HandlerGroup) => void")]
    pub fn define_model_handler_group(&mut self, name: String, callback: JsFunction) -> Result<()> {
        let tsfn: ThreadsafeFunction<HandlerGroup, ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<HandlerGroup>| {
            let handler_group = ctx.value;
            Ok(vec![handler_group])
        })?;
        let tsfn_cloned = &*Box::leak(Box::new(tsfn));
        self.teo_namespace.define_model_handler_group(name.as_str(), |teo_handler_group: &mut TeoHandlerGroup| {
            let static_model: &'static mut TeoHandlerGroup = unsafe { &mut *(teo_handler_group as * mut TeoHandlerGroup) };
            let handler_group = HandlerGroup { teo_handler_group: static_model };
            let _ = tsfn_cloned.call(handler_group, napi::threadsafe_function::ThreadsafeFunctionCallMode::Blocking);
        });
        Ok(())
    }

    #[napi(js_name = "defineMiddleware", ts_args_type = "name: string, callback: (args: {[key: string]: any}) => (ctx: RequestCtx, next: (ctx: RequestCtx) => Promise<Response>) => Promise<Response> | Response")]
    pub fn define_middleware(&mut self, name: String, callback: JsFunction) -> Result<()> {
        let threadsafe_callback: ThreadsafeFunction<Arguments, ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<Arguments>| {
            let js_args = teo_args_to_js_args(&ctx.value, &ctx.env)?;
            Ok(vec![js_args])
        })?;
        let threadsafe_callback: &'static ThreadsafeFunction<Arguments, ErrorStrategy::Fatal> = &*Box::leak(Box::new(threadsafe_callback));
        self.teo_namespace.define_middleware(name.as_str(), move |arguments| async move {
            let middleware_function: SendMiddlewareCallback = threadsafe_callback.call_async(arguments).await.into_teo_result()?;
            let wrapped_result = move |ctx: teo::prelude::request::Ctx, next: &'static dyn Next| async move {
                let res_or_promise: ResponseOrPromise = middleware_function.inner.call_async((ctx.clone(), SendNext::new(next))).await.into_teo_result()?;
                let res = res_or_promise.to_teo_response().await.into_teo_result()?;
                return Ok(res);
            };
            let wrapped_box = Box::new(wrapped_result);
            let wrapped_raw = Box::leak(wrapped_box);
            let leak_static_result: &'static dyn Middleware = unsafe { &*(wrapped_raw as * const dyn Middleware) };
            return Ok(leak_static_result);
        });
        Ok(())
    }
}

