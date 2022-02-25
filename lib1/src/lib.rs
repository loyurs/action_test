
#![allow(unused)]
pub mod logy_pop_cards {
    use rand::Rng;

    //创建一个卡池结构体
    struct Tpu(i8, i8, i8);
    pub struct Cards_pool {
        pub cards: Vec<&'static str>,
        name: String,
    }
    fn generate_card_pool(
        five_stars_percent: i8,
        four_stars_percent: i8,
        three_stars_percent: i8,
    ) -> Vec<&'static str> {
        let a = "***";
        let b = "****";
        let c = "*****";
        let mut init_vec = vec![];
        let mut pt = 0;
        while pt < five_stars_percent {
            pt = pt + 1;
            init_vec.push(a)
        }
        let mut pt = 0; //var shadow
        while pt < four_stars_percent {
            pt = pt + 1;
            init_vec.push(b)
        }
        let mut pt = 0;
        while pt < three_stars_percent {
            pt = pt + 1;
            init_vec.push(c);
        }
        // println!("{:?}",init_vec);
        init_vec
    }

    impl Cards_pool {
        //使用泛型T， 替换进去的T需要是个结构体
        //从概率构造一个卡池
        pub fn new(name: String, tup: (i8, i8, i8)) -> Self {
            let card_pool = generate_card_pool(tup.0, tup.1, tup.2);
            Cards_pool {
                cards: card_pool,
                name: String::from("cardpool1"),
            }
        }
    }
    pub fn get_cards(card_pool: Vec<&str>, times: i32) -> Vec<&str> {
        let mut rng = rand::thread_rng();
        let length = card_pool.len();
        let mut emp_vec = vec!["a"];
        let mut pn = 0;
        while pn < times {
            emp_vec.push(card_pool[rng.gen_range(0..length)]);
            pn = pn + 1
        }
        emp_vec
    }

    pub fn when_get_file(
        card_pool: Cards_pool,
        the_times: i32,
    ) -> (Vec<&'static str>, (i32, i32, i32)) {
        let p = get_cards(card_pool.cards, the_times); //p为抽卡抽到的所有卡
                                                       //判断是否抽中了5星
        let mut is5stars = 0;
        let mut is4starts = 0;
        let mut is3starts = 0;
        let mut range_var = 0;
        //记录一共抽卡的总函数
        for ipc in p.iter() {
            if ipc.to_string() == "*****" {
                println!("第{}抽出了五星", range_var);
                is5stars = is5stars + 1;
            } else if ipc.to_string() == "****" {
                is4starts = is4starts + 1;
                range_var = range_var + 1;
            } else {
                is3starts = is3starts + 1;
                range_var = range_var + 1;
            }
        }
        (p, (is5stars, is4starts, is3starts))
    }
}
