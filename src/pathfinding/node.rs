use hexx::Hex;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Node {
    pub position: Hex,
    pub f_score: i32,
    pub g_score: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.f_score.cmp(&other.f_score)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
