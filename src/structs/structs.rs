#[derive(Debug)]
struct Team {
    team_name: String,
    id: i32,
    member_number: u8,
    mgr: String,
    is_tiktok: bool,
}

impl Team {
    fn increase_number(&mut self, num: u8) {
        self.member_number += num; 
    }
}



fn main() {
    let mut team1 = Team {
        team_name: String::from("Velox"),
        id: 1,
        member_number: 11,
        mgr: String::from("Nobody"),
        is_tiktok: false,
    };

    let name = team1.team_name;
    println!("Team name is {}", name);
    team1.mgr = "Tom hanks".to_string();

    let mut team2 = build_team("Joe Bide".to_string(), "Tiger".to_string());
    println!("{:#?}", team2);

    team2.increase_number(20);
    println!("{:#?}", team2);
}

fn build_team(mgr: String, team_name: String) -> Team {
    Team {
        team_name,
        id: 999,
        member_number: 0,
        mgr,
        is_tiktok: false
    }
}