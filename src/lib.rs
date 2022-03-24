use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

fn handler(mut cx: FunctionContext) -> JsResult<JsString> {
    let event = cx.argument::<JsObject>(0)?;
    let context = cx.argument::<JsObject>(1)?;
    let callback = cx.argument::<JsFunction>(2)?;

    let null = cx.null();
    let result = cx.string("test2");

    println!("hello world");

    callback.exec(&mut cx, null, &vec![null.upcast(), result.upcast()])?;

    Ok(cx.string("test"))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("handler", handler)?;
    Ok(())
}
