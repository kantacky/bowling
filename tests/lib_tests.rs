use bowling;

#[test]
fn bowling_answer_test_300() {
    assert_eq!(300, bowling::answer("X|X|X|X|X|X|X|X|X|X||XX"));
}

#[test]
fn bowling_answer_tes_t90() {
    assert_eq!(90, bowling::answer("9-|9-|9-|9-|9-|9-|9-|9-|9-|9-||"));
}

#[test]
fn bowling_answer_test_150() {
    assert_eq!(150, bowling::answer("5/|5/|5/|5/|5/|5/|5/|5/|5/|5/||5"));
}

#[test]
fn bowling_answer_test_167() {
    assert_eq!(167, bowling::answer("X|7/|9-|X|-8|8/|-6|X|X|X||81"));
}

#[test]
fn bowling_get_score_test_10() {
    assert_eq!(10, bowling::get_score('X'));
}

#[test]
fn bowling_get_score_test_miss() {
    assert_eq!(0, bowling::get_score('-'));
}

#[test]
fn bowling_get_score_test_slash() {
    assert_eq!(-1, bowling::get_score('/'));
}

#[test]
fn bowling_get_score_test_number() {
    assert_eq!(1, bowling::get_score('1'));
}

#[test]
fn bowling_get_frame_score_test_10_0() {
    assert_eq!((10, -1), bowling::get_frame_score("X"));
}

#[test]
fn bowling_get_frame_score_test_10_10() {
    assert_eq!((10, 10), bowling::get_frame_score("XX"));
}

#[test]
fn bowling_get_frame_score_test_number_slash() {
    assert_eq!((1, 9), bowling::get_frame_score("1/"));
}

#[test]
fn bowling_get_frame_score_test_number_miss() {
    assert_eq!((1, 0), bowling::get_frame_score("1-"));
}

#[test]
fn bowling_get_frame_score_test_miss_number() {
    assert_eq!((0, 1), bowling::get_frame_score("-1"));
}

#[test]
fn bowling_get_frame_score_test_miss_miss() {
    assert_eq!((0, 0), bowling::get_frame_score("--"));
}

#[test]
fn bowling_get_frame_score_test_miss_slash() {
    assert_eq!((0, 10), bowling::get_frame_score("-/"));
}
