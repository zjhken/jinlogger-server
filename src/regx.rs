use std::io::{stdin, stdout, BufRead, Read, Write};

use anyhow_ext::{bail, Context};
use concat_string::concat_string;
use regex::Regex;
use seafloor::anyhow::Result;

pub fn main() -> Result<()> {
	let args: Vec<String> = std::env::args().collect();
	let regex_str = match args.first() {
		Some(s) => s,
		None => bail!("regex to extract fields is required."),
	};

	read_in_and_write_out(stdin().lock(), stdout(), &regex_str)?;

	return Ok(());
}

fn read_in_and_write_out<R: Read + BufRead, W: Write>(
	mut input: R,
	mut output: W,
	regex_str: &str,
) -> Result<()> {
	let group_name = extract_group_names(regex_str, r"\(\?<(\w+)>");
	let regex = Regex::new(regex_str).context("failed to compile regex")?;
	let first_line = group_name.join("\t") + "\n";
	let _ = stdout()
		.write(first_line.as_bytes())
		.context("failed to write to stdout")?;

	let mut buf = String::new();
	loop {
		let n = input
			.read_line(&mut buf)
			.context("failed to read line from stdin")?;
		if n == 0 {
			break;
		}
		let pat = group_name
			.iter()
			.map(|s| concat_string!("$", s))
			.collect::<Vec<String>>()
			.join("\t");
		// debug!(pat = pat, "got pat");
		let line = regex.replace_all(&buf, pat);
		let line = concat_string!(line, "\n");
		output
			.write(line.as_bytes())
			.context("failed to write data line to stdout")?;
		buf.clear();
	}
	return Ok(());
}

fn extract_group_names(input: &str, reg_str: &str) -> Vec<String> {
	let group_name_pattern = Regex::new(reg_str).unwrap();
	let x: Vec<String> = group_name_pattern
		.captures_iter(&input)
		.map(|cap| {
			let a = &cap[1];
			return a.to_owned();
		})
		.collect();
	return x;
}

#[cfg(test)]
mod test {

	use seafloor::anyhow::Result;
	use std::io::stdout;

	use crate::read_in_and_write_out;
	use tracing_test::traced_test;

	#[test]
	#[traced_test]
	fn test_a() -> Result<()> {
		let input = "2023-10-01 INF test a
2023-10-02 INF test b
2023-10-02 ERR test x
2023-10-03 INF test c
2023-10-04 INF test d
2023-10-05 INF test e
";
		read_in_and_write_out(
			input.as_bytes(),
			stdout(),
			r"^(?<date>\d{4}-\d{2}-\d{2}) (?<level>\w{3,5}) (?<msg>.*)\n$",
		)?;
		Ok(())
	}
}
