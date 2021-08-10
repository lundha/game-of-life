extern crate piston_window;
use piston_window::*;

fn main() {
    let mut matrix = generate_matrix(5,5);
    let mut universe = initiate_universe(matrix);
    println!("Hello, world!");
    simulate_universe(universe);
    graphics();
}

// Create graphics to illustrate game of life 
fn graphics() {

    let mut window: PistonWindow = WindowSettings::new("Hello Piston", [640,480]).exit_on_esc(true).build().unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [0.0, 100.0, 100.0, 100.0],
                      context.transform,
                      graphics);
        });
    }
}






fn generate_matrix(n: usize, m: usize) -> Vec<Vec<i64>> {

    let mut x = vec![vec![0i64; n]; m];
    return x
}

fn set_value_in_matrix(mut matrix: Vec<Vec<i64>>, value: i64, row: usize, col: usize)  {

    matrix[row][col] = value;
    println!("{:?}", matrix);       
}

fn initiate_universe(mut matrix: Vec<Vec<i64>>) -> Vec<Vec<i64>>  {

    /*
    for (i,row) in x.iter_mut().enumerate() {
        for (j, col) in row.iter_mut().enumerate() {
    
        }
    }
    */
    matrix[1][2] = 1;
    matrix[2][2] = 1;
    matrix[3][2] = 1;
  
    return matrix
}


fn print_matrix(mut matrix: Vec<Vec<i64>>) {

    for (i,row) in matrix.iter_mut().enumerate() {
        println!("{:?}", row);       
    }

}



fn iterate_matrix(matrix: Vec<Vec<i64>>, rows: i64, cols: i64) -> Vec<Vec<i64>> {
    
    let mut num_alive: i64;
    let mut new_matrix = generate_matrix(5,5);
    let mut row_index: i64;
    let mut col_index: i64;
    let mut cell_value: i64;
    row_index = 0;
    col_index = 0;


    for (i,row) in matrix.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {

            num_alive = 0;

            // Cast i, j to int32
            if row_index-1 >= 0 {
                num_alive += matrix[i-1][j];
            }
            if col_index-1 >= 0 {
                num_alive += matrix[i][j-1];
            }
            if row_index-1 >= 0 && col_index-1 >= 0 {
                num_alive += matrix[i-1][j-1];

            } 
            if row_index+1 < rows {
                num_alive += matrix[i+1][j];

            }
            if col_index+1 < cols {
                num_alive += matrix[i][j+1];

            } 
            if row_index+1 < rows && col_index+1 < cols {
                num_alive += matrix[i+1][j+1];

            }
            if row_index-1 >= 0 && col_index+1 < cols {
                num_alive += matrix[i-1][j+1];

            }
            if row_index+1 < rows && col_index-1 >= 0 {
                num_alive += matrix[i+1][j-1];

            }
            // Set value in new matrix 
            cell_value = matrix[i][j];
            new_matrix[i][j] = return_live_or_dead(num_alive, cell_value);
            col_index += 1;

        }
        col_index = 0;
        row_index += 1;

    }   
    return new_matrix
}


fn return_live_or_dead(num_alive: i64, cell_value: i64) -> i64 {

    if cell_value == 1 {
        if num_alive < 2 {
            return 0
        } 
        if num_alive > 3 {
            return 0
        }
        else {
            return 1
        }
    }
    else {
        if num_alive == 3 {
            return 1
        }
        else {
            return 0
        }

    }

}

fn simulate_universe(mut universe: Vec<Vec<i64>>) {

    print_matrix(universe.clone());
    for _ in 1..2 {
            let mut new_universe = iterate_matrix(universe.clone(), 5, 5);
            print_matrix(new_universe.clone());
            println!("{}", '\r');
            universe = new_universe;

    }

   
}
