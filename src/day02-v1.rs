fn main() {
    println!("\n");

    let mut number_list = vec![
        10, 56, 34, 75, 82, 21, 45, 62, 86, 95, 74, 91, 203, 58, 401, 301,
    ];

    //let sort_style = Sorting::Ascending;
    let sort_style = Sorting::Descending;

    let len = number_list.len() - 1;

    'outer: loop {
        let mut swapped = false;

        for index in 0..len {
            match sort_style {
                Sorting::Ascending => {
                    if number_list[index] > number_list[index + 1] {
                        number_list.swap(index, index + 1);
                        swapped = true;
                    }
                }

                Sorting::Descending => {
                    if number_list[index] < number_list[index + 1] {
                        number_list.swap(index, index + 1);
                        swapped = true;
                    }
                }
            }
        }

        if !swapped {
            break 'outer;
        }
    }

    for item in &number_list {
        println!(" -> {}", item);
    }

    println!("\n The End ...\n")
}

enum Sorting {
    Ascending,
    Descending,
}
