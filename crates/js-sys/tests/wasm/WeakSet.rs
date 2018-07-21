use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;
use js_sys::*;

#[wasm_bindgen]
extern {
    type SomeValue;
    #[wasm_bindgen(method, setter, structural)]
    fn set_some(this: &SomeValue, val: JsValue);
}

fn some_value() -> Object {
    let value = SomeValue::from(JsValue::from(Object::new()));
    value.set_some("value".into());
    Object::from(JsValue::from(value))
}

#[wasm_bindgen_test]
fn new() {
    assert!(JsValue::from(WeakSet::new()).is_object())
}

#[wasm_bindgen_test]
fn has() {
    let set = WeakSet::new();
    let value = some_value();
    assert!(!set.has(&value));
    set.add(&value);
    assert!(set.has(&value));
    assert!(!set.has(&some_value()));
}

#[wasm_bindgen_test]
fn delete() {
    let set = WeakSet::new();
    let value = some_value();
    set.add(&value);
    assert!(set.has(&value));
    assert!(set.delete(&value));
    assert!(!set.has(&value));
    assert!(!set.delete(&value));
}