use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::{
    prelude::{wasm_bindgen, Closure},
    JsCast, JsValue,
};
use web_sys::console;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // 获取document
    let document = web_sys::window().unwrap().document().unwrap();

    // 创建一个canvas元素
    let o_canvas = document
        .create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    // 设置画板的宽高
    o_canvas.set_width(700);
    o_canvas.set_height(700);
    o_canvas.style().set_property("border", "2px solid")?;
    o_canvas.set_id("wasm-canvas");
    // 添加到body里面
    document.body().unwrap().append_child(&o_canvas)?;
    // 获取context
    let context = o_canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
    let context = Rc::new(context);
    let pressed = Rc::new(Cell::new(false));

    // 鼠标单击下去触发逻辑
    {
        let context = context.clone();
        let pressed = pressed.clone();
        let closure_down = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            context.begin_path();
            context.move_to(event.offset_x() as f64, event.offset_y() as f64);
            pressed.set(true);
            console::log_1(&"我被点击了".into());
        }) as Box<dyn FnMut(_)>);
        o_canvas
            .add_event_listener_with_callback("mousedown", closure_down.as_ref().unchecked_ref())?;
        closure_down.forget();
    }
    // 鼠标移动处理逻辑
    {
        let context = context.clone();
        let pressed = pressed.clone();
        let closure_move = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            if pressed.get() {
                context.line_to(event.offset_x() as f64, event.offset_y() as f64);
                context.stroke();
                context.begin_path();
                context.move_to(event.offset_x() as f64, event.offset_y() as f64);
            }
        }) as Box<dyn FnMut(_)>);
        o_canvas
            .add_event_listener_with_callback("mousemove", closure_move.as_ref().unchecked_ref())?;
        closure_move.forget();
    }
    // 鼠标移出
    {
        let context = context.clone();
        let pressed = pressed.clone();
        let closure_up = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            pressed.set(false);
            console::log_1(&"鼠标抬起".into());
            context.line_to(event.offset_x() as f64, event.offset_y() as f64);
            context.stroke();
        }) as Box<dyn FnMut(_)>);
        o_canvas
            .add_event_listener_with_callback("mouseup", closure_up.as_ref().unchecked_ref())?;
        closure_up.forget();
    }
    Ok(())
}
