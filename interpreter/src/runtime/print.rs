use crate::Value;

pub fn print_double(values: Vec<Value>) -> Value {
	if values.len() == 0 {
		return Value::Null;
	}

	match values.first().unwrap() {
		Value::Boolean(value) => println!("{}", value),
		Value::Double(value) => println!("{}", value),
		Value::Integer(value) => println!("{}", value),
		Value::Null => println!("null"),
	}

	Value::Null
}
