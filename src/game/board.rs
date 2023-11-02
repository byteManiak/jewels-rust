pub struct Board {
}

impl Board {
    fn new_game(&mut self) {
    }

    pub(super) fn load_game(&mut self) {
        self.new_game();
    }

    pub(super) fn update(&mut self) -> bool {
        false
    }
}