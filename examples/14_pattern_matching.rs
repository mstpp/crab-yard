struct Player {
    name: String,
    health: i32,
    shield: Option<i32>,    // Optional shield points
    weapon: Option<String>, // Optional equipped weapon
}

fn player_main() {
    let mut player = Player {
        name: String::from("Hero"),
        health: 100,
        shield: Some(50),
        weapon: Some(String::from("sword")),
    };

    take_damage(&mut player, 30);
    assert_eq!(player.shield, Some(20));
    assert_eq!(player.health, 100);

    take_damage(&mut player, 20);
    assert_eq!(player.shield, None);
    assert_eq!(player.health, 100);

    take_damage(&mut player, 50);
    assert_eq!(player.shield, None);
    assert_eq!(player.health, 50);

    upgrade_weapon(&mut player);
    println!("{}: Weapon: {:?}", player.name, player.weapon);
    assert_eq!(player.weapon, Some("sword +1".to_string()));
}

fn take_damage(player: &mut Player, damage: i32) {
    if let Some(ref mut shield) = player.shield {
        if damage >= *shield {
            if let Some(whole_shield) = player.shield.take() {
                player.health -= damage - whole_shield;
            }
        } else {
            *shield -= damage;
        }
    } else {
        // missing case where damage > health (game over)
        player.health -= damage
    }
    // match player.shield.take() {
    //     Some(shield) if damage >= shield => {
    //         player.shield = None;
    //         player.health -= damage - shield
    //     }
    //     Some(shield) => player.shield = Some(shield - damage),
    //     None => player.health -= damage,
    // }
}

fn upgrade_weapon(player: &mut Player) {
    if let Some(weapon) = player.weapon.as_mut() {
        weapon.push_str(" +1");
    }
}

fn main() {
    // ref mut vs .as_mut()
    //
    // Incrementing an Optional Counter
    // use ref mut in pattern matching
    let mut counter: Option<i32> = Some(10);
    if let Some(ref mut c) = counter {
        *c += 1;
    }
    assert_eq!(counter, Some(11));
    // same with .as_mut()
    if let Some(c) = counter.as_mut() {
        *c += 1;
    }
    assert_eq!(counter, Some(12));
    //
    //
    // Modifying Strings In-Place
    //
    let mut opt_name: Option<String> = Some(String::from("alice"));
    if let Some(name) = opt_name.as_mut()
    /*mutable borrow*/
    {
        // name is now &mut String
        // .get_mut() returns a Option<&mut str>
        if let Some(slice) = name.get_mut(0..1) {
            slice.make_ascii_uppercase(); // it's allowed to have in-place change
        }
    }
    assert_eq!(opt_name, Some("Alice".to_string()));
    if let Some(name) = &opt_name
    /*here is immutable borrow*/
    {
        // name is &String
        assert_eq!(name.len(), 5);
    }
    //
    //
    // Nested Options
    //
    let mut nested: Option<Option<i32>> = Some(Some(5));
    if let Some(Some(ref mut v)) = nested {
        // if let Some(Some(v)) = nested.as_mut() { // also works
        *v *= 2;
    }
    assert_eq!(nested, Some(Some(10)));
    // when nested in None
    let mut nested2: Option<Option<i32>> = Some(None);
    if let Some(ref mut v) = nested2 {
        *v = Some(33);
    }
    assert_eq!(nested2, Some(Some(33)));
    //
    //
    // Struct Fields with Options
    //
    player_main();
}
