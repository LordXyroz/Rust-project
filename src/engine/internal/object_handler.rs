use std::sync::{Arc, Mutex, Once};
use std::{mem};

pub trait GameObject {
    fn awake(&self);
    fn start(&self);
    fn update(&self);
    fn late_update(&self);
    fn destroy(&self);
}

#[derive(Clone)]
struct ObjectHandler {
    object_list: Arc<Mutex<Vec<Box<dyn GameObject>>>>,
}

// Will generate a single instance of ObjectHandler on first call
// and only return a pointer to it on subsequent calls
fn instance() -> ObjectHandler {
    // Initialize a mutable pointer as 0 because we are yet to set it
    static mut OBJECTHANDLER: *const ObjectHandler = 0 as *const ObjectHandler;
    // Create an object that will run only once
    static ONCE: Once = Once::new();
    unsafe {
        // Run once, create the ObjectHandler object, and move to the heap so it can outlive this call
        ONCE.call_once(|| {
            let obj_h = ObjectHandler {
                object_list: Arc::new(Mutex::new(vec![])),
            };
            // Moves to heap
            OBJECTHANDLER = mem::transmute(Box::new(obj_h));
        });
        // Give a copy of the data that is safe to use
        (*OBJECTHANDLER).clone()
    }
}

pub fn add_gameobject<T: GameObject + 'static>(t: T) {
    let handler = instance();
    let mut data = handler.object_list.lock().unwrap();
    &data.push(Box::new(t));
}

pub fn awake_objects() {
    let handler = instance();
    let data = handler.object_list.lock().unwrap();
    for obj in data.iter() {
        obj.awake();
    }
}

pub fn start_objects()  {
    let handler = instance();
    let data = handler.object_list.lock().unwrap();
    for obj in data.iter() {
       obj.start();
    }
}

pub fn update_objects() {
    let handler = instance();
    let data = handler.object_list.lock().unwrap();
    for obj in data.iter() {
        obj.update();
    }
}

pub fn late_update_objects() {
    let handler = instance();
    let data = handler.object_list.lock().unwrap();
    for obj in data.iter() {
        obj.late_update();
    }
}