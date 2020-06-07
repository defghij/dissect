
//TODO make these generic

// Define a function to take in a slice and return a u32 number

pub fn vec_slice_to_u16(slice: &[u8]) -> Option<u16> {
		if slice.len() != 2 {
				return None;
		}
		let mut sum: u16 = 0; // declare mut u32 to hold final result
		for idx in 0..slice.len(){ // iterate over the len of the slice 
				// Shift = (total_len - 1 - current_index) * bits_in_byte
				sum += u16::from(slice[idx]) << ((slice.len()- 1 - idx) * 8);
		}
		return Some(sum);
}

pub fn vec_slice_to_u32(slice: &[u8]) -> Option<u32> {
		if slice.len() != 4 {
				return None;
		}
		let mut sum: u32 = 0;
		for idx in 0..slice.len(){ 
				sum += u32::from(slice[idx]) << ((slice.len()- 1 - idx) * 8);
		}
		return Some(sum);
}

pub fn vec_slice_to_u64(slice: &[u8]) -> Option<u64> {
		if slice.len() != 8 {
				return None;
		}
		let mut sum: u64 = 0;
		for idx in 0..slice.len(){ 
				sum += u64::from(slice[idx]) << ((slice.len()- 1 - idx) * 8);
		}
		return Some(sum);
}

