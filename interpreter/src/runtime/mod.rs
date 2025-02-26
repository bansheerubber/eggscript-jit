mod mapping;
pub mod print;

pub use mapping::get_native_function_mapping_for_interpreter;
pub use mapping::get_native_function_mapping_for_jit;
pub use mapping::get_test_native_function_mapping_for_interpreter;
pub use mapping::get_test_native_function_mapping_for_jit;
