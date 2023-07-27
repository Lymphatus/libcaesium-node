use caesium::CSParameters;
use neon::prelude::*;

fn compress(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let input = cx.argument::<JsString>(0)?;
    let output = cx.argument::<JsString>(1)?;
    let parameters_object = cx.argument::<JsObject>(2)?;
    let compression_parameters = parse_parameters(&mut cx, parameters_object);
    let result: Handle<JsBoolean> = match caesium::compress(input.value(&mut cx), output.value(&mut cx), &compression_parameters) {
        Ok(_) => cx.boolean(true),
        Err(_) => cx.boolean(false),
    };

    Ok(result)
}

fn compress_to_size(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let input = cx.argument::<JsString>(0)?;
    let output = cx.argument::<JsString>(1)?;
    let parameters_object = cx.argument::<JsObject>(2)?;
    let mut compression_parameters = parse_parameters(&mut cx, parameters_object);
    let max_output_size = cx.argument::<JsNumber>(3)?;
    let result: Handle<JsBoolean> = match caesium::compress_to_size(input.value(&mut cx), output.value(&mut cx), &mut compression_parameters, max_output_size.value(&mut cx) as usize) {
        Ok(_) => cx.boolean(true),
        Err(_) => cx.boolean(false),
    };

    Ok(result)
}

fn parse_parameters(cx: &mut FunctionContext, parameters_object: Handle<JsObject>) -> CSParameters
{
    let mut compression_parameters = caesium::initialize_parameters();

    let jpeg_parameters: Handle<JsObject> = parameters_object.get(cx, "jpeg").unwrap_or(cx.empty_object());
    let png_parameters: Handle<JsObject> = parameters_object.get(cx, "png").unwrap_or(cx.empty_object());
    let gif_parameters: Handle<JsObject> = parameters_object.get(cx, "gif").unwrap_or(cx.empty_object());
    let webp_parameters: Handle<JsObject> = parameters_object.get(cx, "webp").unwrap_or(cx.empty_object());

    let keep_metadata: Handle<JsBoolean> = parameters_object.get(cx, "keep_metadata").unwrap_or(cx.boolean(false));
    let optimize: Handle<JsBoolean> = parameters_object.get(cx, "optimize").unwrap_or(cx.boolean(false));
    let width: Handle<JsNumber> = parameters_object.get(cx, "width").unwrap_or(cx.number(0));
    let height: Handle<JsNumber> = parameters_object.get(cx, "height").unwrap_or(cx.number(0));

    let jpeg_quality:Handle<JsNumber> = jpeg_parameters.get(cx, "quality").unwrap_or(cx.number(80));
    let png_quality:Handle<JsNumber> = png_parameters.get(cx, "quality").unwrap_or(cx.number(80));
    let png_zopfli:Handle<JsBoolean> = png_parameters.get(cx, "force_zopfli").unwrap_or(cx.boolean(false));
    let webp_quality:Handle<JsNumber> = webp_parameters.get(cx, "quality").unwrap_or(cx.number(60));
    let gif_quality:Handle<JsNumber> = gif_parameters.get(cx, "quality").unwrap_or(cx.number(80));


    compression_parameters.jpeg.quality = jpeg_quality.value(cx) as u32;
    compression_parameters.png.quality = png_quality.value(cx) as u32;
    compression_parameters.png.force_zopfli = png_zopfli.value(cx);
    compression_parameters.webp.quality = webp_quality.value(cx) as u32;
    compression_parameters.gif.quality = gif_quality.value(cx) as u32;

    compression_parameters.optimize = optimize.value(cx);
    compression_parameters.keep_metadata = keep_metadata.value(cx);
    compression_parameters.width = width.value(cx) as u32;
    compression_parameters.height = height.value(cx) as u32;

    compression_parameters
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("compress", compress)?;
    cx.export_function("compressToSize", compress_to_size)?;
    Ok(())
}
