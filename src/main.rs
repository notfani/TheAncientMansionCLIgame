mod game;
mod dice;

use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use rand::Rng;

#[derive(Clone)]
struct Weapon {
    name: &'static str,
    dice_count: u32,
    damage_bonus: i32,
}

struct Enemy {
    _name: &'static str,
    health: i32,
    damage: i32,
    _damage_dice: Option<u32>, // Some для врагов с кубами, None для фиксированного урона
}

fn main() {
    game::start();

    let mut player_weapon: Option<Weapon> = None;

    // После выхода из первой комнаты
    hallway_choice(&mut player_weapon);
}

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

fn hallway_choice(player_weapon: &mut Option<Weapon>) {
    clear_screen();
    println!("═══════════════════════════════════════════════════");
    println!("            КОРИДОР ДРЕВНЕГО ОСОБНЯКА");
    println!("═══════════════════════════════════════════════════\n");

    println!("Перед вами коридор. Прямо у стены стоит старый сундук.");
    println!("Налево уходит темный проход, от которого веет холодом.");
    println!("Направо ведет лестница вверх.\n");

    loop {
        println!("\nЧто вы будете делать?");
        println!("1. Осмотреть сундук");
        println!("2. Пойти налево");
        println!("3. Пойти направо");

        let choice = get_user_input();

        match choice.trim() {
            "1" => {
                chest_scene(player_weapon);
                break;
            },
            "2" => {
                catacombs_scene(player_weapon);
                break;
            },
            "3" => {
                mansion_scene(player_weapon);
                break;
            },
            _ => println!("Неверный выбор. Попробуйте снова."),
        }
    }
}

fn chest_scene(player_weapon: &mut Option<Weapon>) {
    clear_screen();
    println!("═══════════════════════════════════════════════════");
    println!("                   ДРЕВНИЙ СУНДУК");
    println!("═══════════════════════════════════════════════════\n");

    println!("Вы открываете сундук и находите коллекцию старого оружия!\n");
    thread::sleep(Duration::from_secs(1));

    let weapons = vec![
        Weapon { name: "Меч", dice_count: 2, damage_bonus: 2 },
        Weapon { name: "Вилы", dice_count: 3, damage_bonus: 0 },
        Weapon { name: "Топор", dice_count: 2, damage_bonus: 4 },
        Weapon { name: "Дубина", dice_count: 4, damage_bonus: -2 },
        Weapon { name: "Лук", dice_count: 1, damage_bonus: 3 },
    ];

    println!("Доступное оружие:");
    for (i, weapon) in weapons.iter().enumerate() {
        println!("{}. {} ({}d6 + {})", i + 1, weapon.name, weapon.dice_count, weapon.damage_bonus);
    }

    loop {
        println!("\nВыберите оружие (1-5):");
        let choice = get_user_input();

        if let Ok(idx) = choice.trim().parse::<usize>() {
            if idx >= 1 && idx <= weapons.len() {
                *player_weapon = Some(weapons[idx - 1].clone());
                println!("\nВы взяли {}!", weapons[idx - 1].name);
                thread::sleep(Duration::from_secs(1));
                break;
            }
        }
        println!("Неверный выбор.");
    }

    after_chest_choice(player_weapon);
}

fn after_chest_choice(player_weapon: &mut Option<Weapon>) {
    clear_screen();
    println!("\nКуда вы хотите пойти?");
    println!("1. Налево (в темный проход)");
    println!("2. Направо (вверх по лестнице)");

    loop {
        let choice = get_user_input();
        match choice.trim() {
            "1" => {
                catacombs_scene(player_weapon);
                break;
            },
            "2" => {
                mansion_scene(player_weapon);
                break;
            },
            _ => println!("Неверный выбор."),
        }
    }
}

