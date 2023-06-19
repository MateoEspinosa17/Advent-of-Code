use std::fs::File;
use std::io::{ prelude::*, BufReader};

fn get_ribon_length(l:u32, w:u32, h:u32) -> u32
{
    let ribon_lenght:u32 = l * w * h;
    let min:u32;
    
    if l > w
    {
        if l > h
        {
            min = 2 * (w + h);
        }
        else
        {
            min = 2 * (l + w);
        }
    }
    else
    {
        if w > h
        {
            min = 2 * (l + h);
        }
        else
        {
            min = 2 * (l + w);
        }
    }

    return ribon_lenght + min;
}

fn main()
{
    let mut length:u32 = 0;
    let mut width:u32 = 0;
    let mut height:u32;         // We don't initialize it to avoid [warning: value assigned to `height` is never read]
    let mut total_area:u32 = 0;
    let mut ribon:u32 = 0;


    let file = File::open("test.txt").expect("Failed to open the file!");
    let input = BufReader::new(file);

    let mut number = String::with_capacity(10);

    for line in input.lines()
    {
        let mut counter:u8 = 0;

        for character in line.unwrap().chars()
        {
            if character != 'x'
            {
                number.push(character);
            }
            else
            {
                if counter == 0
                {
                    length = number.parse().unwrap();
                    counter += 1;
                }
                else if counter == 1
                {
                    width = number.parse().unwrap();
                    counter += 1;
                }

                number.clear();
            }
        }

        height = number.parse().unwrap();
        number.clear();

        let mut min_area = length*width;

        if width*height < min_area
        {
            min_area = width*height;
        }

        if height*length < min_area
        {
            min_area = height*length;
        }
        
        total_area += 2*length*width + 2*width*height + 2*height*length + min_area;
        ribon += get_ribon_length(length, width, height);
    }

    println!("Total Area is: {total_area} square feet.");
    println!("Ribon's Lenght is: {ribon} feet.");
}