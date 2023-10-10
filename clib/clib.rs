extern crate libc;
use std::{ffi::CStr, ptr::null};

#[repr(C)]
pub struct CompileResult {
    data: *const libc::c_void,
    len: libc::size_t,
    err: u8,
    msg: *const libc::c_char
}

fn module_to_spv(module: naga::Module) -> Result<Vec<u8>, String> {
    use naga::back::spv;

    let validation_caps = naga::valid::Capabilities::all();
    let validation_flags = naga::valid::ValidationFlags::all();

    let info = match naga::valid::Validator::new(validation_flags, validation_caps)
        .validate(&module)
    {
        Ok(info) => info,
        Err(error) => {
            return Result::Err(error.to_string());
            // if let Some(input) = &input_text {
            //     let filename = input_path.file_name().and_then(std::ffi::OsStr::to_str);
            //     emit_annotated_error(&error, filename.unwrap_or("input"), input);
            // }
            // print_err(&error);
            // panic!("");
            // None
        }
    };

    let mut options = naga::back::spv::Options::default();
    options.lang_version = (1, 3);
    // options.bounds_check_policies = 


    // let pipeline_options_owned;
    let pipeline_options = None;
    // match params.entry_point {
    //     Some(ref name) => {
    //         let ep_index = module
    //             .entry_points
    //             .iter()
    //             .position(|ep| ep.name == *name)
    //             .expect("Unable to find the entry point");
    //         pipeline_options_owned = spv::PipelineOptions {
    //             entry_point: name.clone(),
    //             shader_stage: module.entry_points[ep_index].stage,
    //         };
    //         Some(&pipeline_options_owned)
    //     }
    //     None => None,
    // };

    // params.spv.bounds_check_policies = params.bounds_check_policies;

    //Insert Debug infos
    // params.spv.debug_info = args.generate_debug_symbols.and_then(|debug| {
    //     params.spv.flags.set(spv::WriterFlags::DEBUG, debug);

    //     if debug {
    //         Some(spv::DebugInfo {
    //             source_code: input_text.as_ref()?,
    //             file_name: input_path.file_name().and_then(std::ffi::OsStr::to_str)?,
    //         })
    //     } else {
    //         None
    //     }
    // });

    options.flags.set(
        spv::WriterFlags::ADJUST_COORDINATE_SPACE,
        false
    );

    let spv = match spv::write_vec(
        &module,
        &info,
        &options,
        pipeline_options,
    ) {
        Ok(spv) => spv,
        Err(e) => {
            return Result::Err(e.to_string());
        }
    };
    let bytes = spv
        .iter()
        .fold(Vec::with_capacity(spv.len() * 4), |mut v, w| {
            v.extend_from_slice(&w.to_le_bytes());
            v
        });
    Ok(bytes)
}

#[no_mangle]
#[allow(non_snake_case)]
pub fn qWEjz_klm_compile_wgsl(path: *const libc::c_char, ) -> CompileResult {
    let c_str: &CStr = unsafe { CStr::from_ptr(path) };
    let str_slice: &str = c_str.to_str().unwrap();
    let input: String = str_slice.to_owned();  // if necessary
    let result = naga::front::wgsl::parse_str(&input);
    match result {
        Ok(module) => {
            println!("Parse Into IR Complete!");

            match module_to_spv(module) {
                Ok(data) => unsafe {
                    println!("Convert to SPIR-V Complete!");
                    // unsafe {
                        let c_data: *mut libc::c_void = libc::malloc(data.len());
                        libc::memcpy(c_data, data.as_ptr() as *const libc::c_void, data.len());
                        return CompileResult {
                            data: c_data,
                            len: data.len(),
                            err: u8::from(false),
                            msg: null()
                        }
                    // }
                },
                Err(e) => unsafe {
                    // unsafe {
                        let c_str: *mut libc::c_void = libc::malloc(e.len());
                        libc::memcpy(c_str, e.as_ptr() as *const libc::c_void, e.len());
                        return CompileResult {
                            data: null(),
                            len: 0,
                            err: u8::from(true),
                            msg: c_str as *const libc::c_char
                        }
                    // }
                }
            }
            // naga::back::spv::
        },
        Err(ref e) => unsafe {
            // unsafe {
                let e_str = e.emit_to_string(&input);
                // let e_str = e.to_string();
                let c_str: *mut libc::c_void = libc::malloc(e_str.len());
                libc::memcpy(c_str, e_str.as_ptr() as *const libc::c_void, e_str.len());
                return CompileResult {
                    data: null(),
                    len: 0,
                    err: u8::from(true),
                    msg: c_str as *const libc::c_char
                }
            // }
            // // let path = input_path.to_string_lossy();
            // e.emit_to_stderr_with_path(&input, &input);
            // println!("Could not parse WGSL");
        }
    }
}