fn catacombs_scene(player_weapon: &mut Option<Weapon>) {
    clear_screen();
    println!("═══════════════════════════════════════════════════");
    println!("                    КАТАКОМБЫ");
    println!("═══════════════════════════════════════════════════\n");

    println!("Вы спускаетесь в холодные катакомбы...");
    thread::sleep(Duration::from_secs(2));

    // Лабиринт
    labyrinth_navigation();

    // Бой с крысой
    fight_giant_rat(player_weapon);
}

fn labyrinth_navigation() {
    println!("\nВы оказались в лабиринте катакомб!");
    println!("Попробуйте найти выход...\n");
    thread::sleep(Duration::from_secs(1));

    let mut _position = 0;
    let correct_path = vec!["право", "лево", "прямо"];

    for step in &correct_path {
        println!("\nПеред вами развилка. Куда пойдете? (лево/право/прямо)");
        loop {
            let choice = get_user_input().to_lowercase();
            if choice.contains("лево") || choice.contains("право") || choice.contains("прямо") {
                if choice.contains(step) {
                    println!("Вы идете дальше...");
                    _position += 1;
                    thread::sleep(Duration::from_secs(1));
                    break;
                } else {
                    println!("Тупик! Возвращаетесь назад.");
                    thread::sleep(Duration::from_secs(1));
                }
            } else {
                println!("Неверная команда.");
            }
        }
    }

    println!("\nВы нашли выход из лабиринта!");
    thread::sleep(Duration::from_secs(2));
}

fn fight_giant_rat(player_weapon: &mut Option<Weapon>) {
    clear_screen();
    println!("═══════════════════════════════════════════════════");
    println!("              БОЙ С ГИГАНТСКОЙ КРЫСОЙ");
    println!("═══════════════════════════════════════════════════\n");

    let mut rat = Enemy {
        _name: "Гигантская крыса",
        health: 40,
        damage: 8,
        _damage_dice: None,
    };

    let mut player_health = 50;
    let mut rng = rand::thread_rng();

    println!("Из тени выползает ОГРОМНАЯ крыса!");
    println!("Здоровье крысы: {}", rat.health);
    thread::sleep(Duration::from_secs(2));

    while player_health > 0 && rat.health > 0 {
        println!("\n--- Ваш ход ---");
        println!("Ваше здоровье: {} | Здоровье крысы: {}", player_health, rat.health);
        println!("Нажмите Enter для атаки...");
        get_user_input();

        let damage = if let Some(weapon) = player_weapon {
            let mut total = 0;
            print!("Бросок кубов: ");
            for i in 0..weapon.dice_count {
                let roll = rng.gen_range(1..=6);
                print!("{}", roll);
                if i < weapon.dice_count - 1 {
                    print!(" + ");
                }
                total += roll;
            }
            total += weapon.damage_bonus;
            println!(" + {} = {}", weapon.damage_bonus, total);
            total
        } else {
            let roll = rng.gen_range(1..=6);
            println!("Бросок: {}", roll);
            roll
        };

        rat.health -= damage;
        println!("Вы нанесли {} урона!", damage);
        thread::sleep(Duration::from_secs(1));

        if rat.health <= 0 {
            println!("\n🎉 Вы победили гигантскую крысу!");
            break;
        }

        println!("\n--- Ход крысы ---");
        thread::sleep(Duration::from_secs(1));
        player_health -= rat.damage;
        println!("Крыса нанесла вам {} урона!", rat.damage);
        thread::sleep(Duration::from_secs(1));
    }

    if player_health > 0 {
        escape_ending();
    } else {
        death_ending();
    }
}

fn mansion_scene(player_weapon: &mut Option<Weapon>) {
    clear_screen();
    println!("═══════════════════════════════════════════════════");
    println!("                ГЛАВНЫЙ ЗАЛ ОСОБНЯКА");
    println!("═══════════════════════════════════════════════════\n");

    println!("Вы поднимаетесь в роскошный зал особняка...");
    thread::sleep(Duration::from_secs(2));

    println!("\nВ зале вас встречают двое скелетов-горничных.");
    println!("В глубине зала, на троне, сидит элегантный вампир.");
    thread::sleep(Duration::from_secs(2));

    vampire_dialogue(player_weapon);
}

