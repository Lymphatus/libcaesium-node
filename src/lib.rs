use neon::prelude::*;

fn compress(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let input = cx.argument::<JsString>(0)?;
    let output = cx.argument::<JsString>(1)?;
    let mut compression_parameters = caesium::initialize_parameters();
    let result: Handle<JsBoolean> = match caesium::compress(input.value(&mut cx), output.value(&mut cx), &compression_parameters) {
        Ok(_) => cx.boolean(true),
        Err(_) => cx.boolean(false),
    };

    Ok(result)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("compress", compress)?;
    Ok(())
}
