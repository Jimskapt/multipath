#[cfg(test)]
mod tests;

pub fn from(input: &str) -> Vec<String> {
	let mut results: Vec<String> = Vec::new();

	let mut value_buffer = input.clone();

	while !value_buffer.is_empty() {
		// dbg!("new_loop", &value_buffer, &results);

		if let Some(first_open) = value_buffer.find("{") {
			// dbg!(&first_open);
			let value_close_buffer = &value_buffer[first_open..];
			// dbg!(&value_close_buffer);
			let mut found = false;
			if let Some(first_close) = value_close_buffer.find("}") {
				// dbg!(&first_close);
				let split_potential = &value_buffer[first_open + 1..first_open + first_close];
				let split_potential: Vec<&str> =
					split_potential.split(',').map(|e| e.trim()).collect();
				// dbg!(&split_potential);

				if split_potential.len() > 1 {
					if results.is_empty() {
						for split_content in &split_potential {
							let mut temp = String::from(&value_buffer[..first_open]);
							temp += split_content;
							results.push(temp);
						}
					} else {
						let mut new_results = vec![];

						for split_value in split_potential {
							for result in &results {
								let mut new_result = result.clone();
								new_result += &value_buffer[..first_open];
								new_result += split_value;

								new_results.push(new_result);
							}
						}

						results = new_results;
					}

					value_buffer = &value_buffer[first_open + first_close + 1..];
					found = true;
				}
			}

			// dbg!(&found);
			if !found {
				if results.is_empty() {
					results.push(value_buffer[..first_open + 1].to_owned());
				} else {
					let mut new_results = vec![];

					for result in &results {
						let mut new_result = result.clone();
						new_result += &value_buffer[..first_open + 1];

						new_results.push(new_result);
					}

					results = new_results;
				}

				value_buffer = &value_buffer[first_open + 1..];
			}
		} else {
			if results.is_empty() {
				results.push(value_buffer.to_owned());
			} else {
				let mut new_results = vec![];

				for result in &results {
					let mut new_result = result.clone();
					new_result += &value_buffer;

					new_results.push(new_result);
				}

				results = new_results;
			}

			value_buffer = "";
		}
	}

	return results;
}
