use std::collections::BTreeMap;

use colored::Colorize;
use pest::error::LineColLocation;

pub fn print_error_header(name: &str, file_name: &str) {
	println!("{} {}", "error:".red(), name);
	println!("  {} {}", "-->".blue(), file_name);
}

fn reformat_line(line: &str) -> (String, isize) {
	let mut offset = 0;
	for character in line.chars() {
		if character == '\t' {
			offset += 3;
		} else {
			break;
		};
	}

	return (line.replace("\t", "    "), offset);
}

pub fn print_line_with_squiggle(
	lines: BTreeMap<usize, String>,
	location: &LineColLocation,
	message: &str,
	pos_offset: isize,
) {
	let (start_line, mut start_pos, _, mut end_pos) = match *location {
		LineColLocation::Pos((start_line, start_pos)) => (
			start_line,
			start_pos as isize + pos_offset,
			0,
			start_pos as isize + 1 + pos_offset,
		),
		LineColLocation::Span((start_line, start_pos), (end_line, end_pos)) => (
			start_line,
			start_pos as isize + pos_offset,
			end_line,
			end_pos as isize + pos_offset,
		),
	};

	let (line, tab_offset) = reformat_line(lines.get(&start_line).unwrap());
	start_pos += tab_offset;
	end_pos += tab_offset;

	assert!(end_pos > start_pos);

	let line_indicator = format!("{} |", start_line).blue();
	let blank_indicator = format!("{} |", " ".repeat(line_indicator.len() - 2)).blue();
	let squiggle = format!(
		"{}{} {}",
		" ".repeat(start_pos as usize),
		"^".repeat((end_pos - start_pos) as usize),
		message
	)
	.red();

	println!("{}", blank_indicator);

	for (line_number, line) in lines.iter() {
		if line_number >= &start_line {
			break;
		}

		println!("{} {}", format!("{} |", line_number).blue(), line);
	}

	println!("{} {}", line_indicator, line);
	println!("{} {}", blank_indicator, squiggle);

	let mut printed_lines = false;
	for (line_number, line) in lines.iter() {
		if line_number > &start_line {
			println!("{} {}", format!("{} |", line_number).blue(), line);
			printed_lines = true;
		}
	}

	if printed_lines {
		println!("{}", blank_indicator);
	}
}

pub fn print_dots() {
	println!("{}", "...".blue());
}

pub fn print_blank() {
	println!("");
}

pub fn print_line_with_correction(
	line: &str,
	insert_location: &LineColLocation,
	added_code: &str,
	caption: &str,
) {
	let (line, tab_offset) = reformat_line(line);

	let (start_line, start_pos) = match *insert_location {
		LineColLocation::Pos((start_line, start_pos)) => {
			(start_line, start_pos + tab_offset as usize)
		}
		LineColLocation::Span((start_line, start_pos), (_, _)) => {
			(start_line, start_pos + tab_offset as usize)
		}
	};

	let corrected = format!(
		"{}{}{}",
		&line[..start_pos - 1],
		added_code.yellow(),
		&line[start_pos - 1..]
	);

	let caption = format!(
		"{}{} {}",
		" ".repeat(start_pos - 1),
		"^".repeat(added_code.len()),
		caption
	)
	.yellow();

	let line_indicator = format!("{} |", start_line).blue();
	let blank_indicator = format!("{} |", " ".repeat(line_indicator.len() - 2)).blue();
	println!("{}", blank_indicator);
	println!("{} {}", line_indicator, corrected);
	println!("{} {}", blank_indicator, caption);
	println!("{}", blank_indicator);
}
