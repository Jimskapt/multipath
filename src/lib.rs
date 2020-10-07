#[cfg(test)]
mod tests;

const FLAG: &str = "[grgnrt64sg@rth144615r0jk]";

/**
Split `input` as vector of string according to `{a,b,c}` schema.

**Warning :** in some specific case, result can return characters which are not
supported by O.S. paths, like `'{'` or `','`.
Read some [tests](https://github.com/Jimskapt/multipath/blob/master/src/tests.rs)
in order to know more about this cases.

# Example

```rust
assert_eq!(
	multipath::parse("/home/{user,admin}/{Desktop,Download}/file.txt"),
	vec![
		"/home/user/Desktop/file.txt",
		"/home/user/Download/file.txt",
		"/home/admin/Desktop/file.txt",
		"/home/admin/Download/file.txt",
	]
);
```
*/
pub fn parse(input: &str) -> Vec<String> {

	// "start" begins after an `{`.
	let mut start_pos = 0;
	// "level" is current level of `{`.
	// example : in `` level is 0, in `{` level is 1, in `{{` level is 2, in `{{{` level is 3, ...
	let mut level = 0;

	// a new element begins after an `{` or an `,`.
	// example : in `{a,b,c}`, elements are `a`, `b` and `c`.
	let mut previous_element_pos = 0;
	let mut elements = vec![];

	for (i, character) in input.chars().enumerate() {
		if character == '{' {

			level += 1;

			if level == 1 {
				start_pos = i;
				previous_element_pos = start_pos;
			}

		} else if character == ',' && level == 1 {

			let recursive_elements = parse(&input[previous_element_pos + 1..i].trim());
			for element in recursive_elements {
				elements.push(element);
			}

			previous_element_pos = i;

		} else if character == '}' {

			if level == 1 {
				let last_recursive_elements = parse(&input[previous_element_pos + 1..i].trim());
				for element in last_recursive_elements {
					elements.push(element.clone());
				}

				if !elements.is_empty() {
					return elements
						.iter()
						.flat_map(|element| {
							let mut res = vec![];

							let recursive_elements = parse(&input[i + 1..]);
							for recursive_element in recursive_elements {
								let mut temp = String::from(&input[..start_pos]);
								temp += &element;
								temp += &recursive_element;

								res.push(temp);
							}

							res
						})
						.collect();
				} else {
					return vec![String::from(input)];
				}
			}

			if level > 0 {
				level -= 1;
			}

		}
	}

	if level > 0 {
		let mut last_open = 0;
		for (i, character) in input.chars().enumerate() {
			if character == '{' {
				last_open = i;
			}
		}

		if last_open > 0 {
			let mut temp = String::from(&input[..last_open]);
			temp += FLAG;
			temp += &input[(last_open + 1)..];

			dbg!(&temp);

			let res = parse(&temp);

			return res
				.iter()
				.map(|element| element.replace(FLAG, "{"))
				.collect();
		}
	}

	return vec![String::from(input)];
}
