pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut matrix = matrix_of_zeros(size);
    let mut snake = Snake::new();

    for i in 1..=size * size {
        snake.add_number_and_advance(i, &mut matrix);
    }

    return matrix;
}

fn matrix_of_zeros(size: u32) -> Vec<Vec<u32>> {
    vec![vec![0u32; size as usize]; size as usize]
}

#[derive(Copy, Clone)]
struct Snake {
    position: Position,
    facing: Direction,
}

impl Snake {
    fn new() -> Self {
        Self {
            position: Position { x: 0, y: 0 },
            facing: Direction::RIGHT,
        }
    }

    fn must_turn(&self, matrix: &Vec<Vec<u32>>) -> bool {
        let next: &Position = &self.next_pos();
        let max: i32 = matrix.len() as i32;
        if next.x < 0 || next.y < 0 {
            true
        } else if next.x >= max || next.y >= max {
            true
        } else {
            matrix[next.y as usize][next.x as usize] > 0
        }
    }

    fn next_pos(&self) -> Position {
        match self.facing {
            Direction::RIGHT => Position::new(self.position.x + 1, self.position.y),
            Direction::DOWN => Position::new(self.position.x, self.position.y + 1),
            Direction::LEFT => Position::new(self.position.x - 1, self.position.y),
            Direction::UP => Position::new(self.position.x, self.position.y - 1),
        }
    }

    fn turn_right(&mut self) {
        self.facing = match self.facing {
            Direction::RIGHT => Direction::DOWN,
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::UP,
            Direction::UP => Direction::RIGHT,
        }
    }

    fn add_number_and_advance(&mut self, number: u32, matrix: &mut Vec<Vec<u32>>) {
        matrix[self.position.y as usize][self.position.x as usize] = number;
        if self.must_turn(matrix) {
            self.turn_right();
        }
        self.position = self.next_pos();
    }
}
#[derive(Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
#[derive(Copy, Clone)]
enum Direction {
    RIGHT,
    DOWN,
    LEFT,
    UP,
}
