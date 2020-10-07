#[test]
fn pvykhas42a44ldrecbb51ip() {
	let mut parse = crate::parse("{a,b}");
	let mut expected = vec!["a", "b"];
	parse.sort();
	expected.sort();

	assert_eq!(parse, expected);
}

#[test]
fn y0itnt01ead1() {
	let mut parse = crate::parse("c{a,b}");
	let mut expected = vec!["ca", "cb"];
	parse.sort();
	expected.sort();

	assert_eq!(parse, expected);
}

#[test]
fn g38rqc4ewifen() {
	let mut parse = crate::parse("c{a, b}d");
	let mut expected = vec!["cad", "cbd"];
	parse.sort();
	expected.sort();

	assert_eq!(parse, expected);
}

#[test]
fn gxl3yvjnls33deyhr8g14el() {
	let mut parse = crate::parse("c{a ,b}d");
	let mut expected = vec!["cad", "cbd"];
	parse.sort();
	expected.sort();

	assert_eq!(parse, expected);
}

#[test]
fn utqe6f5i7tlgip44mpl() {
	let mut parse = crate::parse("c{a,b}d");
	let mut expected = vec!["cad", "cbd"];
	parse.sort();
	expected.sort();

	assert_eq!(parse, expected);
}

#[test]
fn wzn8fsszr4js() {
	let mut parse = crate::parse("{a,b,c}");
	let mut expected = vec!["a", "b", "c"];
	parse.sort();
	expected.sort();

	assert_eq!(parse, expected);
}

#[test]
fn wfjp6y4g2ix35xp0zcoljry() {
	let mut parse = crate::parse("{a,}");
	let mut expected = vec!["a", ""];
	parse.sort();
	expected.sort();

	assert_eq!(parse, expected);
}

#[test]
fn ro0zhpirqog2() {
	let mut parse = crate::parse("c{a,}d");
	let mut expected = vec!["cad", "cd"];
	parse.sort();
	expected.sort();

	assert_eq!(parse, expected);
}

#[test]
fn oh748141tisb6ajkcxgy7u() {
	let mut parse = crate::parse("c{}d");
	let mut expected = vec!["cd"];
	parse.sort();
	expected.sort();

	assert_eq!(parse, expected);
}

#[test]
fn bfqrw3dw0h6wy02j() {
	let mut parse = crate::parse("c{ }d");
	let mut expected = vec!["cd"];
	parse.sort();
	expected.sort();

	assert_eq!(parse, expected);
}

#[test]
fn kk73nbafsm() {
	let mut parse = crate::parse("c{a}d");
	let mut expected = vec!["cad"];
	parse.sort();
	expected.sort();

	assert_eq!(parse, expected);
}

#[test]
fn f67vo8aowe3alwx5v02sj6() {
	let mut parse = crate::parse("c{,}d");
	let mut expected = vec!["cd", "cd"];
	parse.sort();
	expected.sort();

	assert_eq!(parse, expected);
}

#[test]
fn wsvf53r2v6vprep5ix2yffu() {
	let mut parse = crate::parse("c{a,b}d{e");
	let mut expected = vec!["cad{e", "cbd{e"];
	parse.sort();
	expected.sort();

	assert_eq!(parse, expected);
}

#[test]
fn b1xbpg3ariguy74h() {
	let mut parse = crate::parse("c{a,b}d}e");
	let mut expected = vec!["cad}e", "cbd}e"];
	parse.sort();
	expected.sort();

	assert_eq!(parse, expected);
}

#[test]
fn eohhfeifgj() {
	let mut parse = crate::parse("c{a,b}d,e");
	let mut expected = vec!["cad,e", "cbd,e"];
	parse.sort();
	expected.sort();

	assert_eq!(parse, expected);
}

#[test]
fn s0oxd2l7uthfylivjtv() {
	let mut parse = crate::parse("e{c{a,b}d");
	let mut expected = vec!["ec{ad", "ebd"];
	parse.sort();
	expected.sort();

	assert_eq!(parse, expected);
}

#[test]
fn wojhr1q4t76uksviuxys() {
	let mut parse = crate::parse("e,c{a,b}d");
	let mut expected = vec!["e,cad", "e,cbd"];
	parse.sort();
	expected.sort();

	assert_eq!(parse, expected);
}

#[test]
fn erfjzs0omwxl1zxqf4qmis4() {
	let mut parse = crate::parse("e}c{a,b}d");
	let mut expected = vec!["e}cad", "e}cbd"];
	parse.sort();
	expected.sort();

	assert_eq!(parse, expected);
}

#[test]
fn mt2abejzy3tp0moje7m0cgd() {
	let mut parse = crate::parse("e{}c{a,b}d");
	let mut expected = vec!["ecad", "ecbd"];
	parse.sort();
	expected.sort();

	assert_eq!(parse, expected);
}

#[test]
fn hxxj534fifpie78o() {
	let mut parse = crate::parse("e{f,g}c{a,b}d");
	let mut expected = vec!["efcad", "egcad", "efcbd", "egcbd"];
	parse.sort();
	expected.sort();

	assert_eq!(parse, expected);
}

#[test]
fn ct4o26o4ba23e04w3e() {
	let mut parse = crate::parse("c{a{f,g},b}d");
	let mut expected = vec!["cafd", "cagd", "cbd"];
	parse.sort();
	expected.sort();

	assert_eq!(parse, expected);
}

#[test]
fn nij3zo7tr6us2p768() {
	let mut parse = crate::parse("c{{f,g},b}d");
	let mut expected = vec!["cfd", "cgd", "cbd"];
	parse.sort();
	expected.sort();

	assert_eq!(parse, expected);
}

#[test]
fn wwlezydws4tyf8s8pi() {
	let mut parse = crate::parse("c{a{f,g}h{i,j},b}d");
	let mut expected = vec!["cafhid", "cafhjd", "caghid", "caghjd", "cbd"];
	parse.sort();
	expected.sort();

	assert_eq!(parse, expected);
}
