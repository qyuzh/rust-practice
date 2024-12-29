pub fn rank_teams(votes: Vec<String>) -> String {
    struct Team {
        name: char,
        rank: [i32; 26],
    }

    let mut teams = std::collections::HashMap::new();
    for v in votes {
        for (i, c) in v.chars().enumerate() {
            let team = teams.entry(c).or_insert(Team {
                name: c,
                rank: [0; 26],
            });
            team.rank[i] += 1;
        }
    }

    let mut team_vec: Vec<Team> = teams.into_values().collect();
    team_vec.sort_unstable_by(|a, b| {
        for i in 0..26 {
            if a.rank[i] != b.rank[i] {
                return b.rank[i].cmp(&a.rank[i]);
            }
        }
        a.name.cmp(&b.name)
    });

    team_vec.iter().map(|t| t.name).collect()
}

pub fn rank_teams2(mut votes: Vec<String>) -> String {
    if votes.is_empty() {
        return String::new();
    }
    let mut rank = vec![vec![0; 26]; 26];
    for v in votes.iter() {
        for (i, c) in v.chars().enumerate() {
            rank[c as usize - 'A' as usize][i] -= 1; // reverse the order
        }
    }
    let mut s = unsafe { votes[0].as_bytes_mut() };
    s.sort_by_key(|&c| (&rank[c as usize - 'A' as usize], c)); // sort by rank
    std::mem::take(&mut votes[0])
}
