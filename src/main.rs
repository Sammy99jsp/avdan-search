use v8;
use std::env;
use std::fs;
use std::panic;

use crate::api::clipboard;
use crate::api::clipboard::ClipSource;
use crate::api::debug_bind;
use crate::api::utils_js;

mod api;



fn main() {

    let args: Vec<String> = env::args().collect();

    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    {
        // Create a new Isolate and make it the current one.
        let isolate = &mut v8::Isolate::new(v8::CreateParams::default());

        // Create a stack-allocated handle scope.
        let handle_scope = &mut v8::HandleScope::new(isolate);

        // Create a new context.
        let context = v8::Context::new(handle_scope);


        // Enter the context for script compilation and execution
        let scope = &mut v8::ContextScope::new(handle_scope, context);

        // Make a global scope thing-y
        let global = context.global(scope);

        // Avdan Global Interface
        let key_avdan = v8::String::new(scope, "Avdan").unwrap();
        
        // The global Avdan Search API object.
        let avdan_obj = v8::Object::new(scope);

        // Avdan.Debug API
        {
            let debug = v8::Object::new(scope);

            // Avdan.Debug.log
            {
                let label = v8::String::new(scope, "log").unwrap();
                let func = v8::FunctionBuilder::<v8::Function>::new(debug_bind::log).build(scope).unwrap();
            
                debug.set(scope, label.into(), func.into());
            }

            // Avdan.Debug
            let label = v8::String::new(scope, "Debug").unwrap();
            
            avdan_obj.set(
                scope,
                label.into(),
                debug.into()
            );
        }

        // Avdan.File API 
        {
            let file_api = api::file::AvFile::new().js(scope);

            let file_label = v8::String::new(scope, "File").unwrap();
            
            avdan_obj.set(
                scope,
                file_label.into(),
                file_api.into()
            );
        }

        // // Avdan.Clipboard API
        {
            let clipboard = clipboard::Clip::JS(scope);

            // Avdan.Clipboard
            let clipboard_label = v8::String::new(scope, "Clipboard").unwrap();
            
            avdan_obj.set(
                scope,
                clipboard_label.into(),
                clipboard.into()
            );
        }
        

        global.set(
            scope,
            key_avdan.into(),
            avdan_obj.into()
        );

        
        let source_code = fs::read_to_string(args.get(1).unwrap()).expect("Couldn't read file!");

        // Create a string containing the JavaScript source code.
        let code = v8::String::new(scope, &source_code).unwrap();


        // Compile the source code.
        let script = v8::Script::compile(scope, code, None);

        // Check if there was an error in the javascript
        if script.is_some() {
            // Run the script to get the result.
            script.unwrap().run(scope).unwrap();
        }

    }
    unsafe {
        v8::V8::dispose();
    }

    v8::V8::dispose_platform();
}


// fn fn_callback(
//     scope: &mut v8::HandleScope,
//     _args: v8::FunctionCallbackArguments,
//     mut rv: v8::ReturnValue,
//   ) {
//       println!("Function called with {} args", _args.length());
//       if _args.length() > 0 {
//         for i in 0.._args.length() {
//             if type_of(_args.get(i)).eq("object") || type_of(_args.get(i)).eq("array")
//             {
//                 let tmp = _args.get(i);
//                 println!("{0} ({1}) => {2}", i, type_of(_args.get(i)), to_string(scope, tmp));
//             }
//             else
//             {
//                 println!("{0} ({1}) => `{2}`", i, type_of(_args.get(i)), _args.get(i).to_rust_string_lossy(scope));
//             }
//         }
//       }
//     let s = v8::String::new(scope, "\nDisplay help from Avdan object.").unwrap();
//     rv.set(s.into());
// }


fn to_string(scope : &mut v8::HandleScope, value : v8::Local<v8::Value>) -> String {
    if value.is_object() {
        let value = value.to_object(scope);
        let keys = value.unwrap().get_own_property_names(scope).unwrap();
        let mut out = String::from("\n\t");
        for i in 0..keys.length() {
            let k = v8::Number::new(scope, i.into());
            let key = keys.get(scope, k.into()).unwrap();
            let value = value.unwrap().get(scope, key.into()).unwrap();
            out.push_str(format!("{0} => `{1}`\n\t", key.to_rust_string_lossy(scope), value.to_rust_string_lossy(scope)).as_str());
        }

        out.pop();
        out.pop();
        return out;
    }

    return String::from("");
}