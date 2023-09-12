
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Text(pub Vec<u8>);

impl Text {
    pub fn new(t: Vec<u8>) -> Self {
        Text(t)
    }
    pub fn from_string(s: String) -> Self {
        Self::new(s.as_bytes().to_vec())
    }

    pub fn from_str(s: &str) -> Self {
        Self::new(s.as_bytes().to_vec())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, i: usize) -> Option<&u8> {
        self.0.get(i)
    }

    pub fn from_slice(slice: &[u8]) -> Self {
        Text::new(slice.to_vec())
    }

    // Returns the index of the first character for which cond returns true
    // If no characters return true for cond, returns len
    // Skips some characters at the start of text
    pub fn find_first<F>(&self, cond: F, skip: usize) -> usize 
    where F: Fn(&u8) -> bool {
        for (i, c) in self.0.iter().enumerate().skip(skip) {
            if cond(c) { return i }
        }
        return self.len()
    }
}

impl ToString for Text {
    fn to_string(&self) -> String {
        // Convert the internal Vec<u8> to a UTF-8 encoded string
        String::from_utf8_lossy(&self.0).into_owned()
    }
}