pub const FINGERPRINT_SIZE: usize = 2;
pub const BUCKET_SIZE: usize = 8;
const EMPTY_FINGERPRINT_DATA: [u8; FINGERPRINT_SIZE] = [100; FINGERPRINT_SIZE];

// Fingerprint Size is 1 byte so lets remove the Vec
#[derive(PartialEq, Copy, Clone, Hash)]
pub struct Fingerprint {
  data: [u8; FINGERPRINT_SIZE]
}

impl Fingerprint {
  /// Attempts to create a new Fingerprint based on the given
  /// number. If the created Fingerprint would be equal to the
  /// empty Fingerprint, None is returned.
  pub fn from_usize(mut n: usize) -> Option<Fingerprint> {
    let mut data = [0; FINGERPRINT_SIZE];
    for i in 0..FINGERPRINT_SIZE {
      data[i] = (n & 0xff) as u8;
      n >>= 8;
    }
    let result = Fingerprint{ data: data };
    if result.is_empty() { None }
    else { Some(result) }
  }

  /// Returns the empty Fingerprint.
  pub fn empty() -> Fingerprint {
    Fingerprint { data: EMPTY_FINGERPRINT_DATA }
  }

  /// Checks if this is the empty Fingerprint.
  pub fn is_empty(&self) -> bool {
    self.data == EMPTY_FINGERPRINT_DATA
  }
}

/// Manages BUCKET_SIZE fingerprints at most.
#[derive(Clone)]
pub struct Bucket {
    pub buffer: [Fingerprint; BUCKET_SIZE]
}

impl Bucket {
    /// Creates a new bucket with a pre-allocated buffer.
    pub fn new() -> Bucket {
        Bucket {
            buffer: [Fingerprint::empty(); BUCKET_SIZE]
        }
    }

    /// Inserts the fingerprint into the buffer if the buffer is not full. This
    /// operation is O(1).
    pub fn insert(&mut self, fp: Fingerprint) -> bool {
      for entry in self.buffer.iter_mut() {
        if entry.is_empty() {
          *entry = fp;
          return true;
        }
      }
      false
    }

    /// Deletes the given fingerprint from the bucket. This operation is O(1).
    pub fn delete(&mut self, fp: Fingerprint) -> bool {
        match self.get_fingerprint_index(fp) {
            Some(index) => { self.buffer[index] = Fingerprint::empty(); true }
            None => false
        }
    }

    /// Returns the index of the given fingerprint, if its found. O(1)
    pub fn get_fingerprint_index(&mut self, fp: Fingerprint) -> Option<usize> {
        self.buffer.iter().position(|e| *e == fp)
    }
}
