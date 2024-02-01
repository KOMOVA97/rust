// 平民（commoner）们见多识广，收到什么礼物都能应对。
// 所有礼物都显式地使用 `match` 来处理。
fn give_commoner(gift: Option<&str>) {
    // 指出每种情况下的做法。
    match gift {
        Some("snake") => println!("Yuck! I'm throwing that snake in a fire."),
        Some(gift)   => println!("{}? How nice.", gift),
        None          => println!("No gift? Oh well."),
    }
}

fn give_princess(gift: Option<&str>) {
    // `unwrap` 在接收到 `None` 时将返回 `panic`。
    let inside = gift.unwrap();
    if inside == "snake" { panic!("AAAaaaaa!!!!"); }

    println!("I love {}s!!!!!", inside);
}

#[test]
fn test_give_commer() {
    let food  = Some("chicken");
    let snake = Some("snake");
    let void  = None;

    give_commoner(food);
    give_commoner(snake);
    give_commoner(void);

    let bird = Some("robin");


    println!("ceshi{:?}", food);
    give_princess(bird);
    //give_princess(nothing);
}