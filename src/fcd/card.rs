#[derive(Debug)]
pub struct Card
{
	pub difficulty: u8,
	pub question: String,
	pub answer: String,
}

impl Card
{
	pub fn new(difficulty: u8, question: String, answer: String) -> Card
	{
		Card { difficulty, question, answer }
	}
}
