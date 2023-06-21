use std::fs;


#[derive(Eq, PartialEq)]
struct Position
{
    x:i16,
    y:i16,
}


fn main()
{
    let input = fs::read_to_string("test.txt").unwrap();
    let mut position_vector:Vec<Position> = Vec::new();
    let mut position = Position{x:0, y:0};
    let mut santa_i:i16 = 0;
    let mut santa_j:i16 = 0;
    let mut robot_i:i16 = 0;
    let mut robot_j:i16 = 0;

    // Store initial value.
    position_vector.push(position);

    for (index, direction) in input.chars().enumerate()
    {   
        let mut current_x:i16 = if index % 2 == 0
        {
            santa_i
        }
        else
        {
            robot_i
        };

        let mut current_y:i16 = if index % 2 == 0
        {
            santa_j
        }
        else
        {
            robot_j
        };

        match direction
        {
            '^' => current_y+=1,
            'v' => current_y-=1,
            '<' => current_x-=1,
            '>' => current_x+=1,
            _ => continue,
        }

        if !is_in_vector(position_vector.as_mut_slice(), current_x, current_y)
        {
            position = Position{x:current_x, y:current_y};
            position_vector.push(position);
        }

        if index % 2 == 0
        {
            santa_i = current_x;
            santa_j = current_y;
        }
        else
        {
            robot_i = current_x;
            robot_j = current_y;
        }
    }

    let len = position_vector.len();
    println!("Houses which received at least 1 gift: {len}")
}


fn is_in_vector(vec:&mut [Position], pos_x:i16, pos_y:i16) -> bool 
{
    let new_position = Position{x:pos_x, y:pos_y};

    for position in vec
    {
        if new_position == *position
        {
            return true;
        }
    }

    return false;
}