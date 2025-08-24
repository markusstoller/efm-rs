pub struct EFM {
    decode_mappings: [u16; 0x4000],
    encode_mappings: [u16; 256],
}

static EFM_TABLE_RAW: [u16; 256] = [
    0b01001000100000,
    0b10000100000000,
    0b10010000100000,
    0b10001000100000,
    0b01000100000000,
    0b00000100010000,
    0b00010000100000,
    0b00100100000000,
    0b01001001000000,
    0b10000001000000,
    0b10010001000000,
    0b10001001000000,
    0b01000001000000,
    0b00000001000000,
    0b00010001000000,
    0b00100001000000,
    0b10000000100000,
    0b10000010000000,
    0b10010010000000,
    0b00100000100000,
    0b01000010000000,
    0b00000010000000,
    0b00010010000000,
    0b00100010000000,
    0b01001000010000,
    0b10000000010000,
    0b10010000010000,
    0b10001000010000,
    0b01000000010000,
    0b00001000010000,
    0b00010000010000,
    0b00100000010000,
    0b00000000100000,
    0b10000100001000,
    0b00001000100000,
    0b00100100100000,
    0b01000100001000,
    0b00000100001000,
    0b01000000100000,
    0b00100100001000,
    0b01001001001000,
    0b10000001001000,
    0b10010001001000,
    0b10001001001000,
    0b01000001001000,
    0b00000001001000,
    0b00010001001000,
    0b00100001001000,
    0b00000100000000,
    0b10000010001000,
    0b10010010001000,
    0b10000100010000,
    0b01000010001000,
    0b00000010001000,
    0b00010010001000,
    0b00100010001000,
    0b01001000001000,
    0b10000000001000,
    0b10010000001000,
    0b10001000001000,
    0b01000000001000,
    0b00001000001000,
    0b00010000001000,
    0b00100000001000,
    0b01001000100100,
    0b10000100100100,
    0b10010000100100,
    0b10001000100100,
    0b01000100100100,
    0b00000000100100,
    0b00010000100100,
    0b00100100100100,
    0b01001001000100,
    0b10000001000100,
    0b10010001000100,
    0b10001001000100,
    0b01000001000100,
    0b00000001000100,
    0b00010001000100,
    0b00100001000100,
    0b10000000100100,
    0b10000010000100,
    0b10010010000100,
    0b00100000100100,
    0b01000010000100,
    0b00000010000100,
    0b00010010000100,
    0b00100010000100,
    0b01001000000100,
    0b10000000000100,
    0b10010000000100,
    0b10001000000100,
    0b01000000000100,
    0b00001000000100,
    0b00010000000100,
    0b00100000000100,
    0b01001000100010,
    0b10000100100010,
    0b10010000100010,
    0b10001000100010,
    0b01000100100010,
    0b00000000100010,
    0b01000000100100,
    0b00100100100010,
    0b01001001000010,
    0b10000001000010,
    0b10010001000010,
    0b10001001000010,
    0b01000001000010,
    0b00000001000010,
    0b00010001000010,
    0b00100001000010,
    0b10000000100010,
    0b10000010000010,
    0b10010010000010,
    0b00100000100010,
    0b01000010000010,
    0b00000010000010,
    0b00010010000010,
    0b00100010000010,
    0b01001000000010,
    0b00001001001000,
    0b10010000000010,
    0b10001000000010,
    0b01000000000010,
    0b00001000000010,
    0b00010000000010,
    0b00100000000010,
    0b01001000100001,
    0b10000100100001,
    0b10010000100001,
    0b10001000100001,
    0b01000100100001,
    0b00000000100001,
    0b00010000100001,
    0b00100100100001,
    0b01001001000001,
    0b10000001000001,
    0b10010001000001,
    0b10001001000001,
    0b01000001000001,
    0b00000001000001,
    0b00010001000001,
    0b00100001000001,
    0b10000000100001,
    0b10000010000001,
    0b10010010000001,
    0b00100000100001,
    0b01000010000001,
    0b00000010000001,
    0b00010010000001,
    0b00100010000001,
    0b01001000000001,
    0b10000010010000,
    0b10010000000001,
    0b10001000000001,
    0b01000010010000,
    0b00001000000001,
    0b00010000000001,
    0b00100010010000,
    0b00001000100001,
    0b10000100001001,
    0b01000100010000,
    0b00000100100001,
    0b01000100001001,
    0b00000100001001,
    0b01000000100001,
    0b00100100001001,
    0b01001001001001,
    0b10000001001001,
    0b10010001001001,
    0b10001001001001,
    0b01000001001001,
    0b00000001001001,
    0b00010001001001,
    0b00100001001001,
    0b00000100100000,
    0b10000010001001,
    0b10010010001001,
    0b00100100010000,
    0b01000010001001,
    0b00000010001001,
    0b00010010001001,
    0b00100010001001,
    0b01001000001001,
    0b10000000001001,
    0b10010000001001,
    0b10001000001001,
    0b01000000001001,
    0b00001000001001,
    0b00010000001001,
    0b00100000001001,
    0b01000100100000,
    0b10000100010001,
    0b10010010010000,
    0b00001000100100,
    0b01000100010001,
    0b00000100010001,
    0b00010010010000,
    0b00100100010001,
    0b00001001000001,
    0b10000100000001,
    0b00001001000100,
    0b00001001000000,
    0b01000100000001,
    0b00000100000001,
    0b00000010010000,
    0b00100100000001,
    0b00000100100100,
    0b10000010010001,
    0b10010010010001,
    0b10000100100000,
    0b01000010010001,
    0b00000010010001,
    0b00010010010001,
    0b00100010010001,
    0b01001000010001,
    0b10000000010001,
    0b10010000010001,
    0b10001000010001,
    0b01000000010001,
    0b00001000010001,
    0b00010000010001,
    0b00100000010001,
    0b01000100000010,
    0b00000100000010,
    0b10000100010010,
    0b00100100000010,
    0b01000100010010,
    0b00000100010010,
    0b01000000100010,
    0b00100100010010,
    0b10000100000010,
    0b10000100000100,
    0b00001001001001,
    0b00001001000010,
    0b01000100000100,
    0b00000100000100,
    0b00010000100010,
    0b00100100000100,
    0b00000100100010,
    0b10000010010010,
    0b10010010010010,
    0b00001000100010,
    0b01000010010010,
    0b00000010010010,
    0b00010010010010,
    0b00100010010010,
    0b01001000010010,
    0b10000000010010,
    0b10010000010010,
    0b10001000010010,
    0b01000000010010,
    0b00001000010010,
    0b00010000010010,
    0b00100000010010,
];

