#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
enum Point {
    P0, P15, P30, P40, PAdv, PWin
}

#[derive(PartialEq)]
#[derive(Debug)]
enum GameResult {
    P1Win, P2Win, OnGoing
}

#[derive(PartialEq)]
#[derive(Debug)]
enum GameScore {
    P1, P2
}

#[derive(PartialEq)]
#[derive(Debug)]
struct Game {
    player_1: Point,
    player_2: Point,
    result: GameResult
}

impl Game {
    fn score(&mut self, update: &GameScore) {
        if self.result != GameResult::OnGoing {
            return;
        }

        match update {
            GameScore::P1 => {
                let (p1, p2) = Game::adv_score(self.player_1, self.player_2);
                self.player_1 = p1;
                self.player_2 = p2;
            },
            GameScore::P2 => {
                let (p2, p1) = Game::adv_score(self.player_2, self.player_1);
                self.player_1 = p1;
                self.player_2 = p2;
            }
        }

        self.check_winner();
    }

    fn check_winner(&mut self) {
        self.result = match (&self.player_1, &self.player_2) {
            (Point::PWin, _) => GameResult::P1Win,
            (_, Point::PWin) => GameResult::P2Win,
            _ => GameResult::OnGoing
        }
    }

    fn adv_score(player: Point, opponent: Point) -> (Point, Point) {
        match (&player, &opponent) {
            (Point::PWin, _) => {
                (Point::PWin, Point::P0)
            },
            (Point::PAdv, _) => {
                (Point::PWin, Point::P0)
            },
            (Point::P40, Point::PAdv) => {
                (Point::P40, Point::P40)
            },
            (Point::P40, Point::P40) => {
                (Point::PAdv, opponent)
            },
            (Point::P40, _) => {
                (Point::PWin, Point::P0)
            },
            (Point::P30, _) => {
                (Point::P40, opponent)
            },
            (Point::P15, _) => {
                (Point::P30, opponent)
            },
            (Point::P0, _) => {
                (Point::P15, opponent)
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tennis_test_p1() {
        generic_tennis_test_p1(Point::P0, Point::P0, Point::P15, Point::P0, false);
        generic_tennis_test_p1(Point::P15, Point::P0, Point::P30, Point::P0, false);
        generic_tennis_test_p1(Point::P30, Point::P0, Point::P40, Point::P0, false);
        generic_tennis_test_p1(Point::P40, Point::P0, Point::PWin, Point::P0, true);
        generic_tennis_test_p1(Point::P15, Point::P15, Point::P30, Point::P15, false);
        generic_tennis_test_p1(Point::P30, Point::P15, Point::P40, Point::P15, false);
        generic_tennis_test_p1(Point::P40, Point::P15, Point::PWin, Point::P0, true);
        generic_tennis_test_p1(Point::P15, Point::P30, Point::P30, Point::P30, false);
        generic_tennis_test_p1(Point::P30, Point::P30, Point::P40, Point::P30, false);
        generic_tennis_test_p1(Point::P40, Point::P30, Point::PWin, Point::P0, true);
        generic_tennis_test_p1(Point::P15, Point::P40, Point::P30, Point::P40, false);
        generic_tennis_test_p1(Point::P30, Point::P40, Point::P40, Point::P40, false);
        generic_tennis_test_p1(Point::P40, Point::P40, Point::PAdv, Point::P40, false);
        generic_tennis_test_p1(Point::P40, Point::PAdv, Point::P40, Point::P40, false);
    }

    #[test]
    fn tennis_test_p2() {
        generic_tennis_test_p2(Point::P0, Point::P0, Point::P15, Point::P0, false);
        generic_tennis_test_p2(Point::P15, Point::P0, Point::P30, Point::P0, false);
        generic_tennis_test_p2(Point::P30, Point::P0, Point::P40, Point::P0, false);
        generic_tennis_test_p2(Point::P40, Point::P0, Point::PWin, Point::P0, true);
        generic_tennis_test_p2(Point::P15, Point::P15, Point::P30, Point::P15, false);
        generic_tennis_test_p2(Point::P30, Point::P15, Point::P40, Point::P15, false);
        generic_tennis_test_p2(Point::P40, Point::P15, Point::PWin, Point::P0, true);
        generic_tennis_test_p2(Point::P15, Point::P30, Point::P30, Point::P30, false);
        generic_tennis_test_p2(Point::P30, Point::P30, Point::P40, Point::P30, false);
        generic_tennis_test_p2(Point::P40, Point::P30, Point::PWin, Point::P0, true);
        generic_tennis_test_p2(Point::P15, Point::P40, Point::P30, Point::P40, false);
        generic_tennis_test_p2(Point::P30, Point::P40, Point::P40, Point::P40, false);
        generic_tennis_test_p2(Point::P40, Point::P40, Point::PAdv, Point::P40, false);
        generic_tennis_test_p2(Point::P40, Point::PAdv, Point::P40, Point::P40, false);
    }

    fn generic_tennis_test_p1(p1: Point, p2: Point, p1_result: Point, p2_result: Point, is_win: bool) {
        generic_tennis_test(p1, p2, GameScore::P1, p1_result, p2_result, if is_win { GameResult::P1Win } else { GameResult::OnGoing})
    }

    fn generic_tennis_test_p2(p1: Point, p2: Point, p1_result: Point, p2_result: Point, is_win: bool) {
        generic_tennis_test(p2, p1, GameScore::P2, p2_result, p1_result, if is_win { GameResult::P2Win } else { GameResult::OnGoing})
    }

    fn generic_tennis_test(p1: Point, p2: Point, update: GameScore, p1_result: Point, p2_result: Point, game_result: GameResult) {
        let mut actual_game = Game { player_1: p1, player_2: p2, result: GameResult::OnGoing };
        actual_game.score(&update);
        let expected_game = Game { player_1: p1_result, player_2: p2_result, result: game_result };
        assert_eq!(&actual_game, &expected_game);
    }
}