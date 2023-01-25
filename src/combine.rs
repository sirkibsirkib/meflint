
use crate::Values;
use crate::Data;

struct Slot<'a> {
	next: usize,
	datas: Vec<&'a Data>,
}
pub(crate	) struct Combination<'a> {
	slots: Vec<Slot<'a>>,
	stop: bool 
}

impl<'a> Combination<'a> {
	pub fn new(args: &'a [Values]) -> Self {
		let slots: Vec<Slot> = args.iter()
			.map(|values| Slot { next: 0, datas: values.datas.iter().collect()}).collect();
		let stop = slots.is_empty() || slots.iter().any(|s| s.datas.is_empty());
        Self { slots, stop }
	}
	pub fn next(&mut self) -> Option<Data> {
		// for slot in self.slots.iter() {
		// 	print!("{:?} ", slot.next);
		// }
		// println!();
		if self.stop {
			return	None
		} 
		// build datas
		let ret: Data = {
			let mut v = vec![];
			for slot in  self.slots.iter() {
				v.extend(&slot.datas[slot.next].0);
			}
			Data(v)
		};

		// advance
		'outer: loop {
			for slot in self.slots.iter_mut().rev() {
				assert!(!slot.datas.is_empty());
				slot.next += 1;
				if slot.datas.len() <= slot.next {
					slot.next = 0;
				} else {
					break 'outer;
				}
			}
			self.stop = true;
			break 'outer;
		}
		Some(ret)
	}
}