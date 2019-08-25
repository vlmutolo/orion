fn compression_g(block_x: &[u64; 128], block_y: &[u64; 128], out: &mut [u64; 128]) {
	// R = X xor Y
	let mut block_r = [0u64; 128];
	for (el_r, (&el_x, &el_y)) in block_r.iter_mut().zip(block_x.iter().zip(block_y.iter())) {
		*el_r = el_x ^ el_y;
	}

	// Operate row-wise on R. Note that we treat each two consecutive u64
	// elements as a single u128. Thus, this is can effectively be interpreted
	// as an 8x8 matrix. This produces "Q" in the spec.
	let mut block_q = block_r;
	for slice_idx in 0..8 {
		let start_slice: usize = 16 * slice_idx;
		let end_slice: usize = 16 * (slice_idx + 1);
		permutation_p(&mut block_q[start_slice..end_slice])
	}

	// Operate column-wise on R. We continue to treat two consecutive
	// u64's as one u128. This produces "Z" in the spec.
	// The strategy below is to first build the columns into a contiguous array,
	// call the permutation function on that array, and then reassign that array to
	// the original elements.
	let mut block_z = block_q;
	let mut temp_col_buffer = [0u64; 16];
	for slice_idx in 0..8 {
		// Assign block_z -> buffer
		for row_idx in 0..8 {
			temp_col_buffer[row_idx] = block_z[row_idx * 2 * 8 + slice_idx * 2];
			temp_col_buffer[row_idx + 1] = block_z[row_idx * 2 * 8 + 1 + slice_idx * 2];
		}

		permutation_p(&mut temp_col_buffer);

		// Assign buffer -> block_z
		for row_idx in 0..8 {
			block_z[row_idx * 2 * 8 + slice_idx * 2] = temp_col_buffer[row_idx];
			block_z[row_idx * 2 * 8 + 1 + slice_idx * 2] = temp_col_buffer[row_idx + 1];
		}
	}

	// Use the provided `out` block to "return" Z xor R
	for (el_out, (&el_z, &el_r)) in out.iter_mut().zip(block_z.iter().zip(block_r.iter())) {
		*el_out = el_z ^ el_r;
	}
}

fn permutation_p(s: &mut [u64]) {
	mixing_g(0, 4, 8, 12, s);
	mixing_g(1, 5, 9, 13, s);
	mixing_g(2, 6, 10, 14, s);
	mixing_g(3, 7, 11, 15, s);
	mixing_g(0, 5, 10, 15, s);
	mixing_g(1, 6, 11, 12, s);
	mixing_g(2, 7, 8, 13, s);
	mixing_g(3, 4, 9, 14, s);
}

fn mixing_g(a: usize, b: usize, c: usize, d: usize, w: &mut [u64]) {
	w[a] = g_mix_add_mult(w[a], w[b]);
	w[d] = g_mix_xor_shift(w[d], w[a], 32);
	w[c] = g_mix_add_mult(w[c], w[d]);
	w[b] = g_mix_xor_shift(w[b], w[c], 24);
	w[a] = g_mix_add_mult(w[a], w[b]);
	w[d] = g_mix_xor_shift(w[d], w[a], 16);
	w[c] = g_mix_add_mult(w[c], w[d]);
	w[b] = g_mix_xor_shift(w[b], w[c], 63);
}

fn g_mix_add_mult(x: u64, y: u64) -> u64 {
	let mask = 0xFFFFFFFFu64;
	let x_l = x & mask;
	let y_l = y & mask;
	x.wrapping_add(y)
		.wrapping_add(2)
		.wrapping_mul(x_l)
		.wrapping_mul(y_l)
}

fn g_mix_xor_shift(x: u64, y: u64, rotate_num: u32) -> u64 { (x ^ y).rotate_right(rotate_num) }
