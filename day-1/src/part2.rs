pub fn part2(input: &str) -> String {
    let mut counter = 0;
    let mut current = 50;
    let mut was_zero: bool;

    for line in input.lines() {
        let (dir, str_num) = line.split_at(1);

        let convert_result = str_num.parse::<i32>();

        let mut num = match convert_result {
            Ok(int) => int,
            Err(error) => panic!("Problem converting to int {:?}", error),
        };

        counter += num / 100;
        num %= 100;

        if dir == "L" {
            was_zero = current == 0;

            current -= num;
            if current < 0 {
                if !was_zero {
                    counter += 1 + ((100 + current) / 100)
                }

                current = (100 + current) % 100
            }
            if current == 0 {
                counter += 1
            }
        } else {
            current += num;
            counter += current / 100;

            if current >= 100 {
                current %= 100
            }
        }
    }

    counter.to_string()
}
