use rand::Rng;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

pub fn get_rand_code(len: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..len).map(|_| {
        let idx = rng.gen::<usize>() % CHARSET.len();
        char::from(*CHARSET.get(idx).unwrap())
    }).collect()
}

/**
 * Rand 与 UUID 性能简测
 */
pub fn main() {
    let begin = std::time::SystemTime::now();
    let rand = rand::thread_rng().gen::<u32>();
    for _i in 0..1000 {
        log::info!("rand: {:?}", rand::thread_rng().gen::<u32>())
    }
    let duration = begin.elapsed().unwrap().as_millis();
    println!("duration: {:?}, rand: {:?}", duration, rand); // duration: 0, rand: 1752657391

    // use uuid::Uuid;

    // let begin = std::time::SystemTime::now();
    // let uuid = Uuid::new_v4();
    // for i in 0..1000 {
    //     let uuid = Uuid::new_v4();
    // }
    // let duration = begin.elapsed().unwrap().as_millis();
    // println!("duration: {:?}, uuid: {:?}", duration, uuid); // duration: 2, uuid: 598b501a-7fa9-43b4-9e44-aa1d87c6e882
}

#[cfg(test)]
mod random_test {
    use crate::module::utils;

    #[test]
    pub fn test_random() {
        use rand::Rng;
        let ranint = rand::thread_rng().gen::<u32>() % 1000000;
        println!("ran_int: {}", ranint);
    }

    // https://llever.com/rust-cookbook-zh/algorithms/randomness.zh.html

    #[test]
    pub fn test_rang_pwd() {
        let password = utils::random::get_rand_code(6);
        println!("test_rang_pwd: {:?}", password);
    }

    // die.sample is unsafe, need nightly.
    /*#[test]
    pub fn test_fast_range_random() {
        let mut range_random = rand::thread_rng();
        let mut die = Uniform::from(1..10000000);
        loop {
            let throw = die.sample(&mut range_random);
            println!("Range_random: {}", throw);
            if throw == 6 {
                break;
            }
        }
    }*/
}