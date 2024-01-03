using System.IO;

static int GetFloor(String? l)
{
    int floor = 0;

    if (l != null)
    {
        int counter = 0;

        foreach (char c in l)
        {
            if (c == '(')
            {
                floor++;
            }
            else if(c == ')')
            {
                floor--;
            }

            counter++;
            if (floor == -1)
            {
                // We just care about the first number.
                Console.WriteLine(counter);
            }
            
        }
        Console.WriteLine(counter);
    }
    else
    {
        return -1;
    }

    return floor;
}

// Open input file.
String? line;   // we add a ? at the end of the type of the variable to make it nullable.

try
{
    // Use streamreader class to open the file.
    StreamReader sr = new("Input.txt");

    // Read the first line of the file. (In this case the file only has one line.)
    line = sr.ReadLine();

    GetFloor(line);

    // Close streamreader object.
    sr.Close();
}
catch(Exception e)
{
    Console.WriteLine("Exception: " + e.Message);
}