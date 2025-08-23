use conway::{CellState, World};

fn simulate_and_check(input: &[u8], expected: &[u8], width: usize) {
    assert_eq!(input.len(), expected.len());

    let height = input.len() / width;
    let mut input = World::from(&input, width, height);

    input.update(); // Simulate one step

    let expected = World::from(&expected, width, height);

    assert_eq!(input, expected);
}

macro_rules! test_step {
    ($name:ident, $input:expr, $expected:expr, $width:expr) => {
        #[test]
        fn $name() {
           simulate_and_check(&$input, &$expected, $width);
        }
    };
}

test_step!(empty_world, [0, 0, 0], [0, 0, 0], 3);

test_step!(rule_1, // Any live cell with fewer than two live neighbours dies
    // Given
    [0, 0, 0, 0, 
     0, 0, 1, 0,
     0, 0, 0, 0, 
     0, 0, 0, 0], 
    // Expected
    [0, 0, 0, 0, 
     0, 0, 0, 0,
     0, 0, 0, 0, 
     0, 0, 0, 0], 4);

test_step!(rule_1_2_cells, 
    // Given
    [0, 0, 0, 0, 
     0, 0, 1, 1,
     0, 0, 0, 0, 
     0, 0, 0, 0], 
    // Expected
    [0, 0, 0, 0, 
     0, 0, 0, 0,
     0, 0, 0, 0, 
     0, 0, 0, 0], 4);

test_step!(rule_2, // Any live cell with two or three live neighbours lives on 
    // Given
    [0, 0, 0, 1, 
     0, 0, 1, 0,
     0, 1, 0, 0, 
     0, 0, 0, 0], 
    // Expected
    [0, 0, 0, 0, 
     0, 0, 1, 0,
     0, 0, 0, 0, 
     0, 0, 0, 0], 4);

test_step!(rule_2_stable,  
    // Given
    [0, 0, 0, 0, 
     0, 1, 1, 0,
     0, 1, 1, 0, 
     0, 0, 0, 0], 
    // Expected
    [0, 0, 0, 0, 
     0, 1, 1, 0,
     0, 1, 1, 0, 
     0, 0, 0, 0], 4);

test_step!(rule_3, // Any live cell with more than three live neighbours dies
    // Given
    [0, 0, 1, 0, 
     0, 1, 1, 1,
     0, 0, 1, 0, 
     0, 0, 0, 0], 
    // Expected
    [0, 1, 1, 1, 
     0, 1, 0, 1,
     0, 1, 1, 1, 
     0, 0, 0, 0], 4);

test_step!(rule_4, // Any dead cell with exactly three live neighbours becomes a live cell
    // Given
    [0, 0, 1, 0, 
     0, 1, 0, 1,
     0, 0, 0, 0, 
     0, 0, 0, 0], 
    // Expected
    [0, 0, 1, 0, 
     0, 0, 1, 0,
     0, 0, 0, 0, 
     0, 0, 0, 0], 4);


test_step!(shape, 
    // Given
    [0, 0, 1, 0, 0, 
     0, 1, 1, 0, 0,
     0, 0, 1, 1, 0, 
     0, 0, 0, 0, 0,
     0, 0, 0, 0, 0], 
    // Expected
    [0, 1, 1, 0, 0, 
     0, 1, 0, 0, 0,
     0, 1, 1, 1, 0, 
     0, 0, 0, 0, 0,
     0, 0, 0, 0, 0],  5);


#[test]
fn compare_cell_states() {
    for i in 0..255 {
        for j in 0..255 {
            assert_ne!(CellState::Alive, CellState::Dead(i));
            assert_eq!(CellState::Dead(i), CellState::Dead(j));
        }
    }
}

#[test]
fn compare_worlds() {
    let cells = [
        0, 0, 0
    ];

    let a = World::from(&cells, 3, 1);
    let b = World::from(&cells, 3, 1);
    
    assert_eq!(a, b);
}