const QUATTUORDECUPLE_SIZE: usize = 14;
const START_PT: usize = 112 - QUATTUORDECUPLE_SIZE;

impl EFM {
    pub fn new() -> Self {
        let mut decode_mappings: [u16; 0x4000] = [0xFFFF; 0x4000];
        let mut encode_mappings: [u16; 256] = [0xFFFF; 256];

        for (decoded, &encoded) in EFM_TABLE_RAW.iter().enumerate() {
            decode_mappings[encoded as usize] = decoded as u16;
            encode_mappings[decoded] = encoded;
        }
        EFM {
            decode_mappings,
            encode_mappings,
        }
    }

    /// Decodes a 128-bit encoded value into a vector of bytes using a predefined mapping table.
    ///
    /// This method interprets the provided `encoded_value` as a sequence of 8 groups of 14 bits (quattuordecuple),
    /// looks up each 14-bit group in the `decode_mappings` table, and constructs a resulting vector of 8 bytes.
    /// If any 14-bit group results in an invalid mapping (determined by the presence of a sentinel value `0xFFFF`),
    /// the method returns `None`.
    ///
    /// # Parameters
    /// - `encoded_value: u128`: A 128-bit value encoded as a series of 14-bit groups to be decoded.
    ///
    /// # Returns
    /// - `Option<Vec<u8>>`:
    ///   - `Some(Vec<u8>)`: A vector of decoded bytes if all 14-bit groups are successfully resolved.
    ///   - `None`: If at least one 14-bit group in the input cannot be decoded (due to an invalid mapping).
    ///
    /// # Notes
    /// - The `decode_mappings` table is used to convert 14-bit groups into decoded bytes.
    /// - `QUATTUORDECUPLE_SIZE` is assumed to represent the size (in bits) of each 14-bit group.
    /// - `START_PT` is a constant defining the starting bit position for the first group in the `encoded_value`.
    /// - The sentinel value `0xFFFF` in the `decode_mappings` table is used to indicate an invalid encoded group.
    ///
    /// # Example
    /// ```rust
    /// let decoder = MyDecoder {
    ///     decode_mappings: [0xABCD, 0xFFFF, 0x1234, ...], // Your decode mapping table here
    /// };
    ///
    /// let encoded_value = 0x1234567890ABCDEF1234567890ABCDEFu128;
    /// let decoded_result = decoder.decode_quattuordecuple(encoded_value);
    ///
    /// match decoded_result {
    ///     Some(bytes) => println!("Decoded bytes: {:?}", bytes),
    ///     None => println!("Failed to decode the provided value due to invalid mapping."),
    /// }
    /// ```
    fn decode_quattuordecuple(&self, encoded_value: u128) -> Option<Vec<u8>> {
        let mut result: Vec<u8> = Vec::with_capacity(4); // Pre-allocate exact capacity

        for j in 0..8 {
            let decoded = self.decode_mappings
                [((encoded_value >> START_PT - j * QUATTUORDECUPLE_SIZE) & 0x3fff) as usize];

            // Skip invalid encodings
            if decoded == 0xFFFF {
                return None;
            }

            result.push((decoded & 0xFF) as u8);
        }
        Some(result)
    }

