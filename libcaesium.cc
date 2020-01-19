#include <node.h>
#include "./build/libcaesium/include/caesium.h"

using namespace std;

namespace libcaesium {

    using v8::FunctionCallbackInfo;
    using v8::Isolate;
    using v8::Local;
    using v8::Object;
    using v8::String;
    using v8::Value;

    void compress(const FunctionCallbackInfo <Value> &args) {
        Isolate *isolate = args.GetIsolate();
        String::Utf8Value v8_input(isolate, args[0]);
        String::Utf8Value v8_output(isolate, args[1]);
        string input(*v8_input);
        string output(*v8_output);
        const char *c_input = input.c_str();
        const char *c_output = output.c_str();

        int err_n = 0;
        cs_image_pars compress_options;
        compress_options = initialize_parameters();


        bool compression_result = cs_compress(c_input, c_output, &compress_options, &err_n);

        args.GetReturnValue().Set(compression_result);
    }

    void Initialize(Local <Object> exports) {
        NODE_SET_METHOD(exports, "compress", compress);
    }

    NODE_MODULE(NODE_GYP_MODULE_NAME, Initialize)

}
