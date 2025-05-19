use std::any::Any;

pub trait DataEntryTrait: Any
{
	fn get_name(&self) -> String;
	 
	fn as_any(&self) -> &dyn Any;
}

#[derive(Clone)]
pub struct IntegerDataEntry
{
	name: String,
	val: u8
}

impl IntegerDataEntry
{
	fn new(name: String, val: u8) -> Self
	{
		IntegerDataEntry { name, val }
	}

	pub fn get_integer(&self) -> u8
	{
		self.val
	}
}

impl DataEntryTrait for IntegerDataEntry
{
	fn get_name(&self) -> String
	{
		self.name.clone()
	}

	fn as_any(&self) -> &dyn Any
	{
		self
	}
}

#[derive(Clone)]
pub struct StringDataEntry
{
	name: String,
	val: String
}

impl StringDataEntry
{
	fn new(name: String, val: String) -> Self
	{
		StringDataEntry { name, val }
	}

	pub fn get_string(&self) -> String
	{
		self.val.clone()
	}
}

impl DataEntryTrait for StringDataEntry
{
	fn get_name(&self) -> String
	{
		self.name.clone()
	}

	fn as_any(&self) -> &dyn Any
	{
		self
	}	
}

pub struct DataEntry
{
	obj: Box<dyn DataEntryTrait>
}

impl DataEntry
{
	pub fn get_name(&self) -> String
	{
		self.obj.get_name().clone()
	}

	pub fn new(obj: Box<dyn DataEntryTrait>) -> Self
	{
		DataEntry { obj }
	}

	pub fn new_integer(name: String, val: u8) -> Self
	{
		DataEntry::new(Box::new(IntegerDataEntry::new(name, val)))
	}

	pub fn as_integer(&self) -> Option<IntegerDataEntry>
	{
		self.obj.as_any().downcast_ref::<IntegerDataEntry>().cloned()
	}

	pub fn new_string(name: String, val: String) -> Self
	{
		DataEntry::new(Box::new(StringDataEntry::new(name, val)))
	}

	pub fn as_string(&self) -> Option<StringDataEntry>
	{
		self.obj.as_any().downcast_ref::<StringDataEntry>().cloned()
	}
}
