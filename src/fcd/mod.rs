mod data_entry;
use data_entry::DataEntry;

mod card;
use card::Card;

fn build_card(entries: Vec<DataEntry>) -> Result<Card, Box<dyn std::error::Error>>
{
	let mut difficulty: Option<u8> = None;
	let mut question: Option<String> = None;
	let mut answer: Option<String> = None;

	for entry in entries
	{
		let name = entry.get_name();
		match name.as_str()
		{
			"difficulty" =>
			{
				if let Some(int_data) = entry.as_integer()
				{
					difficulty = Some(int_data.get_integer());
				}
			}
			"question" =>
			{
				if let Some(str_data) = entry.as_string()
				{
					question = Some(str_data.get_string());
				}
			}
			"answer" =>
			{
				if let Some(str_data) = entry.as_string()
				{
					answer = Some(str_data.get_string());
				}
			}
			_ =>
			{
				return Err(format!("Unexpected field: {}", name).into());
			}
		}
	}

	match (difficulty, question, answer)
	{
		(Some(difficulty), Some(question), Some(answer)) =>
		{
			Ok(Card::new(difficulty, question, answer))
		}
		_ =>
		{
			Err("Missing one or more required fields (difficulty, question, answer)".into())
		}
	}
}

fn parse_data_entry(line: &str) -> Result<DataEntry, Box<dyn std::error::Error>>
{
	let parts: Vec<&str> = line.splitn(2, ':').collect();
	if parts.len() != 2
	{
		return Err("Invalid data entry line format".into());
	}

	let key = parts[0].trim().to_string();
	let value_str = parts[1].trim();

	if key == "difficulty"
	{
		let val = value_str.parse::<u8>()?;
		return Ok(DataEntry::new_integer(key, val));
	}
	else
	{
		if !value_str.starts_with('"') || !value_str.ends_with('"')
		{
			return Err(format!("Expected quoted string for key '{}'", key).into());
		}

		let val = value_str[1..value_str.len() - 1].to_string();
		return Ok(DataEntry::new_string(key, val));
	}
}

pub fn parse_cards(input: &str) -> Result<Vec<Card>, Box<dyn std::error::Error>>
{
	let mut cards = Vec::new();
	let mut lines = input.lines().peekable();

	while let Some(line) = lines.next()
	{
		if line.trim() != "::card"
		{
			continue;
		}

		let mut entries: Vec<DataEntry> = Vec::new();

		while let Some(next_line) = lines.peek()
		{
			if next_line.trim().is_empty() || next_line.trim() == "::card"
			{
				break;
			}

			let line = lines.next().unwrap().trim();
			let entry = parse_data_entry(line)?;
			entries.push(entry);
		}

		let card = build_card(entries)?;
		cards.push(card);
	}

	Ok(cards)
}
