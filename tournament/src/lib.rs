#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Team<'a> {
    name: &'a str,
    win: u16,
    draw: u16,
    loss: u16,
    points: u16,
}

impl<'a> Team<'a> {
    fn new(name: &'a str, win: u16, loss: u16, draw: u16) -> Self {
        Team {
            name,
            win,
            draw,
            loss,
            points: 0,
        }
    }

    fn add(&mut self, team: Team) {
        self.win += team.win;
        self.draw += team.draw;
        self.loss += team.loss;
    }

    fn games_played(&self) -> u16 {
        self.win + self.draw + self.loss
    }

    fn points(&mut self) {
        self.points = self.win * 3 + self.draw;
    }

    fn to_string(&self) -> String {
               //"Team                           | MP |  W |  D |  L |  P"
        format!("{:<31}|{:>3} |{:>3} |{:>3} |{:>3} |{:>3}",
                self.name,
                self.games_played(),
                self.win,
                self.draw,
                self.loss,
                self.points)
    }
}

#[derive(Debug)]
struct Bracket<'a> {
    teams: Vec<Team<'a>>,
}

impl<'a> Bracket<'a> {
    fn new() -> Self {
        Bracket {
            teams: vec![],
        }
    }

    fn add(&mut self, team: Team<'a>) {
        let mut found = (false, 0);
        for (i, t) in self.teams.iter().enumerate() {
            if t.name == team.name { found = (true, i); }
        }

        match found.0 {
            true => { self.teams[found.1].add(team) },
            false => { self.teams.push(team) },
        }
    }

    fn build(&mut self, s: &'a str) {
        let pre_parsed = s.split_terminator("\n").collect::<Vec<_>>();
        for game in pre_parsed.iter() {
            let t1;
            let t2;
            let parsed_game = game.split(";").collect::<Vec<_>>();
            if &parsed_game[2] == &"win" {
                t1 = Team::new(&parsed_game[0], 1, 0, 0);
                t2 = Team::new(&parsed_game[1], 0, 1, 0);
            } else if &parsed_game[2] == &"loss" {
                t1 = Team::new(&parsed_game[0], 0, 1, 0);
                t2 = Team::new(&parsed_game[1], 1, 0, 0);
            } else {
                t1 = Team::new(&parsed_game[0], 0, 0, 1);
                t2 = Team::new(&parsed_game[1], 0, 0, 1);
            }
            self.add(t1);
            self.add(t2);
        }
        for team in self.teams.iter_mut() {
            team.points();
        }
        dbg!("{:?}", &self.teams);
    }

    fn sorted_results(&mut self) -> String {
        let mut sort_res = vec![];
        sort_res.push("Team                           | MP |  W |  D |  L |  P".to_string());
        self.teams.sort_by_key(|t| t.name);
        self.teams.reverse();
        self.teams.sort_by_key(|t| t.points);
        self.teams.reverse();
        for team in self.teams.iter() {
            sort_res.push(team.to_string());
        }
        let res_str = sort_res.join("\n");
        dbg!("{}", &res_str);
        res_str
    }
}


pub fn tally(match_results: &str) -> String {
    let mut bracket = Bracket::new();
    bracket.build(match_results);
    bracket.sorted_results().to_string()
}
