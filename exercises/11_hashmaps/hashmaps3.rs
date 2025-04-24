use std::collections::HashMap;

#[derive(Default)]
struct TeamScores {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: &str) -> HashMap<&str, TeamScores> {
    let mut scores = HashMap::<&str, TeamScores>::new();

    for line in results.lines() {
        let mut parts = line.split(',');
        let team_1_name = parts.next().unwrap();
        let team_2_name = parts.next().unwrap();
        let team_1_score: u8 = parts.next().unwrap().parse().unwrap();
        let team_2_score: u8 = parts.next().unwrap().parse().unwrap();

        // 更新第一支球队的数据
        let team_1 = scores.entry(team_1_name).or_default();
        team_1.goals_scored += team_1_score;
        team_1.goals_conceded += team_2_score;

        // 更新第二支球队的数据
        let team_2 = scores.entry(team_2_name).or_default();
        team_2.goals_scored += team_2_score;
        team_2.goals_conceded += team_1_score;
    }

    scores
}

fn main() {
    // 可以在这里进行实验性代码
}

#[cfg(test)]
mod tests {
    use super::*;

    const RESULTS: &str = "England,France,4,2
France,Italy,3,1
Poland,Spain,2,0
Germany,England,2,1
England,Spain,1,0";

    #[test]
    fn build_scores() {
        let scores = build_scores_table(RESULTS);
        assert!(["England", "France", "Germany", "Italy", "Poland", "Spain"]
            .into_iter()
            .all(|team_name| scores.contains_key(team_name)));
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 6);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 3);
    }
}