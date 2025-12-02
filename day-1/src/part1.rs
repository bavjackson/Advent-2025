pub fn part1(input: &str) -> String {
    let mut counter = 0;
    let mut current = 50;

    for line in input.lines() {
        let (dir, str_num) = line.split_at(1);

        let convert_result = str_num.parse::<i32>();

        let num = match convert_result {
            Ok(int) => int,
            Err(error) => panic!("Problem converting to int {:?}", error),
        };

        if dir == "L" {
            current -= num;
            if current < 0 {
                current = (100 + current) % 100
            }
        } else {
            current += num;

            if current >= 100 {
                current %= 100
            }
        }

        if current == 0 {
            counter += 1;
        }
    }

    counter.to_string()
}
