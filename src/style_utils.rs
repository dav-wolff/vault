pub fn classes<const COUNT: usize>(classes: [&str; COUNT]) -> String {
	classes.join(" ")
}
