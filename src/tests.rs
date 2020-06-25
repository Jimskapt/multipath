#[test]
fn pvykhas42a44ldrecbb51ip() {
    assert_eq!(crate::from("{a,b}"), vec!["a", "b",])
}

#[test]
fn y0itnt01ead1() {
    assert_eq!(crate::from("c{a,b}"), vec!["ca", "cb",])
}

#[test]
fn g38rqc4ewifen() {
    assert_eq!(crate::from("c{a, b}d"), vec!["cad", "cbd",])
}

#[test]
fn gxl3yvjnls33deyhr8g14el() {
    assert_eq!(crate::from("c{a ,b}d"), vec!["cad", "cbd",])
}

#[test]
fn utqe6f5i7tlgip44mpl() {
    assert_eq!(crate::from("c{a,b}d"), vec!["cad", "cbd",])
}

#[test]
fn wzn8fsszr4js() {
    assert_eq!(crate::from("{a,b,c}"), vec!["a", "b", "c",])
}

#[test]
fn wfjp6y4g2ix35xp0zcoljry() {
    assert_eq!(crate::from("{a,}"), vec!["a", ""])
}

#[test]
fn ro0zhpirqog2() {
    assert_eq!(crate::from("c{a,}d"), vec!["cad", "cd"])
}

#[test]
fn oh748141tisb6ajkcxgy7u() {
    assert_eq!(crate::from("c{}d"), vec!["c{}d"])
}

#[test]
fn bfqrw3dw0h6wy02j() {
    assert_eq!(crate::from("c{ }d"), vec!["c{ }d"])
}

#[test]
fn kk73nbafsm() {
    assert_eq!(crate::from("c{a}d"), vec!["c{a}d"])
}

#[test]
fn f67vo8aowe3alwx5v02sj6() {
    assert_eq!(crate::from("c{,}d"), vec!["cd", "cd"])
}

#[test]
fn wsvf53r2v6vprep5ix2yffu() {
    assert_eq!(crate::from("c{a,b}d{e"), vec!["cad{e", "cbd{e",])
}

#[test]
fn b1xbpg3ariguy74h() {
    assert_eq!(crate::from("c{a,b}d}e"), vec!["cad}e", "cbd}e",])
}

#[test]
fn eohhfeifgj() {
    assert_eq!(crate::from("c{a,b}d,e"), vec!["cad,e", "cbd,e",])
}

#[test]
fn s0oxd2l7uthfylivjtv() {
    assert_eq!(crate::from("e{c{a,b}d"), vec!["ec{ad", "ebd",])
}

#[test]
fn wojhr1q4t76uksviuxys() {
    assert_eq!(crate::from("e,c{a,b}d"), vec!["e,cad", "e,cbd",])
}

#[test]
fn erfjzs0omwxl1zxqf4qmis4() {
    assert_eq!(crate::from("e}c{a,b}d"), vec!["e}cad", "e}cbd",])
}

#[test]
fn mt2abejzy3tp0moje7m0cgd() {
    assert_eq!(crate::from("e{}c{a,b}d"), vec!["e{}cad", "e{}cbd",])
}

#[test]
fn hxxj534fifpie78o() {
    assert_eq!(
        crate::from("e{f,g}c{a,b}d"),
        vec!["efcad", "egcad", "efcbd", "egcbd",]
    )
}

#[test]
fn ct4o26o4ba23e04w3e() {
    assert_eq!(crate::from("c{a{f,g},b}d"), vec!["cafd", "cagd", "cbd"])
}

#[test]
fn nij3zo7tr6us2p768() {
    assert_eq!(crate::from("c{{f,g},b}d"), vec!["cfd", "cgd", "cbd"])
}

#[test]
fn wwlezydws4tyf8s8pi() {
    assert_eq!(
        crate::from("c{a{f,g}h{i,j},b}d"),
        vec!["cafhid", "cafhjd", "caghid", "caghjd", "cbd"]
    )
}
