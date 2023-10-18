fn main() {}

struct Game {
    rolls: Vec<usize>,
    current_roll: usize,
}

impl Game {
    fn new() -> Game {
        Game {
            rolls: vec![0; 21],
            current_roll: 0,
        }
    }
    fn roll(&mut self, pins: usize) {
        self.rolls[self.current_roll] = pins;
        self.current_roll += 1;
    }

    fn score(&mut self) -> usize {
        let mut score: usize = 0;
        let mut first_in_frame = 0;

        for _ in 0..10 {
            if self.is_strike(first_in_frame) { 
                score += 10 + self.next_two_balls_for_strike(first_in_frame);
                first_in_frame += 1;
            } else if self.is_spare(first_in_frame) {
                score += 10 + self.next_ball_for_spare(first_in_frame);
                first_in_frame += 2;
            } else {
                score += self.two_balls_in_frame(first_in_frame);
                first_in_frame += 2;
            }                    
        }

        score
    }

    fn two_balls_in_frame(&mut self, first_in_frame: usize) -> usize {
        self.rolls[first_in_frame] + self.rolls[first_in_frame+1]
    }

    fn next_ball_for_spare(&mut self, first_in_frame: usize) -> usize {
        self.rolls[first_in_frame+2]
    }

    fn next_two_balls_for_strike(&mut self, first_in_frame: usize) -> usize {
        self.rolls[first_in_frame+1] + self.rolls[first_in_frame+2]
    }

    fn is_strike(&mut self, first_in_frame: usize) -> bool {
        self.rolls[first_in_frame] == 10
    }

    fn is_spare(&mut self, first_in_frame: usize) -> bool {
        self.rolls[first_in_frame] + self.rolls[first_in_frame+1] == 10
    }
}



#[cfg(test)]
mod tests {

    use super::*;

    struct TestContext{
        game: Game,
    }

    impl TestContext {
        fn new() -> TestContext {
            TestContext {
                game : Game::new(),
            }
        }

        fn roll_many(&mut self, roll: i32, pins: usize) {
            for _ in 0..roll {
                self.game.roll(pins);
            }
        }
    
        fn roll_spare(&mut self) {
            self.game.roll(5);
            self.game.roll(5);
        }
        
        fn roll_strike(&mut self) {
            self.game.roll(10)
        }
    } 



    #[test]
    fn gutter_game() {
        let mut test_context = TestContext::new();

        test_context.roll_many(20, 0);

        assert_eq!(0, test_context.game.score());
    }

    #[test]
    fn all_ones() {
        let mut test_context = TestContext::new();
 
        test_context.roll_many(20, 1);

        assert_eq!(20, test_context.game.score());
    }

    #[test]
    fn one_spare() {
        let mut test_context = TestContext::new();
        
        test_context.roll_spare();
        test_context.game.roll(3);
        test_context.roll_many(17, 0);

        assert_eq!(16, test_context.game.score());
    }


    #[test]
    fn one_strike() {
        let mut test_context = TestContext::new();

        test_context.roll_strike();
        test_context.game.roll(3);
        test_context.game.roll(4);
        test_context.roll_many(16, 0);

        assert_eq!(24, test_context.game.score());
    }

    #[test]
    fn perfect_game() {
        let mut test_context = TestContext::new();

        test_context.roll_many(12, 10);

        assert_eq!(300, test_context.game.score());
    }

    
}
