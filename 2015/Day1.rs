use std::fs;

fn main()
{
    let input = fs::read_to_string("test.txt").unwrap();

    let mut floor = 0;
    let mut position = 0;
    let mut first_time = false;

    for (index, character) in input.chars().enumerate()
    {
        match character 
        {
            '(' => floor += 1, 
            ')' => floor -=1,
            _ => continue,      //prevent pattern error.
        }

        if floor == -1 && first_time == false
        {
            position = index + 1;
            first_time = true;
        }
    }

    println!("Floor:{floor}, Position: {position}");
}