fn vampire_dialogue(player_weapon: &mut Option<Weapon>) {
    clear_screen();
    println!("═══════════════════════════════════════════════════");
    println!("              ГРАФ ВЛАДИМИР КРОВАВЫЙ");
    println!("═══════════════════════════════════════════════════\n");

    println!("Вампир: 'Ах, гость... Как интересно. Скажи мне...'");
    thread::sleep(Duration::from_secs(2));

    let mut score = 0;

    // Вопрос 1
    println!("\nВампир: 'Зачем ты пришел в мой дом?'");
    println!("1. Я заблудился и хочу только уйти");
    println!("2. Я пришел уничтожить тебя, тварь!");
    println!("3. Я ищу сокровища");

    match get_user_input().trim() {
        "1" => { score += 2; println!("Вампир усмехается: 'Честность... редкость.'"); },
        "2" => { score -= 1; println!("Вампир хмурится: 'Дерзость...'"); },
        "3" => { score += 1; println!("Вампир: 'Хм, по крайней мере честно.'"); },
        _ => { score -= 1; },
    }
    thread::sleep(Duration::from_secs(2));

    // Вопрос 2
    println!("\nВампир: 'Что ты думаешь о вечной жизни?'");
    println!("1. Это проклятие");
    println!("2. Это великий дар");
    println!("3. У всего должен быть конец");

    match get_user_input().trim() {
        "1" => { score += 2; println!("Вампир кивает: 'Мудрые слова...'"); },
        "2" => { score += 0; println!("Вампир: 'Наивность.'"); },
        "3" => { score += 1; println!("Вампир: 'Философски.'"); },
        _ => { score -= 1; },
    }
    thread::sleep(Duration::from_secs(2));

    // Вопрос 3
    println!("\nВампир: 'Последний вопрос: присоединишься ли ты ко мне?'");
    println!("1. Никогда!");
    println!("2. Я хочу только уйти с миром");
    println!("3. Может быть... расскажите больше");

    match get_user_input().trim() {
        "1" => { score -= 2; println!("Вампир встает: 'Жаль...'"); },
        "2" => { score += 2; println!("Вампир: 'Хорошо. Можешь идти.'"); },
        "3" => { score += 1; println!("Вампир смеется: 'Дипломатично!'"); },
        _ => { score -= 1; },
    }
    thread::sleep(Duration::from_secs(2));

    // Определяем исход
    if score >= 4 {
        peaceful_ending();
    } else if score >= 1 {
        fight_skeletons(player_weapon);
    } else {
        fight_vampire(player_weapon);
    }
}

fn fight_skeletons(player_weapon: &mut Option<Weapon>) {
    clear_screen();
    println!("═══════════════════════════════════════════════════");
    println!("            БОЙ СО СКЕЛЕТАМИ-ГОРНИЧНЫМИ");
    println!("═══════════════════════════════════════════════════\n");

    println!("Вампир: 'Мои слуги, проводите гостя...'");
    thread::sleep(Duration::from_secs(2));

    let mut skeleton_health = 50;
    let skeleton_damage = 6;
    let mut player_health = 50;
    let mut rng = rand::thread_rng();

    while player_health > 0 && skeleton_health > 0 {
        println!("\n--- Ваш ход ---");
        println!("Ваше здоровье: {} | Здоровье скелетов: {}", player_health, skeleton_health);
        println!("Нажмите Enter для атаки...");
        get_user_input();

        let damage = if let Some(weapon) = player_weapon {
            let mut total = 0;
            for _ in 0..weapon.dice_count {
                total += rng.gen_range(1..=6);
            }
            total += weapon.damage_bonus;
            total
        } else {
            rng.gen_range(1..=6)
        };

        skeleton_health -= damage;
        println!("Вы нанесли {} урона!", damage);
        thread::sleep(Duration::from_secs(1));

        if skeleton_health <= 0 {
            println!("\n🎉 Вы победили скелетов!");
            escape_ending();
            return;
        }

        println!("\n--- Ход скелетов ---");
        player_health -= skeleton_damage;
        println!("Скелеты нанесли {} урона!", skeleton_damage);
        thread::sleep(Duration::from_secs(1));
    }

    if player_health <= 0 {
        death_ending();
    }
}

