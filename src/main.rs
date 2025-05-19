use std::fs;
use std::env;
use std::process;
use rand::seq::IndexedRandom as _;
use std::io::{self, Write};

mod fcd;

fn main()
{
	let args: Vec<String> = env::args().collect();

	if args.len() != 4
	{
		eprintln!("Usage: {} <file.fcd> <difficulty> <count>", args[0]);
		process::exit(1);
	}

	let file_path = &args[1];
	let difficulty_filter: u8 = match args[2].parse()
	{
		Ok(val) => val,
		Err(_) =>
		{
			eprintln!("Invalid difficulty: must be an integer");
			process::exit(1);
		}
	};

	let count: usize = match args[3].parse()
	{
		Ok(val) => val,
		Err(_) =>
		{
			eprintln!("Invalid count: must be an integer");
			process::exit(1);
		}
	};

	let file_content = fs::read_to_string(file_path)
		.unwrap_or_else(|_| {
			eprintln!("Failed to read file: {}", file_path);
			process::exit(1);
		});

	let cards = match fcd::parse_cards(&file_content)
	{
		Ok(cards) => cards,
		Err(e) =>
		{
			eprintln!("Parse error: {}", e);
			process::exit(1);
		}
	};

	if cards.is_empty()
	{
		eprintln!("No cards found.");
		process::exit(1);
	}

	run_flashcards(cards, difficulty_filter, count);
}

fn run_flashcards(all_cards: Vec<fcd::card::Card>, max_difficulty: u8, count: usize)
{
	// Weight cards based on difficulty
	let mut weighted_pool: Vec<&fcd::card::Card> = Vec::new();

	for card in &all_cards
	{
		if card.difficulty <= max_difficulty
		{
			// Higher difficulty means more weight
			let weight = card.difficulty as usize;
			for _ in 0..weight
			{
				weighted_pool.push(card);
			}
		}
	}

	if weighted_pool.is_empty()
	{
		eprintln!("No cards match the given difficulty filter.");
		return;
	}

	let mut rng = rand::rng();

	for i in 1..=count
	{
		let card = match weighted_pool.choose(&mut rng)
		{
			Some(c) => c,
			None =>
			{
				eprintln!("Failed to select a card.");
				break;
			}
		};

		println!("\nFlashcard {}/{}", i, count);

		print!("Q: {}", card.question);
		wait_for_enter(" ... ");

		println!("A: {}", card.answer);
	}
}

fn wait_for_enter(prompt: &str)
{
	print!("{}", prompt);
	io::stdout().flush().unwrap();

	let mut buffer = String::new();
	let _ = io::stdin().read_line(&mut buffer);
}
