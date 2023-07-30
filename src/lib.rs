use caesium::CSParameters;
use neon::prelude::*;

fn compress(mut cx: FunctionContext) -> JsResult<JsObject> {
    let input = cx.argument::<JsString>(0)?;
    let output = cx.argument::<JsString>(1)?;
    let parameters_object = cx.argument::<JsObject>(2)?;
    let compression_parameters = parse_parameters(&mut cx, parameters_object);
    let result = cx.empty_object();
    let mut success = cx.boolean(false);
    let mut message = cx.string("");
    match caesium::compress(input.value(&mut cx), output.value(&mut cx), &compression_parameters) {
        Ok(_) => {
            success = cx.boolean(true);
        }
        Err(e) => {
            message = cx.string(e.to_string());
        }
    }
    result.set(&mut cx, "success", success)?;
    result.set(&mut cx, "message", message)?;

    Ok(result)
}

fn compress_to_size(mut cx: FunctionContext) -> JsResult<JsObject> {
    let input = cx.argument::<JsString>(0)?;
    let output = cx.argument::<JsString>(1)?;
    let parameters_object = cx.argument::<JsObject>(2)?;
    let mut compression_parameters = parse_parameters(&mut cx, parameters_object);
    let max_output_size = cx.argument::<JsNumber>(3)?;
    let result = cx.empty_object();
    let mut success = cx.boolean(false);
    let mut message = cx.string("");
    match caesium::compress_to_size(input.value(&mut cx), output.value(&mut cx), &mut compression_parameters, max_output_size.value(&mut cx) as usize) {
        Ok(_) => {
            success = cx.boolean(true);
        }
        Err(e) => {
            message = cx.string(e.to_string());
        }
    };

    result.set(&mut cx, "success", success)?;
    result.set(&mut cx, "message", message)?;

    Ok(result)
}

fn parse_parameters(cx: &mut FunctionContext, parameters_object: Handle<JsObject>) -> CSParameters
{
    let mut compression_parameters = caesium::initialize_parameters();

    let jpeg_parameters: Handle<JsObject> = parameters_object.get_opt(cx, "jpeg").unwrap().unwrap_or(cx.empty_object());
    let png_parameters: Handle<JsObject> = parameters_object.get_opt(cx, "png").unwrap().unwrap_or(cx.empty_object());
    let gif_parameters: Handle<JsObject> = parameters_object.get_opt(cx, "gif").unwrap().unwrap_or(cx.empty_object());
    let webp_parameters: Handle<JsObject> = parameters_object.get_opt(cx, "webp").unwrap().unwrap_or(cx.empty_object());

    let keep_metadata: Handle<JsBoolean> = parameters_object.get_opt(cx, "keep_metadata").unwrap().unwrap_or(cx.boolean(compression_parameters.keep_metadata));
    let optimize = parameters_object.get_opt(cx, "optimize").unwrap().unwrap_or(cx.boolean(false));
    let width: Handle<JsNumber> = parameters_object.get_opt(cx, "width").unwrap().unwrap_or(cx.number(compression_parameters.width));
    let height: Handle<JsNumber> = parameters_object.get_opt(cx, "height").unwrap().unwrap_or(cx.number(compression_parameters.height));

    let jpeg_quality: Handle<JsNumber> = jpeg_parameters.get_opt(cx, "quality").unwrap().unwrap_or(cx.number(compression_parameters.jpeg.quality));
    let png_quality: Handle<JsNumber> = png_parameters.get_opt(cx, "quality").unwrap().unwrap_or(cx.number(compression_parameters.png.quality));
    let png_zopfli: Handle<JsBoolean> = png_parameters.get_opt(cx, "force_zopfli").unwrap().unwrap_or(cx.boolean(compression_parameters.png.force_zopfli));
    let webp_quality: Handle<JsNumber> = webp_parameters.get_opt(cx, "quality").unwrap().unwrap_or(cx.number(compression_parameters.webp.quality));
    let gif_quality: Handle<JsNumber> = gif_parameters.get_opt(cx, "quality").unwrap().unwrap_or(cx.number(compression_parameters.gif.quality));

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