fn fight_vampire(player_weapon: &mut Option<Weapon>) {
    clear_screen();
    println!("═══════════════════════════════════════════════════");
    println!("           БОЙ С ГРАФОМ КРОВАВЫМ");
    println!("═══════════════════════════════════════════════════\n");

    println!("Вампир: 'Тогда умри, смертный!'");
    thread::sleep(Duration::from_secs(2));

    let mut vampire_health = 80;
    let mut player_health = 50;
    let mut rng = rand::thread_rng();

    while player_health > 0 && vampire_health > 0 {
        println!("\n--- Ваш ход ---");
        println!("Ваше здоровье: {} | Здоровье вампира: {}", player_health, vampire_health);
        println!("Нажмите Enter для атаки...");
        get_user_input();

        let damage = if let Some(weapon) = player_weapon {
            let mut total = 0;
            for _ in 0..weapon.dice_count {
                total += rng.gen_range(1..=6);
            }
            total += weapon.damage_bonus;
            total
        } else {
            rng.gen_range(1..=6)
        };

        vampire_health -= damage;
        println!("Вы нанесли {} урона!", damage);
        thread::sleep(Duration::from_secs(1));

        if vampire_health <= 0 {
            println!("\n🎉 НЕВЕРОЯТНО! Вы победили графа Кровавого!");
            hero_ending();
            return;
        }

        println!("\n--- Ход вампира ---");
        let vampire_damage: i32 = (0..5).map(|_| rng.gen_range(1..=6)).sum();
        player_health -= vampire_damage;
        println!("Вампир нанес {} урона! (5d6)", vampire_damage);
        thread::sleep(Duration::from_secs(1));
    }

    if player_health <= 0 {
        death_ending();
    }
}

fn peaceful_ending() {
    clear_screen();
    println!("═══════════════════════════════════════════════════");
    println!("                 МИРНЫЙ ИСХОД");
    println!("═══════════════════════════════════════════════════\n");

    println!("Вампир встает и указывает на дверь:");
    println!("'Ты интересный собеседник. Можешь идти.'");
    println!("\nВы покидаете особняк живым и невредимым.");
    println!("\n🎭 КОНЦОВКА: Дипломат");
    println!("\nНажмите Enter для выхода...");
    get_user_input();
}

fn escape_ending() {
    clear_screen();
    println!("═══════════════════════════════════════════════════");
    println!("                   ПОБЕГ");
    println!("═══════════════════════════════════════════════════\n");

    println!("Вы находите выход и сбегаете из проклятого особняка!");
    println!("\n🏃 КОНЦОВКА: Выживший");
    println!("\nНажмите Enter для выхода...");
    get_user_input();
}

fn hero_ending() {
    clear_screen();
    println!("═══════════════════════════════════════════════════");
    println!("                 ГЕРОИЧЕСКИЙ ФИНАЛ");
    println!("═══════════════════════════════════════════════════\n");

    println!("Граф Кровавый повержен! Особняк начинает рушиться...");
    println!("Вы едва успеваете выбежать наружу.");
    println!("\n⚔️ КОНЦОВКА: Герой");
    println!("\nНажмите Enter для выхода...");
    get_user_input();
}

fn death_ending() {
    clear_screen();
    println!("═══════════════════════════════════════════════════");
    println!("                   СМЕРТЬ");
    println!("═══════════════════════════════════════════════════\n");

    println!("Ваше зрение меркнет...");
    println!("Древний особняк забирает еще одну душу.");
    println!("\n💀 GAME OVER");
    println!("\nНажмите Enter для выхода...");
    get_user_input();
}