    /// Decodes a byte slice into a vector of bytes using the specified decoding logic.
    ///
    /// This method processes the input byte slice in fixed-size chunks of `QUATTUORDECUPLE_SIZE`
    /// and performs decoding on each chunk. The decoding logic is defined in the
    /// `decode_quattuordecuple` function of the struct's implementation. If decoding fails for
    /// any chunk, the function will return `None`. Otherwise, it will return a `Vec<u8>`
    /// containing the fully decoded result.
    ///
    /// ### Parameters
    /// - `value`: A slice of bytes (`&[u8]`) to be decoded. The slice is expected
    ///            to consist of groups of `QUATTUORDECUPLE_SIZE` bytes for proper decoding.
    ///
    /// ### Returns
    /// - `Option<Vec<u8>>`:
    ///    - `Some(Vec<u8>)` if decoding succeeds for all chunks in the input slice, where the
    ///       `Vec<u8>` contains the fully decoded data.
    ///    - `None` if any chunk fails to decode.
    ///
    /// ### Behavior
    /// - The function processes the input using the `chunks_exact` iterator to divide
    ///   the slice into fixed-size chunks of length `QUATTUORDECUPLE_SIZE`.
    /// - Each chunk is converted into a 128-bit big-endian integer (`u128`) with the first
    ///   two bytes padded with zeros.
    /// - The `decode_quattuordecuple` function is called on the 128-bit value to perform
    ///   the actual decoding.
    /// - If `decode_quattuordecuple` returns `None` for any chunk, the entire function
    ///   returns `None`. Otherwise, the decoded bytes from all chunks are appended to the
    ///   final result vector, which is returned wrapped in `Some`.
    ///
    /// ### Examples
    /// ```rust
    /// let decoder = YourDecoder::new();
    /// let encoded_data: Vec<u8> = vec![0x00, 0x01, 0x02, /* ... */];
    /// if let Some(decoded_data) = decoder.decode(&encoded_data) {
    ///     println!("Decoded data: {:?}", decoded_data);
    /// } else {
    ///     println!("Failed to decode data.");
    /// }
    /// ```
    ///
    /// ### Notes
    /// - Ensure that the input slice `value` has a length that is a multiple of `QUATTUORDECUPLE_SIZE`,
    ///   since `chunks_exact` will ignore any remainder bytes that do not fit into a complete chunk.
    /// - The effectiveness of the decoding relies on the correctness of the `decode_quattuordecuple`
    ///   function and the expected format of the input data.
    pub fn decode(&self, value: &[u8]) -> Option<Vec<u8>> {
        let mut result: Vec<u8> = Vec::new();
        // Process chunks more efficiently using exact_chunks
        for chunk in value.chunks_exact(QUATTUORDECUPLE_SIZE) {
            let final_value = u128::from_be_bytes([
                0, 0, chunk[0], chunk[1], chunk[2], chunk[3], chunk[4], chunk[5], chunk[6],
                chunk[7], chunk[8], chunk[9], chunk[10], chunk[11], chunk[12], chunk[13],
            ]);

            if let Some(res) = self.decode_quattuordecuple(final_value) {
                result.extend(res);
            } else {
                return None;
            }
        }
        Some(result)
    }

