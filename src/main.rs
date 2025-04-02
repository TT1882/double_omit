use input_handle::{get_u32_input, get_string_input};

fn main() {
    let mut omit_operators: Vec<String> = Vec::new();
    let mut page_numbers: Vec<u32> = Vec::new();
    let mut operator_range: Vec<u32>;
    let mut print_range_one: Vec<u32> = Vec::new();
    let mut print_range_two: Vec<u32> = Vec::new();

    // Get user inputs
    let document_length = get_u32_input("How many pages does the document contain? ");
    let mut omit_pages_raw = get_string_input("\nWhich pages should be omitted? ");

    // Create array of page numbers
    for i in 0..document_length {
        page_numbers.push(i+1);
    }

    // Remove whitespace from omit_pages and splits on each comma
    let omit_pages_clean = remove_whitespace(&mut omit_pages_raw).split(",");
    
    // Loop for each split in variable
    for i in omit_pages_clean {
        omit_operators.push(i.to_string());
    }

    for i in 0..omit_operators.len() {
        // If operator is a range
        if omit_operators[i].contains("-") {
            operator_range = Vec::new();
            // Split the operator range with -
            let operator_range_split = omit_operators[i].split("-");

            // Loop for each split in variable
            for i in operator_range_split {
                operator_range.push(i.parse().unwrap());
            }
            // Remove range from all numbers
            for i in operator_range[0]..operator_range[1]+1 {
                page_numbers.retain(|&x| x != i);
            }
        }
        else{
            // If the value is a number
            match omit_operators[i].parse::<u32>() {
                // Remove single number
                Ok(_ok) => page_numbers.retain(|&x| x != omit_operators[i].parse().unwrap()),
                Err(_e) => return,
            }
        }
    }

    // Sorts pages to print into two print ranges
    for i in 0..document_length/2 {
        if i%2 == 0 {
            for _i in 0..2 {
                if page_numbers.len() != 0 {
                    print_range_one.push(page_numbers[0]);
                    page_numbers.remove(0);
                }
            }
        }
        else {
            for _i in 0..2 {
                if page_numbers.len() != 0 {
                    print_range_two.push(page_numbers[0]);
                    page_numbers.remove(0);
                }
            }
        }
    }

    // Print print ranges
    println!("\nPrint Range 1: {}", Vec::from_iter(print_range_one.iter().map(|i| i.to_string())).join(", "));
    println!("\nPrint Range 2: {}", Vec::from_iter(print_range_two.iter().map(|i| i.to_string())).join(", "));

}

fn remove_whitespace(s: &mut String) -> &String{
    s.retain(|c| !c.is_whitespace());
    return s;
}
