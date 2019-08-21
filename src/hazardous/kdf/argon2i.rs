/// Fake docs, fake function. Why? Angry clippy.
pub fn fake() {
    permutation_p(&mut [1u128,0,0,0,0,0,0,0]);
}


fn permutation_p(s: &mut [u128; 8]) {
	// Initialize an array to store the new 16 u64's.
	let mut w = [0u64; 16];

    // Reinterpret the S-box values to be twice as many values of half the size.
    // 1x(u128) -> 2x(u64)
    for (idx, &s_el) in s.iter().enumerate() {
        w[idx * 2] = (s_el >> 64) as u64;   // Set most significant half
        w[idx * 2 + 1] = s_el as u64;       // Set least significant half
    }

	mixing_function_g(0, 4, 8, 12, &mut w);
	mixing_function_g(1, 5, 9, 13, &mut w);
	mixing_function_g(2, 6, 10, 14, &mut w);
	mixing_function_g(3, 7, 11, 15, &mut w);
	mixing_function_g(0, 5, 10, 15, &mut w);
	mixing_function_g(1, 6, 11, 12, &mut w);
	mixing_function_g(2, 7, 8, 13, &mut w);
	mixing_function_g(3, 4, 9, 14, &mut w);

    // Reinterpret the mixed w values to be half as many values of twice the size.
    // 2x(u64) -> 1x(u128)
    for (w_chunk, s_el) in w
        .chunks_exact(2)
        .zip(s.iter_mut()) {
        let (most_sig, least_sig) = (w_chunk[0], w_chunk[1]);
        *s_el = (most_sig as u128) << 64 | least_sig as u128;
    }
}

fn mixing_function_g(a: usize, b: usize, c: usize, d: usize, w: &mut [u64]) {
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
	x.wrapping_add(y).wrapping_add(2).wrapping_mul(x_l).wrapping_mul(y_l)
}

fn g_mix_xor_shift(x: u64, y: u64, rotate_num: u32) -> u64 {
	(x ^ y).rotate_right(rotate_num)
}