    /// Encodes a slice of bytes into a 128-bit unsigned integer (u128) using a quattuordecuple (base-14) encoding scheme.
    ///
    /// # Parameters
    /// - `decoded_value`: A reference to a slice of `u8` values representing the input data to be encoded. The slice must have a length of at least 8 elements.
    ///
    /// # Returns
    /// - A `u128` representing the encoded value in quattuordecuple (base-14) format.
    ///
    /// # Behavior
    /// - This function performs base-14 encoding by mapping each byte of the input slice (`decoded_value`) to a predefined encoding value
    ///   (`encode_mappings`) and then shifting and combining these values into a single 128-bit unsigned integer (`u128`).
    /// - The loop iterates through the first 8 elements of the input slice, calculating bit-shift amounts (`shift_amount_high`) to correctly position
    ///   each encoded value within the resulting `u128`.
    ///
    /// # Requirements
    /// - `self.encode_mappings` is assumed to be a lookup array or slice where each input byte value directly maps to a corresponding encoded value.
    /// - Constants such as `START_PT` and `QUATTUORDECUPLE_SIZE` must be defined in the surrounding context to correctly calculate the bit shift.
    ///
    /// # Example
    /// ```
    /// // Example encode mappings and constants
    /// const START_PT: u128 = 112;
    /// const QUATTUORDECUPLE_SIZE: u128 = 14;
    /// let encode_mappings: [u8; 256] = [0; 256]; // Example mappings
    /// let decoded_value: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
    ///
    /// let result = example_struct.encode_quattuordecuple(&decoded_value);
    /// ```
    ///
    /// # Notes
    /// - Ensure the input slice length is at least 8 to avoid out-of-bounds panics.
    /// - The behavior of this function depends on the correctness of `START_PT`, `QUATTUORDECUPLE_SIZE`, and `self.encode_mappings`.
    fn encode_quattuordecuple(&self, decoded_value: &[u8]) -> u128 {
        let mut acc: u128 = 0;

        for i in 0..8 {
            let shift_amount_high = START_PT - i * QUATTUORDECUPLE_SIZE;
            acc |= (self.encode_mappings[decoded_value[i] as usize] as u128) << shift_amount_high;
        }
        acc
    }

    /// Encodes the input byte slice using a custom encoding scheme and returns the resulting encoded byte vector.
    ///
    /// This function processes the input in fixed-size chunks of 8 bytes (`chunks_exact(8)`) and applies
    /// a custom encoding method (`encode_quattuordecuple`) to each chunk. The encoded result for each chunk
    /// is then serialized into bytes and appended to the output vector.
    ///
    /// # Parameters
    /// * `value`: A reference to the input byte slice (`&[u8]`) that needs to be encoded.
    ///
    /// # Returns
    /// * A `Vec<u8>` containing the encoded byte sequence.
    ///
    /// # Behavior
    /// - The function calculates a `result` vector with a pre-allocated capacity, determined
    ///   by `value.len()` multiplied by `QUATTUORDECUPLE_SIZE` to optimize memory allocation.
    /// - It processes the input slice in chunks of 8 bytes and ensures that all chunks are
    ///   treated as fixed sizes (`chunks_exact(8)`).
    /// - For each chunk, the encoding is computed using the `encode_quattuordecuple` method.
    /// - Only the relevant portion (skipping the first 2 most significant bytes)
    ///   of the resulting encoded value is converted to bytes (using `to_be_bytes`) and added
    ///   to the output vector.
    ///
    /// # Panics
    /// This function does not panic under normal circumstances. However, improper implementations
    /// of the `encode_quattuordecuple` method or invalid input data may cause unexpected behavior.
    ///
    /// # Example
    /// ```rust
    /// // Assuming `my_encoder` is an instance of the struct that implements this method
    /// let input_data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    /// let encoded = my_encoder.encode(&input_data);
    /// println!("{:?}", encoded);
    /// ```
    ///
    /// # Notes
    /// - The function relies on the existence of the `QUATTUORDECUPLE_SIZE` constant and the `encode_quattuordecuple` method.
    /// - The name `encode_quattuordecuple` suggests encoding into a quattuordecuple (a group of 14), but
    ///   the exact nature of this encoding depends on the implementation of the `encode_quattuordecuple` method.
    pub fn encode(&self, value: &[u8]) -> Vec<u8> {
        let mut result = Vec::with_capacity(value.len() * QUATTUORDECUPLE_SIZE);

        for chunk in value.chunks_exact(8) {
            let acc = self.encode_quattuordecuple(chunk);
            // Convert to bytes using to_be_bytes and extend efficiently
            result.extend_from_slice(&acc.to_be_bytes()[2..]);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn real() {
        let efm = EFM::new();
        let val: Vec<u8> = Vec::from("..hello world!..");
        let encoded = efm.encode(&val);
        let decoded = efm.decode(&encoded).unwrap();
        assert_eq!(decoded, val);
    }
}
