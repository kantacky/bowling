pub fn answer(s: &str) -> i32 {
    let mut score = 0;
    let t = s.replace("||", "|");
    let v: Vec<&str> = t.split('|').collect();
    for i in 0..10 {
        let frame_score = get_frame_score(v[i]);
        score += frame_score.0;
        if frame_score.1 >= 0 {
            score += frame_score.1;
        }
        if frame_score.0 != 10 && frame_score.0 + frame_score.1 != 10 {
            continue;
        }
        let next_frame_score = get_frame_score(v[i + 1]);
        score += next_frame_score.0;
        if frame_score.0 != 10 {
            continue;
        }
        if next_frame_score.1 >= 0 {
            score += next_frame_score.1;
            continue;
        }
        let frame_after_next_frame_score = get_frame_score(v[i + 2]);
        score += frame_after_next_frame_score.0;
    }
    return score;
}

pub fn get_frame_score(s: &str) -> (i32, i32) {
    let mut scores: (i32, i32) = (-1, -1);
    for c in s.chars() {
        let score = get_score(c);
        if score < 0 {
            scores.1 = 10 - scores.0;
            continue;
        }
        if scores.0 >= 0 {
            scores.1 = score;
            continue;
        }
        scores.0 = score;
    }
    return scores;
}

pub fn get_score(c: char) -> i32 {
    if c == 'X' {
        return 10;
    }
    if c == '-' {
        return 0;
    }
    if c == '/' {
        return -1;
    }
    return c as i32 - 48;
}
